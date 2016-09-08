#[macro_use]
extern crate gj;
extern crate gjio;
extern crate capnp;
extern crate capnp_rpc;
extern crate backup_capnp;

use backup_capnp::backup_capnp::backup;
use gj::{Promise, EventLoop, TaskReaper, TaskSet};

pub mod server;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return;
    }

    EventLoop::top_level(move |wait_scope| -> Result<(), ::capnp::Error> {
            use std::net::ToSocketAddrs;
            let mut event_port = try!(::gjio::EventPort::new());
            let network = event_port.get_network();
            let addr = try!(args[2].to_socket_addrs()).next().expect("could not parse address");
            let mut address = network.get_tcp_address(addr);
            let listener = try!(address.listen());

            let back = backup::ToClient::new(server::Backup {})
                .from_server::<::capnp_rpc::Server>();

            let task_set = TaskSet::new(Box::new(server::Reaper));
            try!(server::accept_loop(listener, task_set, back).wait(wait_scope, &mut event_port));

            Ok(())
        })
        .expect("top level error");
}
