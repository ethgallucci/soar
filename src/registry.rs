use std::fs;
use std::path::{Path, PathBuf};
use std::{io, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use ureq;
use walkdir::WalkDir;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GenericErr {
    InitErr,
    QueryErr,
}

///! A Chain holds a vector of metadata which is usually represented as a serde_json::Value
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chain {
    pub meta: Value,
    pub id: Option<String>,
    pub rpc: Option<Vec<String>>,
}

impl Chain {
    pub fn new(meta: Value) -> Self {
        Chain {
            meta,
            id: None,
            rpc: None,
        }
    }

    pub fn parse_rpc(&mut self) -> Result<Vec<String>, ()> {
        Ok(self.meta["apis"]["rpc"]
            .as_array()
            .iter()
            .map(|v| serde_json::to_string(&v.clone()).unwrap())
            .collect::<Vec<String>>())
    }
}

///! Structs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainRPC {
    pub endpoint: String,
    pub last_response: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
    pub recent: Vec<Value>,
}

impl From<Value> for Registry {
    fn from(v: Value) -> Registry {
        Registry {
            recent: vec![v.clone()],
        }
    }
}

impl From<Registry> for Value {
    fn from(r: Registry) -> Value {
        Value::from(r.recent)
    }
}

///! Registry
/// (*) The registry struct keeps a record of the cosmos/chain-registry repository
impl Registry {
    pub fn new() -> Registry {
        // Pull latest registry files from github
        use fs::File;
        use std::process::Command;

        if let Err(_e) = fs::read_dir(Path::new("chain-registry")) {
            Command::new("git")
                .args([
                    "clone",
                    "https://github.com/cosmos/chain-registry",
                    // TODO: Add path to clone repo
                ])
                .status()
                .expect("Failed to get latest registry");

            let regpath = Path::new("chain-registry");

            // look at every folder in the registry, parse all chain.json files
            let registry_vals = crate::registry::Registry::dirwalk(regpath).unwrap();

            // Parse all RPC endpoints listed
            Registry {
                recent: registry_vals,
            }
        } else {
            // Don't need to pull changes
            Registry {
                recent: crate::registry::Registry::dirwalk(Path::new("chain-registry")).unwrap(),
            }
        }
    }

    fn dirwalk(path: impl AsRef<Path>) -> Result<Vec<Value>, std::io::Error> {
        let mut registry_vec: Vec<Value> = vec![];

        for entry in WalkDir::new(path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            log::info!("peeking: {}", f_name);
            if f_name.contains("chain.json") {
                let mut chain = fs::File::open(entry.path())?;
                let mut buf_string = String::new();
                chain.read_to_string(&mut buf_string)?;

                registry_vec.push(
                    serde_json::from_str(buf_string.as_str()).expect(
                        format!(
                            "Failed to read {}/chain.json",
                            entry.path().to_string_lossy()
                        )
                        .as_str(),
                    ),
                );
            }
        }

        Ok(registry_vec)
    }
}

// launch a query with a formatted response ////////////////////////////////////////////////////////////////
pub fn format_response(e: &str, q: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full = format!("{}{}", e, q);
    let res = ureq::get(&full).call()?.into_string()?;
    let j: serde_json::Value = serde_json::from_str(&res)?;
    let pretty = to_string_pretty(&j)?;
    Ok(pretty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_serialize() {
        let most_rec_reg = Registry::new();
        assert!(
            !(most_rec_reg.recent.is_empty()),
            "Failed to serialize registry"
        );
    }
}
