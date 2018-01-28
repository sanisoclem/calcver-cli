use toml;
use std::path::{Path};
use toml::Value;
use std::io::prelude::*;
use std::fs::File;


pub enum ReleaseConfig {
    FileReplace {
        files: String, //glob
        replace_match: String, // regex to look for 
    },
    CargoToml(String)
}

impl ReleaseConfig {
    pub fn run(&self, path: &str, version: &str) {
        match self { 
            &ReleaseConfig::CargoToml(ref cargo_path) => {
                let full_path = Path::new(path).join(cargo_path);
                let mut f = File::open(full_path).unwrap();
                let mut buffer = String::new();
                f.read_to_string(&mut buffer).unwrap();
                let newtoml = update_toml(&buffer,version);
                f.write(newtoml.as_bytes()).unwrap();
            },
            _fr @ &ReleaseConfig::FileReplace { .. } => {
                
            }
        };
    }
}

fn update_toml(toml: &str,version: &str) -> String {
    let mut val: Value = toml.parse::<Value>().unwrap(); {
        let table = val.as_table_mut().unwrap();
        let pkg  = table.get_mut("package").unwrap().as_table_mut().unwrap();
        let v = pkg.get_mut("version").unwrap();
        let newv = String::from(version);
        *v = Value::String(newv);
    }
    toml::to_string_pretty(&val).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn update_cargo_toml_version(){
        let toml =  r###"[package]
name = "calcver"
version = "0.1.0"
authors = ["Jerahmeel Cosinas <me@jerahmeelcosinas.net>","Joelle Ortiz <contact@joelleortiz.me>"]

[dependencies]
libcalcver = "0.2.0-beta.2"
clap = "2.29.2"
git2 = "0.6"
toml = "0.4"
serde_derive = "1.0"
"###;
        let parsed = update_toml(toml, "1.2.3").parse::<Value>().unwrap();
        let updated = parsed.as_table().unwrap()
            .get("package").unwrap().as_table().unwrap()
            .get("version").unwrap().as_str().unwrap();

        assert_eq!(updated,"1.2.3");
    }
}