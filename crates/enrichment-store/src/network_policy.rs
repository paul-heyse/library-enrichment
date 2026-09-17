//! Native endpoint, address and redirect admission, shared by every HTTP consumer.
use crate::{control_jobs::encode, registry::rows, runtime::QueryRuntime};
use arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{SessionContext, col},
};
use enrichment_core::policy::{FetchPolicy, PolicyViolation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct NetworkPolicy {
    runtime: QueryRuntime,
    policy: FetchPolicy,
}
impl NetworkPolicy {
    pub fn new(runtime: &QueryRuntime, policy: FetchPolicy) -> Self {
        Self {
            runtime: runtime.clone(),
            policy,
        }
    }
    fn session(&self) -> Result<SessionContext> {
        let session = self.runtime.session();
        session.register_batch(
            "trusted_endpoints",
            RecordBatch::try_new(
                Arc::new(Schema::new(vec![Field::new(
                    "authority",
                    DataType::Utf8,
                    false,
                )])),
                vec![Arc::new(StringArray::from_iter_values(
                    self.policy.trusted_hosts.iter(),
                ))],
            )?,
        )?;
        Ok(session)
    }
    /// Inputs have URL, hop and optional captured address URL. Source fields survive unchanged.
    pub async fn decisions(&self, input: DataFrame) -> Result<DataFrame> {
        let session = self.session()?;
        crate::native_catalog::work(&session, "network_input", input.into_view())?;
        session.sql(&format!(r#"
        WITH parsed AS (SELECT *,url_parts_v2(url) AS parsed,url_parts_v2(coalesce(resolved_url,url)) AS address_parts FROM network_input),
        facts AS (SELECT p.*,t.authority IS NOT NULL AS trusted FROM parsed p LEFT JOIN trusted_endpoints t ON p.parsed.authority=t.authority)
        SELECT * EXCLUDE(parsed,trusted,address_parts), coalesce(address_parts.host,parsed.host) AS selected_host, CASE
            WHEN hop>{} THEN 'redirect_limit'
            WHEN parsed IS NULL OR address_parts IS NULL THEN 'invalid_url'
            WHEN parsed.scheme<>'https' AND NOT (parsed.scheme='http' AND trusted) THEN 'scheme'
            WHEN parsed.host IS NULL THEN 'host_missing'
            WHEN NOT trusted AND (
                parsed.host='localhost' OR ends_with(parsed.host,'.localhost') OR
                address_parts.ipv4 BETWEEN 0 AND 16777215 OR
                address_parts.ipv4 BETWEEN 167772160 AND 184549375 OR
                address_parts.ipv4 BETWEEN 1681915904 AND 1686110207 OR
                address_parts.ipv4 BETWEEN 2130706432 AND 2147483647 OR
                address_parts.ipv4 BETWEEN 2851995648 AND 2852061183 OR
                address_parts.ipv4 BETWEEN 2886729728 AND 2887778303 OR
                address_parts.ipv4 BETWEEN 3221225984 AND 3221226239 OR
                address_parts.ipv4 BETWEEN 3232235520 AND 3232301055 OR
                address_parts.ipv4 BETWEEN 3323068416 AND 3323199487 OR
                address_parts.ipv4 BETWEEN 3325256704 AND 3325256959 OR
                address_parts.ipv4 BETWEEN 3405803776 AND 3405804031 OR
                address_parts.ipv4>=3758096384 OR
                regexp_like(address_parts.ipv6,'^(0000000000000000000000000000000[01]$|ff|f[cd]|fe[89ab])')
            ) THEN 'private_address' ELSE NULL END AS refusal
        FROM facts"#, self.policy.max_redirects)).await
    }
    pub async fn check(
        &self,
        url: &url::Url,
        hop: u64,
    ) -> Result<std::result::Result<(), PolicyViolation>> {
        let input = crate::native_catalog::batch(
            &self.runtime.session(),
            "network_request",
            encode(
                input_schema(),
                &[Input {
                    url: url.as_str(),
                    hop,
                    resolved_url: None,
                }],
            )?,
        )?;
        let decision: Decision = rows(&self.runtime, self.decisions(input).await?, 1)
            .await?
            .pop()
            .ok_or_else(|| DataFusionError::Internal("missing network admission row".into()))?;
        self.decode(url, decision)
    }

    /// All captured socket choices must pass the same policy before a connector receives them.
    pub async fn check_addresses(
        &self,
        url: &url::Url,
        hop: u64,
        addresses: &[std::net::SocketAddr],
    ) -> Result<std::result::Result<(), PolicyViolation>> {
        if addresses.is_empty() || addresses.len() > 64 {
            return Err(DataFusionError::ResourcesExhausted(
                "DNS answer cardinality must be 1..64".into(),
            ));
        }
        let inputs = addresses
            .iter()
            .map(|address| Input {
                url: url.as_str(),
                hop,
                resolved_url: Some(format!("https://{address}/")),
            })
            .collect::<Vec<_>>();
        let input = crate::native_catalog::batch(
            &self.runtime.session(),
            "captured_addresses",
            encode(input_schema(), &inputs)?,
        )?;
        let refused = self
            .decisions(input)
            .await?
            .filter(col("refusal").is_not_null())?
            .limit(0, Some(1))?;
        match rows(&self.runtime, refused, 1).await?.pop() {
            Some(decision) => self.decode(url, decision),
            None => Ok(Ok(())),
        }
    }

    fn decode(
        &self,
        url: &url::Url,
        decision: Decision,
    ) -> Result<std::result::Result<(), PolicyViolation>> {
        Ok(match decision.refusal.as_deref() {
            None => Ok(()),
            Some("scheme") => Err(PolicyViolation::SchemeNotAllowed {
                url: url.to_string(),
                scheme: url.scheme().into(),
            }),
            Some("host_missing" | "invalid_url") => Err(PolicyViolation::HostMissing {
                url: url.to_string(),
            }),
            Some("private_address") => Err(PolicyViolation::PrivateAddress {
                url: url.to_string(),
                host: decision.selected_host.unwrap_or_default(),
            }),
            Some("redirect_limit") => Err(PolicyViolation::RedirectLimit {
                url: url.to_string(),
                limit: self.policy.max_redirects,
            }),
            Some(_) => return Err(DataFusionError::Internal("unknown network refusal".into())),
        })
    }
}

#[derive(Serialize)]
struct Input<'a> {
    url: &'a str,
    hop: u64,
    resolved_url: Option<String>,
}
#[derive(Deserialize)]
struct Decision {
    refusal: Option<String>,
    selected_host: Option<String>,
}
fn input_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("url", DataType::Utf8, false),
        Field::new("hop", DataType::UInt64, false),
        Field::new("resolved_url", DataType::Utf8, true),
    ]))
}
