use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

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
            store(&args.format, &path, catalog, &output)?;
        }
        Commands::Validate { input } => {
            validate(&args.format, &input)?;
        }
    }
    Ok(())
}

fn store(
    format: &Format,
    path: &PathBuf,
    catalog_choice: CatalogChoice,
    output: &PathBuf,
) -> Result<()> {
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
    let mut writer = std::io::BufWriter::new(file);

    match format {
        Format::Json => serde_json::to_writer(writer, &store)?,
        Format::Bitcode => {
            let data = bitcode::serialize(&store)?;
            writer.write_all(&data)?;
        }
    }

    Ok(())
}

fn validate(format: &Format, input: &PathBuf) -> Result<()> {
    // Open the file specified in the args
    let file = std::fs::File::open(input)?;
    let reader = std::io::BufReader::new(file);

    // Deserialize the store file
    let _store: Store = match format {
        Format::Json => serde_json::from_reader(reader)?,
        Format::Bitcode => {
            let data = reader.bytes().collect::<Result<Vec<u8>, _>>()?;
            bitcode::deserialize(&data)?
        }
    };

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    /// The format of the file containing the serialized store
    #[arg(short, long, value_name = "FORMAT", default_value = "json")]
    pub format: Format,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Format {
    Json,
    Bitcode,
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
        catalog: CatalogChoice,

        /// The file to write the parsed results to
        #[arg(short, long, value_name = "FILE", default_value = "./results.dat")]
        output: std::path::PathBuf,
    },

    /// Validates a JSON dump of the refractiveindex.info database
    Validate {
        /// The path to the JSON dump of the refractiveindex.info database
        #[arg(short, long, value_name = "FILE", default_value = "./results.dat")]
        input: std::path::PathBuf,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum CatalogChoice {
    N2,
    NK,
}
