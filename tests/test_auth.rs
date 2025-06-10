use proarc_server_v2::auth;

#[cfg(test)]
mod tests_token 
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