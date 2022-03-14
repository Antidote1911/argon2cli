mod cli;
use argon2::{password_hash::{
    PasswordHasher, SaltString,
}, Algorithm, Argon2, ParamsBuilder, Version};
use clap::StructOpt;
use cli::{Argon2Type, Cli};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = Cli::parse();

    let mut pb = ParamsBuilder::new();
    pb.m_cost(app.megabytes*1024).map_err(|e| e.to_string())?;
    pb.t_cost(app.passes).map_err(|e| e.to_string())?;
    pb.p_cost(app.parallel).map_err(|e| e.to_string())?;
    pb.output_len(app.length).map_err(|e| e.to_string())?;

    let params = pb.params().map_err(|e| e.to_string())?;

    let algo=match app.ty {
        Argon2Type::Argon2d => Algorithm::Argon2d,
        Argon2Type::Argon2i => Algorithm::Argon2i,
        Argon2Type::Argon2id => Algorithm::Argon2id,
    };

    let argon2 = Argon2::new(algo, Version::V0x13, params);

    let password = app.password.as_str();
    let temp     = app.salt.as_ref();
    let saltstring = SaltString::b64_encode(temp).map_err(|e| e.to_string())?;

    let phc_string = argon2.hash_password(password.as_bytes(), saltstring.as_ref()).map_err(|e| e.to_string())?;
    let key_hash_b64 = phc_string.hash.unwrap();
    let key_hash_bytes = key_hash_b64.as_bytes();

    println!("Password   : {}",password);
    println!("Salt       : {} (in Base64 : {})",app.salt, saltstring.as_ref());
    println!("B64 Hash   : {}", key_hash_b64); // or: base64::encode(&key_vec)
    println!("Hex Hash   : {}",(hex::encode(key_hash_bytes)));
    println!("Hash length : {}",app.length);
    println!("PHC String : {}", phc_string);
    Ok(())
}
