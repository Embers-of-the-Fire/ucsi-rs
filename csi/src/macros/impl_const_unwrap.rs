/// **Note:** Use the alias `csi::macros::unwrap::unwrap_option_const`.
/// 
/// This will be removed once `Option::unwrap` is const-able.
#[macro_export]
macro_rules! __impl_unwrap_option_const {
    ($op:expr) => {
        match $op {
            ::std::option::Option::Some(t) => t,
            ::std::option::Option::None => panic!("called `Option::unwrap()` on a `None` value")
        }
    };
}