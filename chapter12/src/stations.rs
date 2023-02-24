use serde::{Deserialize, de};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::fmt::Debug;

#[derive(Debug , Deserialize)]
pub struct Stations<T> {
    values: HashMap<String , T>
}
impl<T> Stations<T> {
    pub fn new() -> Stations<T> where T:de::DeserializeOwned + Debug {
        let path =  env::current_dir()
            .map(|path| path.join("resources\\stations.json"))
            .unwrap_or_else(|error| panic!("{:?}" , error));
        let reader = File::open(path)
            .and_then(|file| Ok(BufReader::new(file)))
            .unwrap_or_else(|error| panic!("{:?}" , error));
        serde_json::from_reader(reader)
            .unwrap_or_else(|error| panic!("{:?}" , error))
    }
    pub fn search_by_name(&self, name: String) -> Option<&T> where T:de::DeserializeOwned + Debug {
        self.values.get(&name)
    }
}