use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config{
    user_name: String,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            user_name: "User".to_string(),
        }
    }
}

impl Config {
    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }

    pub fn set_user_name(&mut self, name: String) {
        self.user_name = name;
    }
}

impl Config {
    pub fn full_file_path() -> PathBuf {
        return env::current_dir().unwrap().join("config.json");
    }

    pub fn read_file(&mut self){
        if Self::full_file_path().exists() {
            let file = match File::open(Self::full_file_path()){
                Ok(file) => file,
                Err(error) => panic!("There was a problem opening the file: {:?}", error),
            };



            let reader = BufReader::new(file);

            let this: Config = match serde_json::from_reader(reader) {
                Ok(config) => config,
                Err(error) => panic!("There was a problem deserializing the file: {:?}", error),
            };

            *self = this;
            Self::write_file(self);
        }else{
            Self::write_file(self);
        }
    }

    pub fn write_file(&self){
        let file = File::create(Self::full_file_path()).unwrap();
        let writer = BufWriter::new(file);
        match serde_json::to_writer_pretty(writer, self){
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
}

