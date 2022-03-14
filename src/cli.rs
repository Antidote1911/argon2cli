use clap::{ArgEnum, Parser};

const AUTHOR: &str = "
Author : Fabrice Corraire <antidote1911@gmail.com>
Github : https://github.com/Antidote1911/
";

#[derive(Parser)]
#[clap(about, author=AUTHOR, version)]
pub struct Cli {
    #[clap(short, long, default_value_t = String::from("password"))]
    pub password: String,

    #[clap(short, long, default_value_t = String::from("somesalt"))]
    pub salt: String,

    #[clap(long, default_value = "24")]
    pub length: usize,

    #[clap(long, default_value = "4")]
    pub parallel: u32,

    #[clap(long, default_value = "2")]
    pub passes: u32,

    #[clap(long, default_value = "64")]
    pub megabytes: u32,

    #[clap(short, long, arg_enum, name = "TYPE", default_value = "argon2i")]
    pub ty: Argon2Type,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Argon2Type {
    Argon2i,
    Argon2d,
    Argon2id,
}
