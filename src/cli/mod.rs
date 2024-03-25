use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    /// The path to the refractiveindex.info database folders
    #[arg(short, long, value_name = "PATH", default_value = "./database")]
    pub path: std::path::PathBuf,

    /// The catalog to parse
    #[arg(short, long, value_name = "TYPE", default_value = "nk")]
    pub catalog: Catalog,

    /// The file to write the parsed results to
    #[arg(short, long, value_name = "FILE", default_value = "./results.json")]
    pub output: std::path::PathBuf,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Catalog {
    N2,
    NK,
}
