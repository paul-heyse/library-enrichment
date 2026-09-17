#![feature(staged_api, rustc_attrs, associated_type_defaults)]
#![stable(feature = "schema_fixture", since = "1.0.0")]

#[stable(feature = "ordinary_stable", since = "1.2.0")]
pub struct Stable;
#[unstable(feature = "ordinary_unstable", issue = "none")]
pub struct Unstable;

#[stable(feature = "function_stable", since = "1.0.0")]
#[rustc_const_stable(feature = "const_stable", since = "1.3.0")]
pub const fn stable_const(value: u32) -> u32 { value }

#[stable(feature = "function_unstable_const", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_unstable", issue = "none")]
pub const fn unstable_const(value: u32) -> u32 { value }

#[stable(feature = "defaults_trait", since = "1.0.0")]
pub trait Defaults {
    #[stable(feature = "default_method", since = "1.0.0")]
    #[rustc_default_body_unstable(feature = "function_default", issue = "none")]
    fn method(&self, (left, right): (u32, u32)) -> u32 { left + right }

    #[stable(feature = "default_constant", since = "1.0.0")]
    #[rustc_default_body_unstable(feature = "constant_default", issue = "none")]
    const VALUE: u32 = 7;

    #[stable(feature = "default_type", since = "1.0.0")]
    #[rustc_default_body_unstable(feature = "type_default", issue = "none")]
    type Value = u32;

    #[stable(feature = "required_method", since = "1.0.0")]
    fn required(&self);
}
