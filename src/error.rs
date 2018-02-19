#![allow(unknown_lints)]
#![allow(missing_docs)]

use std::io;

use crossbeam_channel;
use futures;
use serde_json;
use ws;

use client::WorkerRequest;

error_chain! {
  foreign_links {
    Crossbeam(crossbeam_channel::RecvError);
    Io(io::Error);
    Futurs(futures::Canceled);
    Json(serde_json::Error);
    Mpsc(futures::sync::mpsc::SendError<ws::Message>);
    SocketWorker(futures::sync::mpsc::SendError<WorkerRequest>);
    Ws(ws::Error);
  }
  errors {
    RpcError(e: String) {
        description("rpc errpr"),
        display("RPC error: {}", e)
    }
    YumError(e: String) {
        description("ethereyum error"),
        display("EthereYUM: {}", e)
    }
  }
}

