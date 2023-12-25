// MOT/Apache2 License

//! Integrations with `hyper::rt` for `smol` types.

use super::FuturesIo;
use hyper::rt::Sleep;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// Use the timer from [`async-io`].
#[derive(Debug, Clone)]
pub struct SmolTimer {
    _private: ()
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
            Poll::Pending => Poll::Pending
        }
    }
}

impl Sleep for SmolSleep {}
