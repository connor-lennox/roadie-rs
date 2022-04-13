use std::{fs, env, path::PathBuf};

use serde::{Serialize, Deserialize};
use clap::{arg, command, Command};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct SampleSet {
    name: String,
    samples: [String; 8],
}

/// Provides the directory the presets.ron is in
fn get_res_dir() -> PathBuf {
        env::current_dir().expect("could not find resources folder").join("res")
}

fn get_samples_root() -> PathBuf {
    get_res_dir().join("samples")
}

/// Provides the path to the presets.ron file
fn get_presets_path() -> PathBuf {
    get_res_dir().join("presets.ron")
}

fn get_all_samples() -> Vec<String> {
    WalkDir::new(get_samples_root()).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .map(|d| String::from(d.file_name().to_string_lossy()))
        .collect()
}

/// Prompts the user to build a SampleSet
fn build_sample_set(name: String) -> SampleSet {
    SampleSet { name, samples: ["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string()] }
}

/// Write a list of presets. Overwrites previous presets file!
fn write_presets(presets: Vec<SampleSet>) {
    fs::create_dir_all(get_res_dir()).expect("failed to initialize resource directory");
    fs::write(get_presets_path(), ron::to_string(&presets).unwrap()).expect("failed to write presets file");
}

/// Read the presets from file.
/// If the presets file isn't present, or is corrupted, returns an empty Vec<SampleSet>
fn read_presets() -> Vec<SampleSet> {
    fs::read_to_string(get_presets_path()).ok()
        .map(|s| ron::from_str(&s).ok()).flatten()
        .unwrap_or_else(|| vec![])
}

/// Add a single preset to the presets file.
fn add_preset(preset: SampleSet) {
    let mut presets = read_presets();
    presets.push(preset);
    write_presets(presets);
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
                Some(("create", sub_matches)) => {
                    let samples = build_sample_set(sub_matches.value_of("name").unwrap().to_string());
                    add_preset(samples);
                },
                Some(("delete", sub_matches)) => println!("preset delete {:?} was used", sub_matches.value_of("name")),
                _ => unreachable!("exhausted list of subcommands")
            }
        }
        _ => unreachable!("exhausted list of subcommands")
    }
}
