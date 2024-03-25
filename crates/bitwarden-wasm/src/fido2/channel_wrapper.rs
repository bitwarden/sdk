use futures::{Future, SinkExt, StreamExt};
use std::rc::Rc;

/// This is just a thin wrapper around the channels to make it easier to use
/// Instead of dealing with tx and rx in two separate steps, the `call` function does it for you
pub struct CallerChannel<In, Out> {
    chan: async_lock::Mutex<(
        futures::channel::mpsc::Sender<In>,
        futures::channel::mpsc::Receiver<Out>,
    )>,
}

impl<In, Out> CallerChannel<In, Out> {
    pub fn new(
        tx: futures::channel::mpsc::Sender<In>,
        rx: futures::channel::mpsc::Receiver<Out>,
    ) -> Self {
        Self {
            chan: async_lock::Mutex::new((tx, rx)),
        }
    }

    pub async fn call(&self, params: In) -> Result<Option<Out>, futures::channel::mpsc::SendError> {
        let mut lock = self.chan.lock().await;
        lock.0.send(params).await?;
        Ok(lock.1.next().await)
    }
}

/// This struct wraps the object that we want to call functions through channels on
/// It's only purpose is holding a reference to the inner object so we can create the channels based on it later
pub struct ChannelWrapped<Inner> {
    inner: Rc<Inner>,
}

impl<Inner> ChannelWrapped<Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    /// Create a channel for calling a function on the inner object.
    pub fn create_channel<In, Out, F, Fut>(&self, function: F) -> CallerChannel<In, Out>
    where
        F: Fn(Rc<Inner>, In) -> Fut + 'static,
        Fut: Future<Output = Out> + 'static,
        Inner: 'static,
        In: 'static,
        Out: 'static,
    {
        let (tx_caller, mut rx_receiver) = futures::channel::mpsc::channel::<In>(1);
        let (mut tx_receiver, rx_caller) = futures::channel::mpsc::channel::<Out>(1);

        let inner = self.inner.clone();
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                let Some(params) = rx_receiver.next().await else {
                    return;
                };
                let result = function(inner.clone(), params).await;
                tx_receiver.send(result).await.unwrap();
            }
        });

        CallerChannel::new(tx_caller, rx_caller)
    }
}
