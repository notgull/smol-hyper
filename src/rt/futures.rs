// MIT/Apache2 License

//! Integrations with the `hyper::rt` module for `futures-io` types.

use hyper::rt::ReadBufCursor;

use std::io;
use std::pin::Pin;
use std::slice;
use std::task::{Context, Poll};

pin_project_lite::pin_project! {
    /// This type implements [`hyper`] I/O traits for [`futures-io`] implementors.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct FuturesIo<T: ?Sized> {
        #[pin]
        inner: T
    }
}

impl<T> From<T> for FuturesIo<T> {
    #[inline]
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> FuturesIo<T> {
    /// Wrap a type implementing I/O traits.
    pub fn new(io: T) -> Self {
        io.into()
    }
}

impl<T: ?Sized> FuturesIo<T> {
    /// Get a reference to the inner type.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the inner type.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Get a pinned mutable reference to the inner type.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project().inner
    }
}

impl<T: ?Sized> AsRef<T> for FuturesIo<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.get_ref()
    }
}

impl<T: ?Sized> AsMut<T> for FuturesIo<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T: futures_io::AsyncRead + ?Sized> hyper::rt::Read for FuturesIo<T> {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        // Fill the read buffer with initialized data.
        let read_slice = unsafe {
            let buffer = buf.as_mut();
            buffer.as_mut_ptr().write_bytes(0, buffer.len());
            slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len())
        };

        // Read bytes from the underlying source.
        let n = match self.get_pin_mut().poll_read(cx, read_slice) {
            Poll::Ready(Ok(n)) => n,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Pending => return Poll::Pending,
        };

        // Advance the buffer.
        unsafe {
            buf.advance(n);
        }

        Poll::Ready(Ok(()))
    }
}

impl<T: futures_io::AsyncWrite + ?Sized> hyper::rt::Write for FuturesIo<T> {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_close(cx)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write_vectored(cx, bufs)
    }
}
