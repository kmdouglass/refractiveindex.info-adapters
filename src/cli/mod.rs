use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    path: std::path::PathBuf,
}
