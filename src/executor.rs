use std;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use futures::{Async, Poll};
use futures::future::{BoxFuture, Future, IntoFuture, ok};
use napi::{NapiEnv, create_async_work, delete_async_work, queue_async_work};
use error::Result;

pub struct NapiFuture<T, E> {
    receiver: Receiver<std::result::Result<T, E>>,
}

impl<T, E> Future for NapiFuture<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.try_recv() {
            Ok(Ok(val)) => Ok(Async::Ready(val)),
            Ok(Err(err)) => Err(err),
            Err(TryRecvError::Empty) => Ok(Async::NotReady),
            // todo @jupp0r: place a From<TryRecvError> constraint on E?
            Err(TryRecvError::Disconnected) => Ok(Async::NotReady),
        }
    }
}

pub fn spawn_fn<F, R>(env: NapiEnv, mut function: F) -> NapiFuture<R::Item, R::Error>
    where F: FnOnce() -> R + Send + 'static,
          R: IntoFuture + 'static,
          R::Future: Send + 'static,
          R::Item: Send + 'static,
          R::Error: Send + 'static
{
    println!("spawn");
    let (tx, rx) = channel();
    let work = create_async_work(env,
                                 move |env_| {
        println!("async_work");
        function()
            .into_future()
            .then(|val| {
                      println!("send value");
                      tx.send(val);
                      ok::<(), R::Error>(())
                  });
    },
                                 |env, status| {
                                     println!("async work complete");
                                 })
            .unwrap();
    queue_async_work(env, work).unwrap();
    NapiFuture { receiver: rx }
}
