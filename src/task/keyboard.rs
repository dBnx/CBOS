use core::{
    pin::Pin,
    task::{Context, Poll, Waker},
};

use alloc::string::String;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream, StreamExt};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

/// Meaning the queue has a size > 128 * 8 ~ 1kiB
const SCANCODE_QUEUE_SIZE: usize = 128;

/// `OnceCell` lets us initialize us wehnever we wan't, meaning not during an interrupt.
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// For `ScancodeStream`
static WAKER: AtomicWaker = AtomicWaker::new();

lazy_static! {
    pub static ref KEYBOARD_STATE: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            kprintln!("[ERROR] scancode queue full");
        } else {
            WAKER.wake();
        }
    } else {
        kprintln!("[WARNING] scancode queue not initialized");
    }
}

pub struct ScancodeStream {
    _prevent_construction: (),
}

impl ScancodeStream {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(SCANCODE_QUEUE_SIZE))
            .expect("ScancodeStream::new() may only be called once");
        ScancodeStream {
            _prevent_construction: (),
        }
    }

    /// Make sure the waker is installed if returning `Poll::Pending`
    #[inline]
    fn ret_pending(waker: &Waker) -> Poll<Option<DecodedKey>> {
        WAKER.register(waker);
        Poll::Pending
    }
}

impl Stream for ScancodeStream {
    type Item = DecodedKey;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // SCANCODE_QUEUE is guaranteed to be initialized by the constructor
        let queue = SCANCODE_QUEUE.try_get().unwrap();
        let scancode: u8 = match queue.pop() {
            Some(scancode) => scancode,
            None => return ScancodeStream::ret_pending(cx.waker()),
        };
        let decoded = match decode_key(scancode) {
            Some(key) => key,
            None => return ScancodeStream::ret_pending(cx.waker()),
        };

        Poll::Ready(Some(decoded))
    }
}

fn decode_key(scancode: u8) -> Option<DecodedKey> {
    let mut keyboard = KEYBOARD_STATE.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            return Some(key);
        }
    }
    None
}

/// Returns a string with all inputs until a Enter key is pressed or the maximal
/// amount of characters, given by `max_len`, is reached.
pub async fn get_and_print_line(kb: &mut ScancodeStream, max_len: usize) -> String {
    use pc_keyboard::DecodedKey::{RawKey, Unicode};

    let mut line = String::with_capacity(10);
    loop {
        while let Some(key) = kb.next().await {
            match key {
                Unicode('\n') => {
                    return line;
                }
                Unicode(character) => {
                    print!("{}", character);
                    line.push(character);
                }
                RawKey(k) => {
                    kprintln!("{:?}", k);
                } //RawKey(_) => {}
            }
            if line.len() > max_len {
                return line;
            }
        }
    }
}
