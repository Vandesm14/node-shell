use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::OsString, fs, path::Path};

const PACKAGE_JSON_PATH: &str = "./package.json";
const NODE_MODULES_BIN_PATH: &str = "./node_modules/.bin";
const BASH_CONFIG_PATH: &str = "/tmp/node-shell.bashrc";

#[derive(Serialize, Deserialize, Debug)]
struct PackageJson {
  bin: Option<HashMap<String, String>>,
  scripts: Option<HashMap<String, String>>,
}

fn main() {
  // Create an empty vec for lines of the config file
  let mut bash_config: Vec<String> = vec![
    "#!/usr/bin/env bash".to_string(),
    "source ~/.bashrc".to_string(),
    "PS1=\"\\[\\e[0;92m\\][\\[\\e[0;92m\\]node-shell\\[\\e[0;92m\\]:\\[\\e[0;92m\\]\\w\\[\\e[0;92m\\]]\\[\\e[0;92m\\]$ \\[\\e[0m\\]\"".to_string()
  ];

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

  // Read package.json
  let package_json = fs::read_to_string(PACKAGE_JSON_PATH);
  let package_json = match package_json {
    Ok(package_json) => package_json,
    Err(e) => {
      eprintln!("Failed to read package.json: {e}");
      std::process::exit(1);
    }
  };
  let package_json: Option<PackageJson> =
    match serde_json::from_str(&package_json) {
      Ok(package_json) => package_json,
      Err(_) => {
        eprintln!("Failed to parse package.json");
        std::process::exit(1);
      }
    };

  if let Some(scripts) =
    package_json.and_then(|package_json| package_json.scripts)
  {
    scripts.into_iter().for_each(|(key, _)| {
      bash_config.push(format!("alias {key}=\"{package_manager} run {key}\""))
    });
  }

  // Get the binaries from node_modules
  let binaries: Vec<(OsString, OsString)> =
    match fs::read_dir(NODE_MODULES_BIN_PATH) {
      Ok(binaries) => binaries
        .into_iter()
        .filter_map(|bin| match bin {
          Ok(bin) => Some((bin.file_name(), bin.path().into_os_string())),
          Err(_) => None,
        })
        .collect::<Vec<_>>(),
      Err(_) => Vec::new(),
    };

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
    Err(e) => eprintln!("Failed to write config file: {e}"),
  }
}
