use crate::marshal;

pub struct Event<'a, M: marshal::Marshal> {
    pub subject: &'a str,
    pub(crate) marshaller: M,
    pub data: Vec<u8>,
}

impl<'a, M: marshal::Marshal> Event<'a, M> {
    pub fn new(subject: &'a str, marshaller: M, data: Vec<u8>) -> Event<M> {
        Event {
            subject,
            marshaller,
            data,
        }
    }
    pub fn marshalled_data(&self) -> Vec<u8> {
        self.marshaller.marshal(&self.data)
    }
}
