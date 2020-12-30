pub enum Marshalling {
    None,
    Protobuffer,
}

#[derive(Clone)]
pub enum Unmarshalling {
    None,
    Protobuffer,
}

pub trait Marshal {
    fn marshal(&self, data: impl AsRef<[u8]>) -> Vec<u8>;
}

pub trait Unmarshal {
    fn unmarshal(&self, data: impl AsRef<[u8]>) -> Vec<u8>;
}

impl Marshal for Marshalling {
    fn marshal(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        match self {
            _ => data.as_ref().into(),
        }
    }
}

impl Unmarshal for Unmarshalling {
    fn unmarshal(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        match self {
            _ => data.as_ref().into(),
        }
    }
}
