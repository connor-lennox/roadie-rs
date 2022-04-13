use std::{env, path::PathBuf};

use serde::{Serialize, Deserialize};
use clap::{arg, command, Command};

#[derive(Debug, Serialize, Deserialize)]
struct SampleSet {
    name: String,
    samples: [String; 8],
}

fn get_presets_path() -> PathBuf {
    [
        env::current_dir().expect("could not find presets file").to_str().expect("could not find presets file"),
        "res",
        "presets.ron"
    ].iter().collect()
}

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("create")
                .about("Creates a sample set and pushes to SD card"),
        )
        .subcommand(
            Command::new("preset")
                .subcommand(
                    Command::new("list")
                        .about("List available presets")
                )
                .subcommand(
                    Command::new("info")
                        .about("Display info about a preset")
                        .arg(
                            arg!([name] "Preset name")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("create")
                        .about("Create a new preset")
                        .arg(
                            arg!([name] "Preset name")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a preset")
                        .arg(
                            arg!([name] "Preset name")
                                .required(true)
                        )
                )
                .about("Modify, list, and view presets")
        ).get_matches();

    match matches.subcommand() {
        Some(("create", _)) => println!("create was used"),
        Some(("preset", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("list", _)) => println!("preset list was used"),
                Some(("info", sub_matches)) => println!("preset info {:?} was used", sub_matches.value_of("name")),
                Some(("create", sub_matches)) => println!("preset create {:?} was used", sub_matches.value_of("name")),
                Some(("delete", sub_matches)) => println!("preset delete {:?} was used", sub_matches.value_of("name")),
                _ => unreachable!("exhausted list of subcommands")
            }
        }
        _ => unreachable!("exhausted list of subcommands")
    }

    let presets_path = get_presets_path();
    let samples = SampleSet{name: "test".to_string(), samples: ["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()]};
    println!("{:?}", samples);
    println!("Presets at: {:?}", presets_path);
}
