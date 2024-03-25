use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    /// A refractiveindex.info catalog file
    #[arg(short, long, value_name = "FILE", default_value = "./catalog-nk.yml")]
    pub input: std::path::PathBuf,

    /// The file to write the parsed results to
    #[arg(short, long, value_name = "FILE", default_value = "./results.json")]
    pub output: std::path::PathBuf,
}
