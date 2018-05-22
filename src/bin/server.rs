extern crate grpc;
extern crate httpbis;
extern crate raft;

use raft::greeter::*;
use raft::greeter_grpc::*;
use std::thread;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(
        &self,
        _m: grpc::RequestOptions,
        req: HelloRequest,
    ) -> grpc::SingleResponse<HelloReply> {
        let mut r = HelloReply::new();
        let name = if req.get_name().is_empty() {
            "world"
        } else {
            req.get_name()
        };
        r.set_message(format!("Hello {}", name));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let port = 50051;

    let mut conf = httpbis::ServerConf::default();
    conf.reuse_port = Some(true);

    let mut servers = vec![];
    for _ in 1..=8 {
        let mut server = grpc::ServerBuilder::new_plain();
        server.http.conf = conf.clone();
        server.http.set_addr("127.0.0.1:50051").unwrap();
        // server.http.set_port(port);
        server.add_service(GreeterServer::new_service_def(GreeterImpl));
        // server.http.set_cpu_pool_threads(8);
        let _server = server.build().expect("server");
        servers.push(_server);
    }

    println!("greeter server started on port {} {}", port, "without tls");

    loop {
        thread::park();
    }
}
