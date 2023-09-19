use clap::{Parser, ValueEnum};

const AUTHOR: &str = "
Author : Fabrice Corraire <antidote1911@gmail.com>
Github : https://github.com/Antidote1911/
";

#[derive(Parser)]
#[clap(about, author=AUTHOR, version)]
pub struct Cli {
    /// The password to hash
    #[clap(long, default_value_t = String::from("password"))]
    pass: String,

    /// The salt to use
    #[clap(long, default_value_t = String::from("somesalt"))]
    salt: String,

    /// Hash output length
    #[clap(short, long, default_value = "24")]
    length: usize,

    /// Number of core to use
    #[clap(short, long, default_value = "4")]
    parallel: u32,

    /// Number of passes
    #[clap(short, long, default_value = "2")]
    iteration: u32,

    /// Memory in KiB. (65536 KiB = 64 MiB * 1024)
    #[clap(short, long, default_value = "65536")]
    memory: u32,

    /// Algo variant to use
    #[clap(short, long, value_enum, name = "TYPE", default_value = "argon2i")]
    pub ty: Argon2Type,

    /// Only output hex hash
    #[clap(long)]
    oh: bool,

    /// Only output Phc formated hash
    #[clap(long)]
    op: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Argon2Type {
    Argon2i,
    Argon2d,
    Argon2id,
}
impl Cli {
    pub fn password(&self) -> String {
        self.pass.to_string()
    }
    pub fn salt(&self) -> String {
        self.salt.to_string()
    }
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn parallel(&self) -> u32 {
        self.parallel
    }
    pub fn iteration(&self) -> u32 {
        self.iteration
    }
    pub fn memory(&self) -> u32 {
        self.memory
    }
    pub fn oh(&self) -> bool {
        self.oh
    }
    pub fn op(&self) -> bool {
        self.op
    }
}
