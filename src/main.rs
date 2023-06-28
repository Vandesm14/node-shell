use std::{collections::BTreeMap, ffi::OsString, fs, path::Path};

use serde::{Deserialize, Serialize};

const PACKAGE_JSON_PATH: &str = "./package.json";
const NODE_MODULES_BIN_PATH: &str = "./node_modules/.bin";
const BASH_CONFIG_PATH: &str = "/tmp/node-shell.bashrc";

#[derive(Serialize, Deserialize, Debug)]
struct PackageJson {
  bin: Option<BTreeMap<String, String>>,
  scripts: Option<BTreeMap<String, String>>,
}

fn main() {
  // Check if package.json exists
  if !Path::new(PACKAGE_JSON_PATH).exists() {
    println!("package.json not found")
  }

  // Read package.json
  let package_json = fs::read_to_string(PACKAGE_JSON_PATH);
  let package_json: Option<PackageJson> = match package_json {
    Ok(package_json) => match serde_json::from_str(&package_json) {
      Ok(package_json) => Some(package_json),
      Err(_) => None,
    },
    Err(_) => None,
  };

  // Get the scripts from package.json
  let scripts = match package_json {
    Some(package_json) => match package_json.scripts {
      Some(scripts) => scripts,
      None => BTreeMap::new(),
    },
    None => BTreeMap::new(),
  };

  // Get the binaries from node_modules
  let binaries: Vec<(OsString, OsString)> =
    match fs::read_dir(NODE_MODULES_BIN_PATH) {
      #[allow(clippy::unnecessary_filter_map)]
      Ok(binaries) => binaries
        .into_iter()
        .filter_map(|bin| match bin {
          Ok(bin) => Some(bin),
          Err(_) => None,
        })
        .filter_map(|bin| {
          let (name, path) = (bin.file_name(), bin.path().into_os_string());
          Some((name, path))
        })
        .collect::<Vec<_>>(),
      Err(_) => Vec::new(),
    };

  // Detect the package manager
  let package_manager = {
    if Path::new("./yarn.lock").exists() {
      "yarn"
    } else if Path::new("./pnpm-lock.yaml").exists() {
      "pnpm"
    } else {
      "npm"
    }
  };

  // Create an empty vec for lines of the config file
  let mut bash_config: Vec<String> = vec![
    "#!/usr/bin/env bash".to_string(),
    "source ~/.bashrc".to_string(),
    "PS1=\"\\[\\e[0;92m\\][\\[\\e[0;92m\\]node-shell\\[\\e[0;92m\\]:\\[\\e[0;92m\\]\\w\\[\\e[0;92m\\]]\\[\\e[0;92m\\]$ \\[\\e[0m\\]\"".to_string()
  ];

  // Add aliases for scripts
  scripts.into_iter().for_each(|(key, _)| {
    bash_config
      .push(format!("alias {}=\"{} run {}\"", key, package_manager, key))
  });

  // Add aliases for binaries
  binaries.into_iter().for_each(|(name, path)| {
    bash_config.push(format!(
      "alias {}=\"{}\"",
      name.into_string().unwrap(),
      path.into_string().unwrap()
    ))
  });

  // Write the config file
  let result = fs::write(BASH_CONFIG_PATH, bash_config.join("\n"));

  match result {
    Ok(_) => println!("{}", BASH_CONFIG_PATH),
    Err(e) => println!("{}", e),
  }
}
