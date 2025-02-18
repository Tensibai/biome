use crate::error::Result;
use clap::Parser;

use biome_butterfly::rumor::{dat_file,
                               Departure,
                               Election,
                               ElectionUpdate,
                               Service,
                               ServiceConfig,
                               ServiceFile};
use log::error;
use std::{path::PathBuf,
          process};

mod error;

#[derive(Debug, Clone, Parser)]
#[command(name = "Biome Rst Reader",
          about = "Introspection for the butterfly RST file.",
          arg_required_else_help = true,
          help_template = "{name}\n{about-section}\n{usage-heading} {usage}\n\n{all-args}")]
struct RstReader {
    #[arg(name = "FILE", help = "Path to the RST file.")]
    file: String,

    #[arg(name = "STATS",
          short = 's',
          long = "stats",
          help = "Display statistics about the contents of the file.")]
    stats: bool,
}

fn main() -> error::Result<()> {
    env_logger::init();

    let rst_reader = RstReader::parse();

    let dat_file =
        dat_file::DatFileReader::read(PathBuf::from(&rst_reader.file)).unwrap_or_else(|e| {
                                                                          error!("Could not read \
                                                                                  dat file {}: {}",
                                                                                 &rst_reader.file,
                                                                                 e);
                                                                          process::exit(1);
                                                                      });

    let result = if rst_reader.stats {
        output_stats(dat_file)
    } else {
        output_rumors(dat_file)
    };

    if result.is_err() {
        error!("Error processing dat file: {:?}", result);
        process::exit(1);
    }

    Ok(())
}

fn output_rumors(mut dat_file: dat_file::DatFileReader) -> Result<()> {
    for member in dat_file.read_members()? {
        println!("{}", member);
    }

    for service in dat_file.read_rumors::<Service>()? {
        println!("{}", service);
    }

    for service_config in dat_file.read_rumors::<ServiceConfig>()? {
        println!("{}", service_config);
    }

    for service_file in dat_file.read_rumors::<ServiceFile>()? {
        println!("{}", service_file);
    }

    for election in dat_file.read_rumors::<Election>()? {
        println!("{}", election);
    }

    for update_election in dat_file.read_rumors::<ElectionUpdate>()? {
        println!("{}", update_election);
    }

    for departure in dat_file.read_rumors::<Departure>()? {
        println!("{}", departure);
    }

    Ok(())
}

fn output_stats(mut dat_file: dat_file::DatFileReader) -> Result<()> {
    let mut membership = 0;
    let mut services = 0;
    let mut service_configs = 0;
    let mut service_files = 0;
    let mut elections = 0;
    let mut update_elections = 0;
    let mut departures = 0;

    membership += dat_file.read_members()?.len();
    services += dat_file.read_rumors::<Service>()?.len();
    service_configs += dat_file.read_rumors::<ServiceConfig>()?.len();
    service_files += dat_file.read_rumors::<ServiceFile>()?.len();
    elections += dat_file.read_rumors::<Election>()?.len();
    update_elections += dat_file.read_rumors::<ElectionUpdate>()?.len();
    departures += dat_file.read_rumors::<Departure>()?.len();

    println!("Summary:");
    println!();
    println!("Membership: {}", membership);
    println!("Services: {}", services);
    println!("Service Configs: {}", service_configs);
    println!("Service Files: {}", service_files);
    println!("Elections: {}", elections);
    println!("Update Elections: {}", update_elections);
    println!("Departures: {}", departures);

    Ok(())
}
