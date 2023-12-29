// MOT/Apache2 License

//! Integrations with `hyper::rt` for `smol` types.

use async_executor::Executor;
use hyper::rt::Sleep;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

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

/// Use the timer from [`async-io`].
#[derive(Debug, Clone, Default)]
pub struct SmolTimer {
    _private: (),
}

impl SmolTimer {
    /// Create a new `SmolTimer`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl hyper::rt::Timer for SmolTimer {
    #[inline]
    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Sleep>> {
        Box::pin(SmolSleep(async_io::Timer::after(duration)))
    }

    #[inline]
    fn sleep_until(&self, at: Instant) -> Pin<Box<dyn Sleep>> {
        Box::pin(SmolSleep(async_io::Timer::at(at)))
    }

    #[inline]
    fn reset(&self, sleep: &mut Pin<Box<dyn Sleep>>, new_deadline: Instant) {
        if let Some(mut sleep) = sleep.as_mut().downcast_mut_pin::<SmolSleep>() {
            sleep.0.set_at(new_deadline);
        } else {
            *sleep = Box::pin(SmolSleep(async_io::Timer::at(new_deadline)));
        }
    }
}

struct SmolSleep(async_io::Timer);

impl Future for SmolSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Sleep for SmolSleep {}
