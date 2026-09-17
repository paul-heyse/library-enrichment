//! Producer-supplied declaration facts. Renderings describe syntax, never type equivalence.
use crate::native_union::{Rule, Unit};

crate::native_union! {
    pub enum RustStabilityLevel {
        Stable = "stable" { since: Option<String> => Rule::NonEmpty },
        Unstable = "unstable",
    }
}
crate::native_struct! {
    pub struct RustStability {
        feature: String => Rule::NonEmpty,
        level: RustStabilityLevel => Rule::Text,
    }
}
crate::native_union! {
    /// A body/value is distinct from a declaration without a supplied implementation.
    pub enum RustBody {
        Absent = "absent",
        Present = "present" { unstable_default_feature: Option<String> => Rule::NonEmpty },
    }
}
crate::native_union! {
    pub enum RustAbi {
        Rust = "rust",
        C = "c" { unwind: bool => Rule::Text },
        Cdecl = "cdecl" { unwind: bool => Rule::Text },
        Stdcall = "stdcall" { unwind: bool => Rule::Text },
        Fastcall = "fastcall" { unwind: bool => Rule::Text },
        Aapcs = "aapcs" { unwind: bool => Rule::Text },
        Win64 = "win64" { unwind: bool => Rule::Text },
        SysV64 = "sysv64" { unwind: bool => Rule::Text },
        System = "system" { unwind: bool => Rule::Text },
        Other = "other" { name: String => Rule::NonEmpty },
    }
}
crate::native_struct! {
    pub struct RustParameter {
        ordinal: u32 => Rule::Coordinate(Unit::Ordinal),
        pattern: String => Rule::Text,
        type_rendering: String => Rule::Text,
    }
}
crate::native_struct! {
    pub struct RustCallable {
        parameters: Vec<RustParameter> => Rule::Sequence,
        output: Option<String> => Rule::Text,
        c_variadic: bool => Rule::Text,
        is_const: bool => Rule::Text,
        is_unsafe: bool => Rule::Text,
        is_async: bool => Rule::Text,
        abi: RustAbi => Rule::Text,
        generics: String => Rule::Text,
    }
}
crate::native_struct! {
    pub struct RustDetails {
        stability: Option<RustStability> => Rule::Text,
        const_stability: Option<RustStability> => Rule::Text,
        body: Option<RustBody> => Rule::Text,
        callable: Option<RustCallable> => Rule::Text,
    }
}

crate::native_vocabulary! {
    pub enum PythonParameterKind {
        PositionalOnly = "positional-only",
        PositionalOrKeyword = "positional or keyword",
        VarPositional = "variadic positional",
        KeywordOnly = "keyword-only",
        VarKeyword = "variadic keyword",
    }
}
crate::native_vocabulary! {
    /// Native interpretation of Griffe's reported value and parameter kind.
    pub enum PythonDefaultOrigin {
        Absent = "absent", Declared = "declared", ImplicitVariadic = "implicit_variadic",
    }
}
crate::native_struct! {
    pub struct PythonParameter {
        ordinal: u32 => Rule::Coordinate(Unit::Ordinal),
        name: String => Rule::NonEmpty,
        kind: Option<PythonParameterKind> => Rule::Vocabulary(PythonParameterKind::VALUES.iter().map(|v| (*v).into()).collect()),
        annotation: Option<String> => Rule::Text,
        reported_default: Option<String> => Rule::Text,
        default_origin: Option<PythonDefaultOrigin> => Rule::Vocabulary(PythonDefaultOrigin::VALUES.iter().map(|v| (*v).into()).collect()),
    }
}
crate::native_struct! {
    pub struct PythonCallable {
        parameters: Vec<PythonParameter> => Rule::Sequence,
        returns: Option<String> => Rule::Text,
        labels: Vec<String> => Rule::Set,
    }
}
crate::native_struct! {
    pub struct PythonOverload {
        ordinal: u32 => Rule::Coordinate(Unit::Ordinal),
        signature: String => Rule::Text,
        callable: PythonCallable => Rule::Text,
    }
}
crate::native_struct! {
    pub struct PythonBase {
        ordinal: u32 => Rule::Coordinate(Unit::Ordinal),
        rendering: String => Rule::Text,
    }
}
