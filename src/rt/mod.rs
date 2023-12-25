// MIT/Apache2 License

//! Integrations with the [`hyper::rt`] module.

mod futures;
pub use futures::FuturesIo;

#[cfg(feature = "smol")]
mod smol;
#[cfg(feature = "smol")]
pub use smol::{SmolExecutor, SmolTimer};
