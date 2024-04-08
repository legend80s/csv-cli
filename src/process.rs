use csv::Reader;
use std::fs;

pub fn save_as_json(input: String, output: String) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        let json = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        records.push(json);
    }

    let json = serde_json::to_string_pretty(&records)?;
    // println!("json: {}", json);
    fs::write(output, json)?;

    Ok(())
}
