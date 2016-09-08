use gj::{Promise, EventLoop, TaskReaper, TaskSet};
use capnp::Error;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};

// use sandstorm::grain_capnp::{session_context, ui_view, ui_session, sandstorm_api};
// use sandstorm::identity_capnp::{user_info};
// use sandstorm::web_session_capnp::{web_session};
use backup_capnp::backup_capnp::backup;

pub struct Backup {}

impl backup::Server for Backup {
    fn have_full(&mut self,
                 _params: backup::HaveFullParams,
                 mut results: backup::HaveFullResults)
                 -> Promise<(), Error> {
        results.get().set_value(false);
        Promise::ok(())
    }
}

pub struct Reaper;

impl TaskReaper<(), ::capnp::Error> for Reaper {
    fn task_failed(&mut self, error: ::capnp::Error) {
        println!("Task failed: {}", error);
    }
}

pub fn accept_loop(listener: ::gjio::SocketListener,
                   mut task_set: TaskSet<(), ::capnp::Error>,
                   back: backup::Client)
                   -> Promise<(), ::std::io::Error> {
    listener.accept().then(move |stream| {
        let mut network = twoparty::VatNetwork::new(stream.clone(),
                                                    stream,
                                                    rpc_twoparty_capnp::Side::Server,
                                                    Default::default());
        let disconnect_promise = network.on_disconnect();

        let rpc_system = RpcSystem::new(Box::new(network), Some(back.clone().client));

        task_set.add(disconnect_promise.attach(rpc_system));
        accept_loop(listener, task_set, back)
    })
}
