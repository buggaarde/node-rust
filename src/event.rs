pub trait Event {
    fn subject(&self) -> &'static str;
    fn marshal(&self) -> &[u8];
}

pub trait Listener<M, P> {
    fn subject(&self) -> &'static str;
    fn unmarshal(&self, msg: M) -> P;
    fn handler(&self, payload: P);
}
