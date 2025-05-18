use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use crate::print;

static PRINT_QUEUE: OnceCell<ArrayQueue<&'static str>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Called to add a print job to the queue
///
/// Must not block or allocate.
pub fn add_print_job(s: &'static str) {
    if let Ok(queue) = PRINT_QUEUE.try_get() {
        if let Err(_) = queue.push(s) {
            // Optionally drop or log
        } else {
            WAKER.wake();
        }
    } else {
        // Optionally log
    }
}

pub struct PrintQueueStream {
    _private: (),
}

impl PrintQueueStream {
    pub fn new() -> Self {
        PRINT_QUEUE
            .try_init_once(|| ArrayQueue::new(128))
            .expect("PrintQueueStream::new should only be called once");
        PrintQueueStream { _private: () }
    }
}

impl Stream for PrintQueueStream {
    type Item = &'static str;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<&'static str>> {
        let queue = PRINT_QUEUE
            .try_get()
            .expect("print queue not initialized");

        // fast path
        if let Some(s) = queue.pop() {
            return Poll::Ready(Some(s));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(s) => {
                WAKER.take();
                Poll::Ready(Some(s))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_queue_task() {
    let mut print_stream = PrintQueueStream::new();
    while let Some(s) = print_stream.next().await {
        print!("{}", s);
    }
} 