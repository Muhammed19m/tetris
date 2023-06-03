#[cfg(test)]
mod client_test {

    use crate::client;

    const ADDR: u16 = 8080;

    #[allow(unused)]
    #[test]
    fn test_cli() {
        dbg!("test started");
        let (cli, sender, recveiver) =
            client::Client::new(&format!("ws://127.0.0.1:{ADDR}/Ply1")).unwrap();
    }
}
