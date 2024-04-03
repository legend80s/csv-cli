use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    /// Name
    name: String,
    /// Position
    position: String,
    /// Nationality
    nationality: String,
    /// DOB
    #[serde(rename = "DOB")]
    dob: String,
    /// Kit Number
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn save_as_json(input: String, output: String) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        records.push(record)
    }

    let json = serde_json::to_string_pretty(&records)?;
    // println!("json: {}", json);
    fs::write(output, json)?;

    Ok(())
}
