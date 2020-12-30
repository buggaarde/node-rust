use node_rust::listener::Listener;
use node_rust::node::Node;

fn main() {
    let n = Node::new().unwrap();
    let l = Listener::new("test".to_string(), |msg| {
        println!("{}", &msg);
        Ok(())
    });
    let h = n.subscribe(l).unwrap();
    h.unsubscribe().unwrap();
}
