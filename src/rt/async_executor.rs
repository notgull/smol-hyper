// MOT/Apache2 License

//! Integrations with `hyper::rt` for `smol` types.

use async_executor::Executor;

use std::future::Future;

/// Use an [`async-executor`] as an executor.
#[derive(Debug, Clone)]
pub struct SmolExecutor<E: ?Sized>(E);

impl<E> From<E> for SmolExecutor<E> {
    #[inline]
    fn from(inner: E) -> Self {
        Self(inner)
    }
}

impl<E> SmolExecutor<E> {
    /// Create a new `SmolExecutor` based around something that derefs to an [`Executor`].
    pub fn new(inner: E) -> Self {
        inner.into()
    }

    /// Get the inner type.
    #[inline]
    pub fn into_inner(self) -> E {
        self.0
    }
}

impl<E: ?Sized> SmolExecutor<E> {
    /// Get a reference to the underlying executor.
    pub fn get_ref(&self) -> &E {
        &self.0
    }

    /// Get a mutable reference to the underlying executor.
    pub fn get_mut(&mut self) -> &mut E {
        &mut self.0
    }
}

impl<E: ?Sized> AsRef<E> for SmolExecutor<E> {
    #[inline]
    fn as_ref(&self) -> &E {
        self.get_ref()
    }
}

impl<E: ?Sized> AsMut<E> for SmolExecutor<E> {
    #[inline]
    fn as_mut(&mut self) -> &mut E {
        self.get_mut()
    }
}

impl<'a, E: AsRef<Executor<'a>> + ?Sized, Fut: Future + Send + 'a> hyper::rt::Executor<Fut>
    for SmolExecutor<E>
where
    Fut::Output: Send + 'a,
{
    #[inline]
    fn execute(&self, fut: Fut) {
        self.get_ref().as_ref().spawn(fut).detach();
    }
}
