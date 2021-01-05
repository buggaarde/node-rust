use node_rust::event::Event;
use node_rust::listener::Listener;
use node_rust::marshal::{Marshalling, Unmarshalling};
use node_rust::node::Node;

fn main() {
    let n = Node::new().unwrap();
    let l = Listener::new("test", Unmarshalling::None, |msg| {
        if let Ok(s) = std::str::from_utf8(&msg) {
            println!("{}", s);
        }
        Ok(())
    });
    let h = n.subscribe(l).unwrap();

    let m = Node::new().unwrap();
    m.publish(Event::new("test", Marshalling::None, "test".into()))
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1));

    h.unsubscribe().unwrap();
}
