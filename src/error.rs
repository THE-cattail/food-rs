#[macro_export]
macro_rules! result {
    ($f:expr, $($err:tt)*) => {
        ($f).map_err(|err| anyhow::anyhow!($($err)* err))
    };
}
