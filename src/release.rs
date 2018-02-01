use toml;
use std;
use std::process::Command;
use std::path::{Path};
use toml::Value;
use std::io::prelude::*;
use std::fs::{File,OpenOptions};

pub enum ReleaseConfig {
    RunScript(String),
    CargoToml(String)
}

impl ReleaseConfig {
    pub fn run(&self, path: &str, version: &str) {
        match self { 
            &ReleaseConfig::CargoToml(ref cargo_path) => {
                // -- get path
                let full_path = Path::new(path).join(cargo_path);

                let mut buffer = String::new();
                {
                    let mut f = File::open(&full_path).unwrap();
                    f.read_to_string(&mut buffer).unwrap();
                }
                {
                    let (old_version,newtoml) = update_toml(&buffer,version);
                    let mut file = OpenOptions::new().write(true).truncate(true).open(&full_path).unwrap();
                    file.write_all(newtoml.as_bytes()).unwrap();
                    println!("Patched '{}'\nOld version: {}\nNew version: {}",full_path.to_str().unwrap(),old_version,version);
                }
            },
            &ReleaseConfig::RunScript (ref script) => {
                let output = run_command(script,version);
                // -- todo: merge???
                if output.stderr.len() > 0 {
                    println!("{}",std::str::from_utf8(&output.stderr).unwrap());
                }
                if output.stdout.len() > 0 {
                    println!("{}",std::str::from_utf8(&output.stdout).unwrap());
                }
                println!("'{}' exited with code: {}",script,output.status.code().unwrap())
            }
        };
    }
}

fn update_toml(toml: &str,version: &str) -> (String,String) {
    let old;
    let mut val: Value = toml.parse::<Value>().unwrap(); {
        let table = val.as_table_mut().unwrap();
        let pkg  = table.get_mut("package").unwrap().as_table_mut().unwrap();
        let v = pkg.get_mut("version").unwrap();
        old = v.as_str().unwrap().to_owned();
        let newv = String::from(version);
        *v = Value::String(newv);
    }
    (old,toml::to_string_pretty(&val).unwrap())
}

fn run_command(cmd: &str,version: &str) -> std::process::Output {
    if cfg!(target_os = "windows") {
    Command::new("cmd")
            .env("CALCVER_VERSION", version)
            .args(&["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
    Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    }
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