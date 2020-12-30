use std::io;

/// Listeners wrap subjects and handlers in a single struct
#[derive(Clone)]
pub struct Listener<F>
where
    F: Fn(Vec<u8>) -> io::Result<()> + Send + Sync + Clone + 'static,
{
    pub(crate) subject: String,
    pub(crate) handler: F,
}

impl<F> Listener<F>
where
    F: Fn(Vec<u8>) -> io::Result<()> + Send + Sync + Clone + 'static,
{
    /// Create a new listener
    pub fn new(subject: String, handler: F) -> Listener<F> {
        Listener {
            subject: subject,
            handler: handler,
        }
    }
}
