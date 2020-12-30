use crate::event::Event;
use crate::listener::Listener;
use crate::marshal::{Marshal, Unmarshal};
use nats;
use std::io;

/// A Node wraps a nats connection
pub struct Node<M, U>
where
    M: Marshal + Sync,
    U: Unmarshal + Clone + Send + Sync + 'static,
{
    client: nats::Connection,
    marshaller: M,
    unmarshaller: U,
}

impl<M, U> Node<M, U>
where
    M: Marshal + Sync,
    U: Unmarshal + Clone + Send + Sync + 'static,
{
    /// Create a new node
    pub fn new(marshal: M, unmarshal: U) -> io::Result<Node<M, U>> {
        let client = nats::connect("0.0.0.0:4222")?;
        Ok(Node {
            client: client,
            marshaller: marshal,
            unmarshaller: unmarshal,
        })
    }

    /// Publish an event
    pub fn publish(&self, e: Event) -> io::Result<()> {
        self.client
            .publish(&e.subject, self.marshaller.marshal(e.data))
    }

    /// Subscribe using a listener
    pub fn subscribe<F>(&self, listener: Listener<F>) -> io::Result<nats::subscription::Handler>
    where
        F: Fn(Vec<u8>) -> io::Result<()> + Send + Sync + Clone + 'static,
    {
        let unmarshaller = self.unmarshaller.clone();
        let l = listener.clone();

        let unmarshalling_handler = move |msg: nats::Message| {
            let unmarshalled_data = unmarshaller.unmarshal(msg.data);
            l.handler.call((unmarshalled_data,))
        };
        let h = self
            .client
            .subscribe(&listener.subject)?
            .with_handler(unmarshalling_handler);
        Ok(h)
    }
}
