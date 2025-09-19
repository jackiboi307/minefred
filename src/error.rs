pub use eyre::Error;
// pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! conv_err {
    () => {{
        |e| crate::prelude::anyhow!(e)
    }};
}

// #[macro_export]
// macro_rules! fmt_err {
//     ($msg:expr) => {{
//         format!("{} ({}:{}:{})", $msg, file!(), line!(), column!())
//     }};
// }

pub use crate::{
    conv_err,
    // fmt_err,
};
