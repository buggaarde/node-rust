use crate::event;
use async_trait::async_trait;
use std::io;
use std::marker::{PhantomData, Send};

#[async_trait]
pub trait Subscription<Message> {
    async fn next(&self) -> Option<Message>;
    async fn drain(&self) -> io::Result<()>;
    async fn unsubscribe(&self) -> io::Result<()>;
}

#[async_trait]
impl Subscription<async_nats::Message> for async_nats::Subscription {
    async fn next(&self) -> Option<async_nats::Message> {
        self.next().await
    }
    async fn drain(&self) -> io::Result<()> {
        self.drain().await
    }
    async fn unsubscribe(&self) -> io::Result<()> {
        self.unsubscribe().await
    }
}

#[async_trait]
pub trait Connection<Message, S: Subscription<Message>> {
    async fn publish(
        &self,
        subject: &str,
        msg: impl AsRef<[u8]> + Send + 'async_trait, // Send + 'async_trait are required due to #[async_trait] constraints
    ) -> io::Result<()>;
    async fn subscribe(&self, subject: &str) -> io::Result<S>;
    async fn close(&self) -> io::Result<()>;
}

#[async_trait]
impl Connection<async_nats::Message, async_nats::Subscription> for async_nats::Connection {
    async fn publish(
        &self,
        subject: &str,
        msg: impl AsRef<[u8]> + Send + 'async_trait,
    ) -> io::Result<()> {
        self.publish(subject, msg).await
    }
    async fn subscribe(&self, subject: &str) -> io::Result<async_nats::Subscription> {
        self.subscribe(subject).await
    }
    async fn close(&self) -> io::Result<()> {
        self.close().await
    }
}

pub struct Node<Message, Sub, Conn>
where
    Conn: Connection<Message, Sub>,
    Sub: Subscription<Message>,
{
    connection: Conn,
    sub: PhantomData<Sub>,
    msg: PhantomData<Message>,
}

impl Node<async_nats::Message, async_nats::Subscription, async_nats::Connection> {
    pub async fn new() -> Self {
        let conn = async_nats::connect("127.0.0.1").await.unwrap();
        Node {
            connection: conn,
            sub: PhantomData,
            msg: PhantomData,
        }
    }

    pub async fn publish<E: event::Event>(&self, event: E) -> io::Result<()> {
        self.connection
            .publish(event.subject(), event.marshal())
            .await
    }
    pub async fn listen<L>(&self, listener: &L) -> io::Result<()>
    where
        L: event::Listener<async_nats::Message, String>,
    {
        let subscription = self.connection.subscribe(listener.subject()).await?;
        while let Some(msg) = subscription.next().await {
            listener.handler(listener.unmarshal(msg));
        }
        Ok(())
    }
}
