use anyhow::Result;

use clap::Parser;

use lib_ria::cli::{Args, Catalog as CatalogChoice};
use lib_ria::database::Catalog;
use lib_ria::Store;

fn main() -> Result<()> {
    let args = Args::parse();

    // Save the current directory
    let current_dir = std::env::current_dir()?;

    // Change the current directory to the database path
    std::env::set_current_dir(&args.path)?;

    // Open the file specified in the args
    let file = match &args.catalog {
        CatalogChoice::N2 => std::fs::File::open("catalog-n2.yml")?,
        CatalogChoice::NK => std::fs::File::open("catalog-nk.yml")?,
    };
    let reader = std::io::BufReader::new(file);

    // Deserialize the catalog YAML file
    let catalog: Catalog = serde_yaml::from_reader(reader)?;

    // Parse the catalog into this library's internal representation
    match &args.catalog {
        CatalogChoice::N2 => {
            std::env::set_current_dir("data-n2")?;
        }
        CatalogChoice::NK => {
            std::env::set_current_dir("data-nk")?;
        }
    }
    let store = Store::try_from(catalog)?;

    // Write the store to the output file in JSON format
    std::env::set_current_dir(current_dir)?;
    let file = std::fs::File::create(&args.output)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, &store)?;

    Ok(())
}
