extern crate logs;
extern crate log;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _init = logs::init("hello", logs::DateLogFormatter);
        println!("{:?}", _init);
    }
}
