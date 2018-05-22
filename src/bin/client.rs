extern crate grpc;
extern crate raft;

use raft::greeter::*;
use raft::greeter_grpc::*;

fn main() {
    let port = 50051;
    let client_conf = Default::default();

    let client = GreeterClient::new_plain("127.0.0.1", port, client_conf).unwrap();

    loop {
        let opts = grpc::RequestOptions::new();
        let mut req = HelloRequest::new();
        req.set_name("my name".to_string());
        let resp = client.say_hello(opts, req);
        println!("{:?}", resp.wait());
    }
}
