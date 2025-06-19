use proarc_server_v2::auth;

#[cfg(test)]
mod tdd
{
    use super::*;
    use std::env;
    use proarc_server_v2::auth::RequestType;
    use proarc_server_v2::load_env;

    fn setup() 
    {
        load_env();
    }

    fn clean() 
    {
        
    }

    struct TestGuard;

    impl Drop for TestGuard {
        fn drop(&mut self) {
            clean();
        }
    }

    #[test]
    fn validate_login_token()
    {
        setup();
        let _guard = TestGuard;

        let token = "nil";

        match auth::validate_token(token)
        {
            RequestType::AUTH => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn validate_user_token_sucess() 
    {
        setup();
        let _guard = TestGuard;

        let token = env::var("USER_TOKEN")
            .expect("USER_TOKEN environment variable must be set for this test");

        match auth::validate_token(&token)
        {
            RequestType::VALID => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn validate_token_fail() 
    {
        setup();
        let _guard = TestGuard;

        let token = "invalid_token";

        match auth::validate_token(&token)
        {
            RequestType::INVALID => assert!(true),
            _ => assert!(false)
        }
    }


    #[test]
    fn validate_adm_token_sucess() 
    {
        setup();
        let _guard = TestGuard;

        let token = env::var("ADM_TOKEN")
            .expect("USER_TOKEN environment variable must be set for this test");

        match auth::validate_token(&token)
        {
            RequestType::ADM => assert!(true),
            _ => assert!(false)
        }
    }
}

#[cfg(test)]
mod system
{
    use super::*;
    use std::env;
    use proarc_server_v2::auth::RequestType;
    use proarc_server_v2::load_env;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    fn setup() 
    {
        load_env();
    }

    fn clean() 
    {
        
    }

    struct TestGuard;

    impl Drop for TestGuard {
        fn drop(&mut self) {
            clean();
        }
    }

    #[tokio::test]
    async fn auth_token_fail()
    {
        setup();
        let _guard = TestGuard;

        let mut s = connect_to_server().await;

        s.write_all(b"notatoken").await.unwrap();
        let res = {
            let mut res = String::new();
            s.read_to_string(&mut res).await.unwrap();

            res
        };

        assert!(res.contains("NOT OK"), "Unexpected response: {res}");
    }

    #[tokio::test]
    async fn login_sucess()
    {
        setup();
        let _guard = TestGuard;

        let mut s = connect_to_server().await;

        assert!(send_token(&mut s, "nil").await, "Failed on TOKEN phase");
        assert!(send_pwd(&mut s).await, "Failed on PASSWORD phase");
    }

    async fn connect_to_server() -> TcpStream {
        let server_addr = env::var("SERVER_ADDR")
            .expect("SERVER_ADDR environment variable must be set for this test");

        let s = match TcpStream::connect(server_addr).await {
            Ok(stream) => stream,
            Err(_) => panic!("Remember to start the server to run system tests")
        };

        s
    }

    async fn send_token(s: &mut TcpStream, token: &str) -> bool
    {
        s.write_all(token.as_bytes()).await.unwrap();
        let r: String = {
            let mut str = String::new();
            s.read_to_string(&mut str).await.unwrap();

            str
        };

        "OK" == r
    }

    async fn send_pwd(s: &mut TcpStream) -> bool 
    {
        let pwd = env::var("PASSWORD")
            .expect("PASSWROD environment variable must be set for this test");

        s.write_all(pwd.as_bytes()).await.unwrap();
        let r: String = {
            let mut str = String::new();
            s.read_to_string(&mut str).await.unwrap();

            str
        };

        "OK" == r
    }
}