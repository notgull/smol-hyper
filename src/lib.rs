// MIT/Apache2 License

//! Implementation of [`hyper`]'s runtime traits for [`smol`].
//!
//! The goal of this crate is to allow for greater integration between [`hyper`] and
//! [`smol`]. It is deliberately constrained and implements the following:
//!
//! - Implements [`hyper::rt::Read`] and [`hyper::rt::Write`] for
//!   [`futures_io::AsyncRead`] and [`futures_io::AsyncWrite`], respectively.
//! - Implements [`hyper::rt::Executor`] on [`SmolExecutor`], which wraps around
//!   something that derefs to [`smol::Executor`] (`&Executor`, `Arc<Executor>`,
//!   etc).
//! - Implements [`hyper::rt::Timer`] on [`SmolTimer`], which uses the
//!   [`async_io::Timer`] type to create timeouts.
//!
//! This crate should allow for [`smol`]'s type to be used in [`hyper`] contexts.
//!
//! [`smol`]: https://crates.io/crates/smol
//! [`hyper`]: https://crates.io/crates/hyper
//! [`hyper::rt::Read`]: https://docs.rs/hyper/latest/hyper/rt/trait.Read.html
//! [`hyper::rt::Write`]: https://docs.rs/hyper/latest/hyper/rt/trait.Write.html
//! [`futures_io::AsyncRead`]: https://docs.rs/futures-io/latest/futures_io/trait.AsyncRead.html
//! [`futures_io::AsyncWrite`]: https://docs.rs/futures-io/latest/futures_io/trait.AsyncWrite.html
//! [`hyper::rt::Executor`]: https://docs.rs/hyper/latest/hyper/rt/trait.Executor.html
//! [`hyper::rt::Timer`]: https://docs.rs/hyper/latest/hyper/rt/trait.Timer.html
//! [`async_io::Timer`]: https://docs.rs/async_io/latest/async-io/struct.Timer.html
//! [`SmolExecutor`]: crate::rt::SmolExecutor
//! [`SmolTimer`]: crate::rt::SmolTimer
//! [`smol::Executor`]: https://docs.rs/async_executor/latest/async-executor/struct.Executor.html

pub mod rt;
