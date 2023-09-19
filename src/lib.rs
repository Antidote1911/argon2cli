use std::fmt::Debug;
use std::time::Instant;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Algorithm, Argon2, ParamsBuilder, Version,
};
use color_eyre::eyre::{eyre, Result};

#[derive(Debug)]
pub struct PasswordHashing {
    pub password: String,
    pub salt: String,
    pub length: usize,
    pub parallel: u32,
    pub iterations: u32,
    pub memory: u32,
    pub variant: Argon2Type,
}

impl Default for PasswordHashing {
    /// A set of recommended settings.
    fn default() -> Self {
        Self {
            password: String::from("password"),
            salt: String::from("somesalt"),
            length: 24,
            parallel: 4,
            iterations: 2,
            memory: 65536,
            variant: Argon2Type::Argon2i,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Argon2Type {
    Argon2i,
    Argon2d,
    Argon2id,
}

impl PasswordHashing {
    // Create a new configuration with default values.
    pub fn new() -> Self {
        PasswordHashing::default()
    }

    // return String PHC formated
    pub fn start(&mut self) -> Result<(String, String, String, String, String)> {
        let algo = match self.variant {
            Argon2Type::Argon2d => Algorithm::Argon2d,
            Argon2Type::Argon2i => Algorithm::Argon2i,
            Argon2Type::Argon2id => Algorithm::Argon2id,
        };

        let params = ParamsBuilder::new()
            .m_cost(self.memory)
            .t_cost(self.iterations)
            .p_cost(self.parallel)
            .output_len(self.length)
            .build()
            .unwrap();

        let argon2 = Argon2::new(algo, Version::V0x13, params);
        let password = self.password.as_str();
        let temp = self.salt.as_ref();

        let saltstring = SaltString::encode_b64(temp).unwrap();

        let start = Instant::now();
        let phc_string = argon2
            .hash_password(password.as_bytes(), &saltstring)
            .map_err(|e| eyre!(e))?;
        let duration = start.elapsed();
        let executiontime = duration.as_secs_f64().to_string();

        let key_hash_b64 = phc_string.hash.unwrap();
        let key_hash_bytes = key_hash_b64.as_bytes();

        Ok((
            phc_string.to_string(),
            hex::encode(key_hash_bytes),
            key_hash_b64.to_string(),
            saltstring.as_ref().to_string(),
            executiontime,
        ))
    }
}
