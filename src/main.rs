use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;

use lib_ria::cli::{Args, Catalog as CatalogChoice, Commands};
use lib_ria::database::Catalog;
use lib_ria::Store;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Store {
            path,
            catalog,
            output,
        } => {
            store(&path, catalog, &output)?;
        }
        Commands::Validate { input } => {
            validate(&input)?;
        }
    }
    Ok(())
}

fn store(path: &PathBuf, catalog_choice: CatalogChoice, output: &PathBuf) -> Result<()> {
    // Save the current directory
    let current_dir = std::env::current_dir()?;

    // Change the current directory to the database path
    println!("Changing directory to {}", path.display());
    std::env::set_current_dir(path)?;

    // Open the file specified in the args
    let file = match catalog_choice {
        CatalogChoice::N2 => std::fs::File::open("catalog-n2.yml")?,
        CatalogChoice::NK => std::fs::File::open("catalog-nk.yml")?,
    };
    let reader = std::io::BufReader::new(file);

    // Deserialize the catalog YAML file
    let catalog: Catalog = serde_yaml::from_reader(reader)?;

    // Parse the catalog into this library's internal representation
    match catalog_choice {
        CatalogChoice::N2 => {
            std::env::set_current_dir("data-n2")?;
        }
        CatalogChoice::NK => {
            std::env::set_current_dir("data-nk")?;
        }
    }
    let store = Store::try_from(catalog)?;

    // Write the store to the output file in JSON format
    println!("Changing directory back to {}", &current_dir.display());
    std::env::set_current_dir(current_dir)?;
    let file = std::fs::File::create(output)?;

    println!("Writing store to {}", output.display());
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, &store)?;

    Ok(())
}

fn validate(input: &PathBuf) -> Result<()> {
    // Open the file specified in the args
    let file = std::fs::File::open(input)?;
    let reader = std::io::BufReader::new(file);

    // Deserialize the store JSON file
    let _store: Store = serde_json::from_reader(reader)?;

    Ok(())
}
