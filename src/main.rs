use std::{fs, env, path::PathBuf};

use serde::{Serialize, Deserialize};
use clap::{arg, command, Command};

#[derive(Debug, Serialize, Deserialize)]
struct SampleSet {
    name: String,
    samples: [String; 8],
}

/// Provides the directory the presets.ron is in
fn get_presets_dir() -> PathBuf {
    [
        env::current_dir().expect("could not find resources folder").to_str().expect("invalid resources folder path"),
        "res"
    ].iter().collect()
}

/// Provides the path to the presets.ron file
fn get_presets_path() -> PathBuf {
    [
        get_presets_dir().to_str().unwrap(),
        "presets.ron"
    ].iter().collect()
}

/// Write a list of presets. Overwrites previous presets file!
fn write_presets(presets: Vec<SampleSet>) {
    fs::create_dir_all(get_presets_dir());
    fs::write(get_presets_path(), ron::to_string(&presets).unwrap());
}

/// Read the presets from file.
fn read_presets() -> Vec<SampleSet> {
    ron::from_str(&fs::read_to_string(get_presets_path()).expect("could not open presets file")).expect("could not parse presets file")
}

/// List the presets available in the presets.ron file
fn list_presets() {
    for preset in read_presets() {
        println!("{}: {:?}", preset.name, preset.samples)
    }
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
                Some(("list", _)) => list_presets(),
                Some(("info", sub_matches)) => println!("preset info {:?} was used", sub_matches.value_of("name")),
                Some(("create", sub_matches)) => println!("preset create {:?} was used", sub_matches.value_of("name")),
                Some(("delete", sub_matches)) => println!("preset delete {:?} was used", sub_matches.value_of("name")),
                _ => unreachable!("exhausted list of subcommands")
            }
        }
        _ => unreachable!("exhausted list of subcommands")
    }
}
