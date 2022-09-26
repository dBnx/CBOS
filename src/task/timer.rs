use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll, Waker},
};
use futures_util::{task::AtomicWaker, Stream};

// FIXME: Use lockfree (fixed size) array to store wakers of multiple timers
//const MAX_AMOUNT_OF_CONCURRENT_TIMERS: usize = 64;
static TIMER_COUNTER: AtomicU64 = AtomicU64::new(0);
static TIMER_WAKER: OnceCell<AtomicWaker> = OnceCell::uninit();
//static TIMER_WAKERS: OnceCell<ArrayQueue<Option<AtomicWaker>>> = OnceCell::uninit();
pub(crate) fn tick() {
    TIMER_COUNTER.fetch_add(1, Ordering::Relaxed);
    if let Ok(waker) = TIMER_WAKER.try_get() {
        waker.wake();
    }
}

pub struct TickStream {
    ticks: u64,
    // Atomic so that we can store it in the `Stream` impl, as we don't have a &mut,
    // because of the Pinning
    last_tick: AtomicU64,
    //waker_id: usize,
}

impl TickStream {
    #[must_use]
    pub fn new(ticks: u64) -> Self {
        TIMER_WAKER
            .try_init_once(AtomicWaker::new)
            .expect("Currently only one TickStream (Timer) can exist");
        Self {
            ticks,
            last_tick: AtomicU64::new(TIMER_COUNTER.load(Ordering::Relaxed)),
            //waker_id: TickStream::get_new_waker(),
        }
    }

    #[allow(dead_code)]
    fn get_new_waker() -> usize {
        todo!()
    }

    /// Make sure the waker is installed if returning `Poll::Pending`
    #[inline]
    fn ret_pending(&self, waker: &Waker) -> Poll<Option<()>> {
        let _ = self;
        TIMER_WAKER.try_get().unwrap().register(waker);
        Poll::Pending
    }
}
impl Stream for TickStream {
    type Item = ();
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let ticks = TIMER_COUNTER.load(Ordering::Relaxed);
        let difference = ticks - self.last_tick.load(Ordering::Relaxed);
        if difference >= self.ticks {
            self.last_tick.fetch_add(difference, Ordering::Relaxed);
            Poll::Ready(Some(()))
        } else {
            self.ret_pending(cx.waker())
        }
    }
}
