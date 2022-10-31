#[macro_export]
macro_rules! result {
    ($f:expr, $($args:tt)*) => {
        ($f).map_err(|err| anyhow::anyhow!($($args)* err))
    };
}
