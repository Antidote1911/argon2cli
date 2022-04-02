use clap::{ArgEnum, Parser,AppSettings};

const AUTHOR: &str = "
Author : Fabrice Corraire <antidote1911@gmail.com>
Github : https://github.com/Antidote1911/
";

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(about, author=AUTHOR, version)]
pub struct Cli {

    /// The password to hash
    #[clap(short, long, default_value_t = String::from("password"))]
    pub password: String,

    /// The salt to use
    #[clap(short, long, default_value_t = String::from("somesalt"))]
    pub salt: String,

    /// Output length
    #[clap(long, default_value = "24")]
    pub length: usize,

    /// Nuber of core to use
    #[clap(long, default_value = "4")]
    pub parallel: u32,

    /// Number of passes
    #[clap(long, default_value = "2")]
    pub passes: u32,

    /// Memory in Kib
    #[clap(long, default_value = "65536")]
    pub megabytes: u32,

    /// Algo variant to use
    #[clap(short, long, arg_enum, name = "TYPE", default_value = "argon2i")]
    pub ty: Argon2Type,

    /// Flag to only output hex hash
    #[clap(long)]
    pub oh: bool,

    /// Flag to only output Phc formated hash
    #[clap(long)]
    pub op: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Argon2Type {
    Argon2i,
    Argon2d,
    Argon2id,
}
