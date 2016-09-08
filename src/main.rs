#[macro_use]
extern crate tarpc;

service! {
    rpc has_full(name: String) -> bool;
}

#[derive(Clone)]
struct BackupServer;

impl Service for BackupServer {
    fn has_full(&self, name: String) -> bool {
        false
    }
}

fn main() {
    let addr = "localhost:10000";
    let server = BackupServer.spawn(addr).unwrap();
    let client = Client::new(addr).unwrap();
    println!("has full: {}", client.has_full("Mom".to_string()).unwrap());

    drop(client);
    server.shutdown();
}
