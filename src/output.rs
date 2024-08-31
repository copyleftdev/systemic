use csv::Writer;
use serde_json::json;
use std::fs::File;
use std::io::Write;

pub fn save_results(results: Vec<String>, format: &str, file_path: &str) {
    match format {
        "Json" => {
            let json_results = json!(results);
            std::fs::write(file_path, json_results.to_string()).expect("Unable to write to file");
        },
        "Csv" => {
            let mut wtr = Writer::from_path(file_path).expect("Unable to create CSV writer");
            for line in results {
                wtr.write_record(&[line]).expect("Unable to write CSV record");
            }
        },
        _ => {
            let mut file = File::create(file_path).expect("Unable to create file");
            for line in results {
                writeln!(file, "{}", line).expect("Unable to write to file");
            }
        }
    }
}
