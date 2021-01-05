use crate::event::Event;
use crate::listener::Listener;
use crate::marshal::{Marshal, Unmarshal};
use nats;
use std::io;

/// A Node wraps a nats connection
pub struct Node {
    client: nats::Connection,
}

impl Node {
    /// Create a new node
    pub fn new() -> io::Result<Node> {
        let client = nats::connect("0.0.0.0:4222")?;
        Ok(Node { client: client })
    }

    /// Publish an event
    pub fn publish<M: Marshal>(&self, e: Event<M>) -> io::Result<()> {
        self.client.publish(&e.subject, e.marshalled_data())
    }

    /// Subscribe using a listener
    pub fn subscribe<U, F>(
        &self,
        listener: Listener<'static, U, F>,
    ) -> io::Result<nats::subscription::Handler>
    where
        U: Unmarshal + Clone + Send + Sync + 'static,
        F: Fn(U::Output) -> io::Result<()> + Send + Sync + Clone + 'static,
    {
        let l = listener.clone();

        let unmarshalling_handler = move |msg: nats::Message| {
            let unmarshalled_data = l.unmarshaller.unmarshal(msg.data);
            l.handler.call((unmarshalled_data,))
        };
        let h = self
            .client
            .subscribe(&listener.subject)?
            .with_handler(unmarshalling_handler);
        Ok(h)
    }
}
