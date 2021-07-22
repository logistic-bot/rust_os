use crate::println;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use futures_util::task::AtomicWaker;

use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::print;

/// Statically-allocated fixed-size queue of scancodes
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// Statically-allocated waker to notify the executor when a new scancode is available
static WAKER: AtomicWaker = AtomicWaker::new();

/// Stream of scancodes (Singleton)
pub struct ScancodeStream {
    /// Prevents construction from outside the module.
    _private: (),
}

impl ScancodeStream {
    /// Initializes the scancode queue.
    ///
    /// # Panics
    /// Should only be called once.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("SCANCODE_QUEUE uninitialized");

        // fast path
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            // Scancode found
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            // Missing scancode, queue empty
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

/// Called by the keyboard interrupt handler
///
/// Must not block or allocated.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!(
                "WARNING: scancode queue full; dropping keyboard input {}",
                scancode
            );
        } else {
            // notify the executor that a new scancode is available
            WAKER.wake();
        }
    } else {
        println!(
            "WARNING: scancode queue uninitialized; dropping keyboard input {}",
            scancode
        );
    }
}

/// Wait for keypresses, and print them to the screen
///
/// Since the poll_next function never returns None, this task never finishes
pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}
