extern crate argon2cli;
mod tests {

    #[test]
    fn default_config() {
        let mut config = argon2cli::PasswordHashing::new();
        let out = config.start().unwrap();
        assert_eq!(
            "$argon2i$v=19$m=65536,t=2,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG",
            out.0
        );
    }

    #[test]
    fn default_with_argon2id() {
        let mut config = argon2cli::PasswordHashing::new();
        config.variant = argon2cli::Argon2Type::Argon2id;
        let out = config.start().unwrap();
        assert_eq!(
            "$argon2id$v=19$m=65536,t=2,p=4$c29tZXNhbHQ$F1jG2CV3/Nr+yRuIsPKw0J9r4s7cJHBU",
            out.0
        );
    }

    #[test]
    fn custom_config_1() {
        let mut config = argon2cli::PasswordHashing::new();
        config.variant = argon2cli::Argon2Type::Argon2id;
        config.password = "pass".to_string();
        config.salt = "313233343536373839616263646566".to_string();
        config.length = 32;
        config.parallel = 1;
        config.iterations = 1;
        config.memory = 8;
        let out = config.start().unwrap();
        assert_eq!("$argon2id$v=19$m=8,t=1,p=1$MzEzMjMzMzQzNTM2MzczODM5NjE2MjYzNjQ2NTY2$Y868kb3rSmYL1HxuA7HXq/tqGji8ZV5MnDyWMZX/5aE", out.0);
    }
}
