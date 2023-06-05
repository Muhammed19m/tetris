#[cfg(test)]
mod client_test {

    use crate::client;

    const ADDR: u16 = 8080;

    #[allow(unused)]
    #[test]
    fn test_cli() {
        dbg!("test started");

        let (cli1, sender1, recveiver1) =
            client::Client::new(&format!("ws://127.0.0.1:{ADDR}/Ply1")).unwrap();

        let (cli2, sender2, recveiver2) =
            client::Client::new(&format!("ws://127.0.0.1:{ADDR}/Ply2")).unwrap();

        assert_eq!(sender1.send(vec![1, 0, 1, 0, 0]), Ok(()));
        assert_eq!(recveiver2.recv(), Ok(vec![1, 0, 1, 0, 0]));

        assert_eq!(sender2.send(vec![1, 0, 1, 0, 0, 1, 2]), Ok(()));
        assert_eq!(recveiver1.recv(), Ok(vec![1, 0, 1, 0, 0, 1, 2]));
    }
}
