use anyhow::Result;

use clap::Parser;

use lib_ria::cli::Args;
use lib_ria::database::Catalog;
use lib_ria::Store;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{:?}", args);

    // Open the file specified in the args
    let file = std::fs::File::open(&args.input)?;
    let reader = std::io::BufReader::new(file);

    // Parse the YAML file into the database's internal representation
    let catalog: Catalog = serde_yaml::from_reader(reader)?;
    let store = Store::try_from(catalog)?;

    // Write the store to the output file
    // TODO: Implement this

    Ok(())
}
