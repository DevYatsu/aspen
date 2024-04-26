/// Implements From for a given enum!
///
/// The macro simply does:
///
/// ```rust
/// impl<'a> From<X<'a>> for Y<'a> {
///     // rest of the code
/// }
/// ```
#[macro_export]
macro_rules! impl_from_for {
    ($struct_name:ident, $struct_for:ident) => {
        impl<'a> From<$struct_name<'a>> for $struct_for<'a> {
            fn from(value: $struct_name<'a>) -> Self {
                Self::$struct_name(value)
            }
        }
    };
}
