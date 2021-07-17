use std::future::{ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::future::{Either, Shared, WeakShared};
use futures_util::{FutureExt, Stream};
use tokio::sync::{mpsc, oneshot};

pub struct Shutdown {
    tx: oneshot::Sender<()>,
    alive: mpsc::Receiver<()>,
    signal: Shared<ShutdownSignal>,
}

#[derive(Clone)]
pub struct ShutdownHandle {
    signal: Option<WeakShared<ShutdownSignal>>,
}

#[pin_project::pin_project]
struct ShutdownSignal {
    alive: mpsc::Sender<()>,
    #[pin]
    rx: oneshot::Receiver<()>,
}

impl Shutdown {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel();
        let (alive_tx, alive_rx) = mpsc::channel(1);

        Shutdown {
            tx,
            alive: alive_rx,
            signal: ShutdownSignal {
                alive: alive_tx,
                rx,
            }
            .shared(),
        }
    }

    pub fn handle(&self) -> ShutdownHandle {
        ShutdownHandle {
            signal: self.signal.downgrade(),
        }
    }

    pub async fn shutdown(mut self) {
        // send out shutdown signal
        let _ = self.tx.send(());
        drop(self.signal);

        // once every shutdown and thus every channel sender got dropped, the recv call will
        // return an error which is our signal that everything shutdown and that we are done
        let _ = self.alive.recv().await;
    }
}

impl ShutdownHandle {
    pub fn signal(&self) -> impl Future<Output = ()> {
        match self.signal.as_ref() {
            Some(signal) => signal
                .upgrade()
                .map(Either::Left)
                .unwrap_or_else(|| Either::Right(ready(()))),
            None => Either::Right(ready(())),
        }
    }
}

impl Future for ShutdownSignal {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        match this.rx.poll(cx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A stream that can be aborted via a shutdown signal. Once aborted, the stream will yield a final
/// `None` to gracefully shutdown all stream receivers.
#[pin_project::pin_project]
pub struct AbortableStream<F, S> {
    #[pin]
    state: State<F, S>,
}

#[pin_project::pin_project(project = StateProj)]
enum State<F, S> {
    Stream {
        #[pin]
        shutdown_signal: F,
        #[pin]
        stream: S,
    },
    Done,
}

impl<F, S> AbortableStream<F, S> {
    pub fn new(shutdown_signal: F, stream: S) -> Self {
        AbortableStream {
            state: State::Stream {
                shutdown_signal,
                stream,
            },
        }
    }
}

impl<F, S> Stream for AbortableStream<F, S>
where
    F: Future,
    S: Stream,
{
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();

        match this.state.as_mut().project() {
            StateProj::Stream {
                shutdown_signal,
                stream,
            } => {
                // check the stream only if the shutdown signal is still pending (aka. no shut-
                // down signal received yet)
                if shutdown_signal.poll(cx).is_pending() {
                    // if the stream yields a new item, return it without changing the state of
                    // the abortable stream
                    let item = match stream.poll_next(cx) {
                        Poll::Ready(item) => item,
                        Poll::Pending => return Poll::Pending,
                    };
                    if item.is_some() {
                        return Poll::Ready(item);
                    }
                }

                // if the stream does not yield anymore items, or if a shutdown signal was
                // received, close the abortable stream
                this.state.set(State::Done);
            }
            StateProj::Done => {}
        }

        Poll::Ready(None)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.state {
            State::Stream { ref stream, .. } => stream.size_hint(),
            State::Done => (0, Some(0)),
        }
    }
}
