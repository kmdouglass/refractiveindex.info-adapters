use clap::Parser;

use lib_ria::cli::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
