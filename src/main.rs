#[macro_use]
extern crate log;
mod cli;
mod lib;

use cli::{Argon2Type, Cli};

use std::process::exit;
use color_eyre::eyre::{Result, eyre};
use color_eyre::owo_colors::OwoColorize;
use clap::Parser;


fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    env_logger::init();
    debug!("starting up");
    let app = Cli::parse();

    if app.oh() && app.op(){
        return Err(eyre!("You can't use --oh and --op together. Type -h for help".red()))
    }

    let algo=match app.ty {
        Argon2Type::Argon2d => lib::Argon2Type::Argon2d,
        Argon2Type::Argon2i => lib::Argon2Type::Argon2i,
        Argon2Type::Argon2id => lib::Argon2Type::Argon2id,
    };

    // Create PasswordHashing with default values.
    let mut config = lib::PasswordHashing::new();
    // Modify this values with user inputs
    config.password=app.password();
    config.iterations=app.iteration();
    config.salt=app.salt();
    config.length=app.length();
    config.parallel=app.parallel();
    config.memory=app.memory();
    config.variant=algo;
    let test=config.start()?;

    if app.oh(){
        println!("{}",test.0);
        exit(0)
    }
    if app.op(){
        println!("{}",test.1);
        exit(0)
    }

    println!("Password   : {}", app.password());
    println!("Salt       : {} (in Base64 : {})",app.salt(), test.3);
    println!("Memory: {} KiB, Iterations: {}, Parallelism: {}, Hash length: {}, Algo: {:?})",app.memory() ,app.iteration(), app.parallel(),app.length(), algo);
    println!("===================================");
    println!("Hex hash   : {}", test.1);
    println!("PHC String : {}", test.0.green());
    println!("Execution Time : {} {}", test.4,"s");

    Ok(())
}
