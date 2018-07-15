extern crate rpc;

#[cfg(test)]
mod tests {
    use rpc::rpc_server::start_http;
    #[test]
    fn it_works() {
        start_http();
    }
}
