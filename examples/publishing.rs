use node_rust::event::Event;
use node_rust::node::Node;
use std::string::String;

fn main() {
    let n = Node::new().unwrap();
    n.publish(Event {
        subject: String::from("test"),
        data: String::from("test").into_boxed_str().into_boxed_bytes(),
    })
    .unwrap();
}
