extern crate serde_yaml;

use std::str;
use std::fs;
use std::io::{BufWriter, Write, BufReader, Read};
use std::env;
use serde_json::Value;

pub struct EsaConfig {
    pub team: String,
    pub screen_name: String,
    pub parsonal_access_token: String
}

impl EsaConfig {
    pub fn write(&self) {
        let mut f = BufWriter::new(fs::File::create(filename()).unwrap());
        write!(f, "esanippou:\n  team: {}\n  screen_name: {}\n  parsonal_access_token: {}", self.team, self.screen_name, self.parsonal_access_token).unwrap();
    }
}

pub fn new(p:(String, String, String)) -> EsaConfig {
    EsaConfig{team: p.0, screen_name: p.1, parsonal_access_token: p.2}
}

pub fn load() -> EsaConfig {
    let mut f = BufReader::new(fs::File::open(filename()).unwrap());
    let mut buf = vec![];

    f.read_to_end(&mut buf).unwrap();

    let value: Value = serde_yaml::from_str(str::from_utf8(&buf).unwrap()).unwrap();

    EsaConfig{
        team: value["esanippou"]["team"].as_str().unwrap().to_string(),
        screen_name: value["esanippou"]["team"].as_str().unwrap().to_string(),
        parsonal_access_token: value["esanippou"]["parsonal_access_token"].as_str().unwrap().to_string()
    }
}

fn filename() -> String {
    format!("{path}/{file}", path=env::home_dir().unwrap().to_str().unwrap(), file=".esanippourc")
}
