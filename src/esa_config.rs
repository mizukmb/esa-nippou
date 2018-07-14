use std::fs;
use std::io::{BufWriter, Write};

pub struct EsaConfig {
    team: String,
    screen_name: String,
    parsonal_access_token: String
}

impl EsaConfig {
    pub fn write(&self) {
        let filename = format!("{path}/{file}", path=env!("HOME"), file=".esanippourc");
        let mut f = BufWriter::new(fs::File::create(filename).unwrap());
        write!(f, "esanippou:\n  team: {}\n  screen_name: {}\n  parsonal_access_token: {}", self.team, self.screen_name, self.parsonal_access_token).unwrap();
    }
}

pub fn new(p:(String, String, String)) -> EsaConfig {
    EsaConfig{team: p.0, screen_name: p.1, parsonal_access_token: p.2}
}
