use crate::marshal::Unmarshal;
use std::io;

/// Listeners wrap subjects and handlers in a single struct
#[derive(Clone)]
pub struct Listener<'a, U, F>
where
    U: Unmarshal + Clone + Send + Sync + 'static,
    F: Fn(U::Output) -> io::Result<()> + Send + Sync + Clone + 'static,
{
    pub(crate) subject: &'a str,
    pub(crate) unmarshaller: U,
    pub(crate) handler: F,
}

impl<'a, U, F> Listener<'a, U, F>
where
    U: Unmarshal + Clone + Send + Sync + 'static,
    F: Fn(U::Output) -> io::Result<()> + Send + Sync + Clone + 'static,
{
    /// Create a new listener
    pub fn new(subject: &'a str, unmarshaller: U, handler: F) -> Listener<'a, U, F> {
        Listener {
            subject,
            unmarshaller,
            handler,
        }
    }

    pub fn unmarshalling_handler(
        &self,
    ) -> Box<dyn Fn(nats::Message) -> io::Result<()> + Send + 'static> {
        let unmarshaller = self.unmarshaller.clone();
        let handler = self.handler.clone();
        Box::new(move |msg: nats::Message| {
            let data = unmarshaller.unmarshal(msg.data);
            handler.call((data,))
        })
    }
}
