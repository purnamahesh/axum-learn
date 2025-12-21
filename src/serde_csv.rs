use serde::{Deserialize, Serialize};
use csv;

#[derive(Deserialize, Serialize, Debug)]
struct Man {
    first_name: String,
    last_name: String,
    age: u8,
    id: u32
}

fn main() {
    let dx = csv::Reader::from_path("test_file.csv");

    if let Ok(mut file) = dx {
        for record in file.deserialize() {
            let r:Man = record.unwrap();

            println!("{:?}", r);
        }
    }

}