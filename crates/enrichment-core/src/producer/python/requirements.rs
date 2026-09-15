//! Conservative registry requirement admission. Unsupported marker environments fail explicitly.
//!
//! No URL, VCS, editable or filesystem source reaches a package-manager subprocess. The
//! resolver evaluates only the explicit Linux CPython capsule environment and requested extras.
use pep440_rs::{Version, VersionSpecifiers};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Requirement {
    pub name: String,
    pub extras: Vec<String>,
    pub versions: VersionSpecifiers,
    marker: Vec<Token>,
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
        let marker = tokenize(marker)?;
        let result = Self {
            name: super::normalize_name(name),
            extras,
            versions,
            marker,
        };
        // Validate every branch, even when an extra condition would currently make it false.
        result.applies(&[])?;
        Ok(result)
    }
    pub fn permits(&self, version: &str) -> bool {
        Version::from_str(version).is_ok_and(|v| self.versions.contains(&v))
    }
    pub fn applies(&self, extras: &[String]) -> Result<bool, String> {
        if self.marker.is_empty() {
            return Ok(true);
        }
        let mut parser = Parser {
            tokens: &self.marker,
            position: 0,
            extras,
        };
        let matches = parser.or()?;
        if parser.position != self.marker.len() {
            return Err("trailing dependency marker syntax".into());
        }
        Ok(matches)
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
    extras: &'a [String],
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
    fn or(&mut self) -> Result<bool, String> {
        let mut value = self.and()?;
        while self.word("or") {
            let rhs = self.and()?;
            value |= rhs;
        }
        Ok(value)
    }
    fn and(&mut self) -> Result<bool, String> {
        let mut value = self.atom()?;
        while self.word("and") {
            let rhs = self.atom()?;
            value &= rhs;
        }
        Ok(value)
    }
    fn atom(&mut self) -> Result<bool, String> {
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
            let selected = super::normalize_name(if left_extra { &right } else { &left });
            let contains = self
                .extras
                .iter()
                .any(|v| super::normalize_name(v) == selected);
            return match op.as_str() {
                "==" => Ok(contains),
                "!=" => Ok(!contains),
                _ => Err("only equality and inequality are admitted for extra markers".into()),
            };
        }
        if (left_version || right_version) && matches!(op.as_str(), "in" | "not in") {
            return Err("containment is not defined for version marker fields".into());
        }
        if op == "in" {
            return Ok(right.contains(&left));
        }
        if op == "not in" {
            return Ok(!right.contains(&left));
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
                (&right, &left)
            } else {
                (&left, &right)
            };
            let spec = VersionSpecifiers::from_str(&format!("{op}{constraint}"))
                .map_err(|e| e.to_string())?;
            let version = Version::from_str(candidate).map_err(|e| e.to_string())?;
            return Ok(spec.contains(&version));
        }
        match op.as_str() {
            "==" | "<=" | ">=" => Ok(left == right),
            "!=" => Ok(left != right),
            "<" | ">" => Ok(false),
            _ => Err("unsupported non-version marker comparison".into()),
        }
    }
    fn value(&mut self) -> Result<(String, bool), String> {
        let value = match self.tokens.get(self.position) {
            Some(Token::Literal(value)) => (value.clone(), false),
            Some(Token::Word(name)) => {
                let (value, version) = match name.as_str() {
                    "python_version" => ("3.14", true),
                    "python_full_version" | "implementation_version" => ("3.14.7", true),
                    "implementation_name" => ("cpython", false),
                    "os_name" => ("posix", false),
                    "platform_machine" => ("x86_64", false),
                    "platform_system" => ("Linux", false),
                    "platform_python_implementation" => ("CPython", false),
                    "sys_platform" => ("linux", false),
                    "extra" => ("", false),
                    _ => {
                        return Err(format!(
                            "marker environment field {name} is not reproduced by this capsule"
                        ));
                    }
                };
                (value.into(), version)
            }
            _ => return Err("marker operand missing".into()),
        };
        self.position += 1;
        Ok(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_requirements_admit_versions_extras_and_explicit_markers() {
        let req = Requirement::parse("Some_Package[fast]>=1.2,<2; python_version >= '3.10' and (sys_platform == 'linux' or extra == 'other')").unwrap();
        assert_eq!(req.name, "some-package");
        assert_eq!(req.extras, ["fast"]);
        assert!(req.permits("1.9"));
        assert!(!req.permits("2.0"));
        assert!(req.applies(&[]).unwrap());
        let optional = Requirement::parse("optional; extra == 'speed'").unwrap();
        assert!(!optional.applies(&[]).unwrap());
        assert!(optional.applies(&["speed".into()]).unwrap());
        assert!(
            Requirement::parse("pkg; '3.10' < python_version")
                .unwrap()
                .applies(&[])
                .unwrap()
        );
    }
    #[test]
    fn current_marker_rules_use_field_types_and_extra_membership() {
        assert!(
            !Requirement::parse("pkg; os_name >= 'foo'")
                .unwrap()
                .applies(&[])
                .unwrap()
        );
        assert!(
            !Requirement::parse("pkg; os_name > 'abc'")
                .unwrap()
                .applies(&[])
                .unwrap()
        );
        assert!(
            Requirement::parse("pkg; os_name <= 'posix'")
                .unwrap()
                .applies(&[])
                .unwrap()
        );
        assert!(
            !Requirement::parse("pkg; extra != 'foo'")
                .unwrap()
                .applies(&["foo".into()])
                .unwrap()
        );
        assert!(
            Requirement::parse("pkg; extra == 'fast_mode'")
                .unwrap()
                .applies(&["FAST-Mode".into()])
                .unwrap()
        );
        assert!(Requirement::parse("pkg; python_version in '3.14'").is_err());
        assert!(Requirement::parse("pkg; os_name ~= 'posix'").is_err());
    }
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
