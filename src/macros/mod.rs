#[macro_export]
#[doc(hidden)]
macro_rules! __rust_helpers_if_else__ {
    ($cond:expr, $if:expr, $else:expr) => {
        if $cond {
            $if
        } else {
            $else
        }
    };
}

/// Match if macro provides a match like syntax to use make if statements
/// ## Usage syntax:
/// There are two syntaxes for this macro.
/// ```
/// //Unbound syntax
/// let result = rust_helpers::match_if!(
///         15<10 => 15;
///         15>10 => {
///             println!("15 is greater than 10");
///             10
///         };
///         _ =>{
///             println!("15 is equal to 10");
///             0
///         };
/// );
/// assert_eq!(result, 10);
/// //Bound syntax
/// rust_helpers::match_if!(
/// with (15){
///    (<10) => {println!("15 is less than 10");15};
///    (>10) => {println!("15 is greater than 10");10};
///    (==10) => {println!("15 is equal to 10");0};
///    _ => panic!("This should not happen");
///    }
/// );
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __rust_helpers_match_if__ {
    (with ($base:expr) {
        ($($cond1:tt)+)=> $block:expr; $(($($conds:tt)+) => $blocks:expr;)* $(_ => $block3:expr$(;)?)?
    }
)=>{
        $crate::match_if!(
            $base $($cond1)+ => $block;
            $($base $($conds)+ => $blocks;)*
            $(_ => $block3)?
    )
    };
    ($cond1:expr => $block:expr; $($conds:expr => $blocks:expr;)* $(_ => $block3:expr$(;)?)?) => {
        if $cond1 {
            $block
        } $(
            else if $conds{
                $blocks
            }
        )*
        $(
            else {
                $block3
            }
        )?
    };
}

/// # Inherit Struct Fields
/// This macro is used to inherit common fields one struct to another.
/// You simply define the struct with different fields as you would normally do in rust and wrap it in this macro.
/// ## Usage syntax:
/// ```ignore
/// extends!(
/// [meta]*
/// [pub] base struct [name]{
///     [
///     [meta]*
///     [pub] [field]: [type]
///     ],*
/// }
/// [meta]*
/// [pub] struct [name]{
///    [
///    [meta]*
///    [pub] [field]: [type]
///    ],*
/// }
/// );
/// ```
/// ## Usage Example:
/// ```rust
/// rust_helpers::extends!{
/// pub base struct Base{
///     pub a: u32,
/// }
/// #[derive(Debug, Clone)]
/// pub struct Inherited{
///    pub b:String,
///    //#[serde(skip)]
///    pub others: Vec<usize>,
///    }
/// }
/// let a = Inherited{
///     a: 1,
///     b: "hello".to_string(),
///     others: vec![1,2,3],
/// };
/// ```
/// Both struct will have the common fields from the base struct.
///
#[macro_export]
#[doc(hidden)]
macro_rules! __rust_helpers_extends__ {
    (
    $(
        $(#[$($base_child_meta:meta),*])*
        $base_pub_i:vis $base_field:ident: $base_type:ty
    ),* $(,)? ;
    $(#[$($top_meta:meta),*])* $pub:vis struct $name:ident$(<$l:lifetime>)? {
    $(
    $(#[$($child_meta:meta),*])*
        $pub_i:vis $field:ident: $type:ty
    ),* $(,)?
    }
)=>{
    $(#[$($top_meta),*])*
    $pub struct $name$(<$l>)?{
    $(
        $(#[$($base_child_meta),*])*
        $base_pub_i $base_field : $base_type,
        )*
            $(
        $(#[$($child_meta),*])*
        $pub_i $field : $type
        ),*
    }
};
    (
    $(#[$($base_top_meta:meta),*])* $base_pub:vis base struct  $base_name:ident$(<$base_l:lifetime>)? {$(
    $(#[$($base_child_meta:meta),*])*
        $base_pub_i:vis $base_field:ident: $base_type:ty),* $(,)?
    }
    $(
    $(#[$($top_meta:meta),*])* $pub:vis struct  $name:ident$(<$l:lifetime>)? {
    $(
        $(#[$($child_meta:meta),*])*
        $pub_i:vis $field:ident: $type:ty),* $(,)?
    }
    )*
) => {
        $(#[$($base_top_meta),*])*
        $base_pub struct $base_name$(<$base_l>)?{
        $(
        $(#[$($base_child_meta),*])*
        $base_pub_i $base_field : $base_type
        ),*
        }
        $crate::extends!(
        $(
        $(#[$($base_child_meta),*])*
        $base_pub_i $base_field: $base_type
    ),*;
    $($(#[$($top_meta),*])* $pub struct $name$(<$l>)? {
        $(
        $(#[$($child_meta),*])*
        $pub_i $field: $type
    ),*
    })*
    );
    }
}
#[doc(inline)]
pub use __rust_helpers_extends__ as extends;
#[doc(inline)]
pub use __rust_helpers_if_else__ as if_else;
#[doc(inline)]
pub use __rust_helpers_match_if__ as match_if;
#[cfg(test)]
mod tests {
    pub use super::{extends, if_else, match_if};
    #[test]
    fn test_extends() {
        extends! {
            #[allow(unused)]
            pub base struct Base{
            pub a: u32,
            pub g: u32,
            pub foo: u32,
            }
            #[derive(Debug, Clone,serde::Serialize)]
            pub struct Inherited{
            pub b:String,
            #[serde(skip)]
            pub others: Vec<usize>,
            }
        }
        let a = Inherited {
            a: 1,
            g: 2,
            foo: 3,
            b: "hello".to_string(),
            others: vec![1, 2, 3],
        };
        assert_eq!(a.a, 1);
        assert_eq!(a.b, "hello");
        assert_eq!(a.others, vec![1, 2, 3]);
    }
    #[test]
    fn test_if_else() {
        let a = 1;
        let b = 2;
        let c = if_else!(a > b, a, b);
        assert_eq!(c, 2);
    }

    #[test]
    fn test_match_if() {
        let a = 1;
        let b = 2;
        let c = match_if!(
            a > b => a;
            a < b => b;
            _ => 0
        );
        assert_eq!(c, b);
    }
    #[test]
    fn test_match_if_with() {
        let a = 1;
        let b = 2;
        let c = match_if!(
        with (a){
             (> b) => a;
             (< b) => b;
            _ => 0;
        }
        );
        assert_eq!(c, b);
    }
}
