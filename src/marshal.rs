pub enum Marshalling {
    None,
    Protobuffer,
}

#[derive(Clone)]
pub enum Unmarshalling {
    None,
}

pub trait Marshal {
    fn marshal(&self, data: impl AsRef<[u8]>) -> Vec<u8>;
}

pub trait Unmarshal {
    type Output;
    fn unmarshal(&self, data: impl AsRef<[u8]>) -> Self::Output;
}

impl Marshal for Marshalling {
    fn marshal(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        match self {
            _ => data.as_ref().into(),
        }
    }
}

impl Unmarshal for Unmarshalling {
    type Output = Vec<u8>;
    fn unmarshal(&self, data: impl AsRef<[u8]>) -> Self::Output {
        data.as_ref().into()
    }
}
