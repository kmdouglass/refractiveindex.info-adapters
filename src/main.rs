use clap::Parser;

use lib_ria::cli::Args;

#[derive(Debug)]
struct RiaError {
    message: String,
}

impl std::fmt::Display for RiaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RiaError {}

impl From<std::io::Error> for RiaError {
    fn from(error: std::io::Error) -> Self {
        RiaError {
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), RiaError> {
    let args = Args::parse();

    println!("{:?}", args);

    // Open the file specified in the args
    let file = std::fs::File::open(&args.path)?;
    let reader = std::io::BufReader::new(file);

    Ok(())
}
