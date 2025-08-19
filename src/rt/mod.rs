// MIT/Apache2 License

//! Integrations with the [`hyper::rt`] module.

mod futures;
pub use futures::FuturesIo;

#[cfg(feature = "async-executor")]
mod async_executor;
#[cfg(feature = "async-io")]
mod async_io;
#[cfg(feature = "async-executor")]
pub use async_executor::SmolExecutor;
#[cfg(feature = "async-io")]
pub use async_io::SmolTimer;
