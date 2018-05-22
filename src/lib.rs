extern crate futures;
extern crate grpc;
extern crate grpcio;
extern crate protobuf;
extern crate tls_api;

pub mod greeter;
pub mod greeter_grpc;
pub mod pingcap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
