use node_rust::event::Event;
use node_rust::listener::Listener;
use node_rust::marshal::{Marshalling, Unmarshalling};
use node_rust::node::Node;
use std::string::String;

fn main() {
    let n = Node::new(Marshalling::None, Unmarshalling::None).unwrap();
    let l = Listener::new("test".to_string(), |msg| {
        if let Ok(s) = std::str::from_utf8(&msg) {
            println!("{}", s);
        }
        Ok(())
    });
    let h = n.subscribe(l).unwrap();

    let m = Node::new(Marshalling::None, Unmarshalling::None).unwrap();
    m.publish(Event {
        subject: String::from("test"),
        data: "test".into(),
    })
    .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1));

    h.unsubscribe().unwrap();
}
