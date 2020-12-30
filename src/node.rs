use crate::event::Event;
use crate::listener::Listener;
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
    pub fn publish(&self, e: Event) -> io::Result<()> {
        self.client.publish(&e.subject, e.data)
    }

    /// Subscribe using a listener
    pub fn subscribe<F>(&self, l: Listener<F>) -> io::Result<nats::subscription::Handler>
    where
        F: Fn(nats::Message) -> io::Result<()> + Send + 'static,
    {
        let h = self.client.subscribe(&l.subject)?.with_handler(l.handler);
        Ok(h)
    }
}
