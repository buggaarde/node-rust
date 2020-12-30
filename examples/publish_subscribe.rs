use node_rust::event::Event;
use node_rust::listener::Listener;
use node_rust::node::Node;
use std::string::String;

fn main() {
    let n = Node::new().unwrap();
    let l = Listener::new("test".to_string(), |msg| {
        println!("{}", &msg);
        Ok(())
    });
    let h = n.subscribe(l).unwrap();

    let m = Node::new().unwrap();
    m.publish(Event {
        subject: String::from("test"),
        data: String::from("test").into_boxed_str().into_boxed_bytes(),
    })
    .unwrap();

    h.unsubscribe().unwrap();
}
