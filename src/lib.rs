mod event;
mod node;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nodes_can_be_constructed() {
        smol::block_on(async {
            node::Node::new().await;
        });
    }
}
