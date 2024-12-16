use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a single file store of the refractiveindex.info database
    Store {
        /// The path to the refractiveindex.info database folders
        #[arg(short, long, value_name = "PATH", default_value = "./database")]
        path: std::path::PathBuf,

        /// The catalog to parse
        #[arg(short, long, value_name = "TYPE", default_value = "nk")]
        catalog: Catalog,

        /// The file to write the parsed results to
        #[arg(short, long, value_name = "FILE", default_value = "./results.json")]
        output: std::path::PathBuf,
    },

    /// Validates a JSON dump of the refractiveindex.info database
    Validate {
        /// The path to the JSON dump of the refractiveindex.info database
        #[arg(short, long, value_name = "FILE", default_value = "./results.json")]
        input: std::path::PathBuf,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Catalog {
    N2,
    NK,
}
