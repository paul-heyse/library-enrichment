//! Conservative registry requirement admission. Unsupported marker environments fail explicitly.
//!
//! No URL, VCS, editable or filesystem source reaches a package-manager subprocess. The
//! resolver evaluates only the explicit Linux CPython capsule environment and requested extras.
use datafusion::{
    logical_expr::Expr,
    prelude::{col, lit},
};
use pep440_rs::VersionSpecifiers;
use std::ops::Not;
use std::str::FromStr;

crate::native_struct! {
    /// Explicit observed capsule fields; no host defaults are supplied.
    pub struct MarkerEnvironment {
        python_full_version: String => crate::native_union::Rule::Text,
        implementation_version: String => crate::native_union::Rule::Text,
        implementation_name: String => crate::native_union::Rule::Text,
        os_name: String => crate::native_union::Rule::Text,
        platform_machine: String => crate::native_union::Rule::Text,
        platform_system: String => crate::native_union::Rule::Text,
        platform_python_implementation: String => crate::native_union::Rule::Text,
        sys_platform: String => crate::native_union::Rule::Text,
    }
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub name: String,
    pub extras: Vec<String>,
    pub versions: VersionSpecifiers,
    marker: Expr,
}
impl Requirement {
    pub fn parse(text: &str) -> Result<Self, String> {
        if text.len() > 4096 || text.contains(['\n', '\r', '\0', '@', '/', '\\', ':', '#']) {
            return Err("dependency is not an admitted bounded registry requirement".into());
        }
        let (requirement, marker) = text
            .split_once(';')
            .map_or((text.trim(), ""), |(a, b)| (a.trim(), b.trim()));
        let name_end = requirement
            .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')))
            .unwrap_or(requirement.len());
        let name = &requirement[..name_end];
        if !name
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
            || !name
                .as_bytes()
                .last()
                .is_some_and(u8::is_ascii_alphanumeric)
        {
            return Err("invalid registry dependency name".into());
        }
        let mut rest = requirement[name_end..].trim();
        let mut extras = Vec::new();
        if let Some(tail) = rest.strip_prefix('[') {
            let (names, tail) = tail.split_once(']').ok_or("unclosed dependency extras")?;
            for extra in names.split(',') {
                let extra = extra.trim();
                if extra.is_empty()
                    || !extra
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
                {
                    return Err("invalid dependency extra".into());
                }
                extras.push(super::normalize_name(extra));
            }
            rest = tail.trim();
        }
        extras.sort();
        extras.dedup();
        if let Some(inner) = rest.strip_prefix('(') {
            rest = inner
                .strip_suffix(')')
                .ok_or("unclosed version constraint")?
                .trim();
        }
        let versions = VersionSpecifiers::from_str(rest)
            .map_err(|e| format!("unsupported dependency constraint: {e}"))?;
        let tokens = tokenize(marker)?;
        let mut parser = Parser {
            tokens: &tokens,
            position: 0,
        };
        let marker = if tokens.is_empty() {
            lit(true)
        } else {
            parser.or()?
        };
        if parser.position != tokens.len() {
            return Err("trailing dependency marker syntax".into());
        }
        Ok(Self {
            name: super::normalize_name(name),
            extras,
            versions,
            marker,
        })
    }
    /// A native value predicate; this method never evaluates a candidate in Rust.
    pub fn version_predicate(&self, version: Expr) -> Expr {
        crate::native_version::pep440_matches().call(vec![lit(self.versions.to_string()), version])
    }
    /// Native boolean expression over explicitly supplied marker environment fields.
    pub fn marker_predicate(&self) -> Expr {
        self.marker.clone()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Literal(String),
    Word(String),
    Operator(String),
    Open,
    Close,
}
fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    let mut chars = text.chars().peekable();
    let mut out = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '(' => out.push(Token::Open),
            ')' => out.push(Token::Close),
            '\'' | '"' => {
                let quote = c;
                let mut value = String::new();
                let mut closed = false;
                for c in chars.by_ref() {
                    if c == quote {
                        closed = true;
                        break;
                    }
                    value.push(c);
                }
                if !closed {
                    return Err("unclosed marker literal".into());
                }
                out.push(Token::Literal(value));
            }
            '<' | '>' | '=' | '!' | '~' => {
                let mut value = c.to_string();
                while chars.peek() == Some(&'=') {
                    value.push(chars.next().unwrap_or('='));
                }
                if !matches!(
                    value.as_str(),
                    "<" | ">" | "<=" | ">=" | "==" | "!=" | "~=" | "==="
                ) {
                    return Err("unsupported marker operator".into());
                }
                out.push(Token::Operator(value));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut value = c.to_string();
                while chars
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_')
                {
                    value.push(chars.next().unwrap_or('_'));
                }
                out.push(Token::Word(value));
            }
            _ => return Err("unsupported dependency marker syntax".into()),
        }
    }
    if out.len() > 128 {
        return Err("dependency marker is too complex".into());
    }
    Ok(out)
}
struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}
impl Parser<'_> {
    fn word(&mut self, text: &str) -> bool {
        if self.tokens.get(self.position) == Some(&Token::Word(text.into())) {
            self.position += 1;
            true
        } else {
            false
        }
    }
    fn or(&mut self) -> Result<Expr, String> {
        let mut value = self.and()?;
        while self.word("or") {
            let rhs = self.and()?;
            value = value.or(rhs);
        }
        Ok(value)
    }
    fn and(&mut self) -> Result<Expr, String> {
        let mut value = self.atom()?;
        while self.word("and") {
            let rhs = self.atom()?;
            value = value.and(rhs);
        }
        Ok(value)
    }
    fn atom(&mut self) -> Result<Expr, String> {
        if self.tokens.get(self.position) == Some(&Token::Open) {
            self.position += 1;
            let value = self.or()?;
            if self.tokens.get(self.position) != Some(&Token::Close) {
                return Err("unclosed marker grouping".into());
            }
            self.position += 1;
            return Ok(value);
        }
        let left_extra = self.tokens.get(self.position) == Some(&Token::Word("extra".into()));
        let (left, left_version) = self.value()?;
        let op = if self.word("in") {
            "in".to_owned()
        } else if self.word("not") {
            if !self.word("in") {
                return Err("expected 'in' after marker 'not'".into());
            }
            "not in".into()
        } else if let Some(Token::Operator(op)) = self.tokens.get(self.position) {
            self.position += 1;
            op.clone()
        } else {
            return Err("marker comparison operator missing".into());
        };
        let right_extra = self.tokens.get(self.position) == Some(&Token::Word("extra".into()));
        let (right, right_version) = self.value()?;
        if left_extra || right_extra {
            if left_extra && right_extra {
                return Err("extra-to-extra marker comparison is undefined".into());
            }
            let left = normalized_name(left);
            let right = normalized_name(right);
            return match op.as_str() {
                "==" => Ok(left.eq(right)),
                "!=" => Ok(left.not_eq(right)),
                _ => Err("only equality and inequality are admitted for extra markers".into()),
            };
        }

        if (left_version || right_version) && matches!(op.as_str(), "in" | "not in") {
            return Err("containment is not defined for version marker fields".into());
        }
        if op == "in" {
            return Ok(datafusion::functions::unicode::expr_fn::strpos(right, left).gt(lit(0i64)));
        }
        if op == "not in" {
            return Ok(datafusion::functions::unicode::expr_fn::strpos(right, left)
                .gt(lit(0i64))
                .not());
        }
        if left_version || right_version {
            let op = if right_version && !left_version {
                match op.as_str() {
                    "<" => ">",
                    ">" => "<",
                    "<=" => ">=",
                    ">=" => "<=",
                    other => other,
                }
            } else {
                &op
            };
            let (candidate, constraint) = if right_version && !left_version {
                (right, left)
            } else {
                (left, right)
            };
            let specifier =
                datafusion::functions::string::expr_fn::concat(vec![lit(op), constraint]);
            return Ok(crate::native_version::pep440_matches().call(vec![specifier, candidate]));
        }
        match op.as_str() {
            "==" | "===" => Ok(left.eq(right)),
            "!=" => Ok(left.not_eq(right)),
            "<" => Ok(left.lt(right)),
            ">" => Ok(left.gt(right)),
            "<=" => Ok(left.lt_eq(right)),
            ">=" => Ok(left.gt_eq(right)),
            _ => Err("unsupported non-version marker comparison".into()),
        }
    }

    fn value(&mut self) -> Result<(Expr, bool), String> {
        let value = match self.tokens.get(self.position) {
            Some(Token::Literal(value)) => (lit(value.clone()), false),
            Some(Token::Word(name)) => {
                let version = match name.as_str() {
                    "python_version" | "python_full_version" | "implementation_version" => true,
                    "implementation_name"
                    | "os_name"
                    | "platform_machine"
                    | "platform_system"
                    | "platform_python_implementation"
                    | "sys_platform"
                    | "extra" => false,
                    _ => {
                        return Err(format!(
                            "marker environment field {name} is not reproduced by this capsule"
                        ));
                    }
                };
                (col(name), version)
            }
            _ => return Err("marker operand missing".into()),
        };
        self.position += 1;
        Ok(value)
    }
}
fn normalized_name(value: Expr) -> Expr {
    datafusion::functions::regex::expr_fn::regexp_replace(
        datafusion::functions::string::expr_fn::lower(value),
        lit("[-_.]+"),
        lit("-"),
        Some(lit("g")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependency_sources_and_unknown_marker_environments_fail_before_fetch() {
        for text in [
            "pkg @ https://evil.test/x.whl",
            "pkg @ git+https://evil.test/x",
            "../local",
            "-e pkg",
            "pkg; unknown == 'x'",
            "pkg; platform_release == '6.1'",
            "pkg>=1 --index-url=x",
            "pkg; extra=='off' and unknown=='x'",
        ] {
            assert!(Requirement::parse(text).is_err(), "{text}");
        }
    }
}
