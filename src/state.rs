use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();
    //file -> data
    file.read_to_string(&mut data).unwrap();
    //data -> json
    let json: Value = serde_json::from_str(&data).unwrap();
    //json -> state: Map
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state;
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string())
        .expect("Unable to write file")
}

pub fn run_it() {
    let status: &str = "good morning";
    let title: &str = "greeting";

    let mut state: Map<String, Value> = read_file(String::from("./data/state.json").as_str());
    println!("{:?}", state);

    state.insert(title.to_string(), json!(status));
    write_to_file("./data/state.json", &mut state);
}