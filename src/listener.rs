use nats;
use std::io;

/// Listeners wrap subjects and handlers in a single struct
pub struct Listener<F>
where
    F: Fn(nats::Message) -> io::Result<()> + Send + 'static,
{
    pub(crate) subject: String,
    pub(crate) handler: F,
}

impl<F> Listener<F>
where
    F: Fn(nats::Message) -> io::Result<()> + Send + 'static,
{
    /// Create a new listener
    pub fn new(subject: String, handler: F) -> Listener<F> {
        Listener {
            subject: subject,
            handler: handler,
        }
    }
}
