use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    pub path: std::path::PathBuf,
}
