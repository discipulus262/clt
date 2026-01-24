use serde::Serialize;
use std::fs::File;
use std::io::Write;
use toml::to_string;

#[derive(Serialize)]
struct Task {
    name: String,
}

fn write_config_to_file(config: &Task, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

pub fn write(name: String) {
    let config = Task {
        name: name.to_owned(),
    };

    if let Err(e) = write_config_to_file(&config, "config.toml") {
        eprintln!("Error: {}", e);
    } else {
        println!("task saved")
    }
}
