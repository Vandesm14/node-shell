const packageJsonPath = "./package.json";
const nodeModulesBinPath = "./node_modules/.bin";
const bashConfigPath = "/tmp/node-shell.bashrc";

const existsSync = (path: string) => {
  try {
    Deno.statSync(path);
    return true;
  } catch {
    return false;
  }
};

function main() {
  // Check if package.json exists
  if (!existsSync(packageJsonPath)) {
    console.error("package.json not found");
    Deno.exit(1);
  }

  // Read package.json
  const _packageJson = Deno.readTextFileSync(packageJsonPath);
  const packageJson = JSON.parse(_packageJson);

  // Get the scripts from package.json
  const scripts = packageJson.scripts;

  // Get the binaries from node_modules
  const binaries = Deno.readDirSync(nodeModulesBinPath);

  // Detect the package manager
  let packageManager = "npm";
  if (existsSync("yarn.lock")) {
    packageManager = "yarn";
  } else if (existsSync("pnpm-lock.yaml")) {
    packageManager = "pnpm";
  }

  // Create a bash configuration file
  const bashConfig = [];

  // Add aliases for scripts
  for (const script in scripts) {
    bashConfig.push(`alias ${script}='${packageManager} run ${script}'`);
  }

  // Add aliases for binaries
  for (const binary of binaries) {
    bashConfig.push(`alias ${binary.name}='${nodeModulesBinPath}/${binary.name}'`);
  }

  bashConfig.unshift(`PS1='\\[\\e[0;92m\\][\\[\\e[0;92m\\]node-shell\\[\\e[0;92m\\]:\\[\\e[0;92m\\]\\w\\[\\e[0;92m\\]]\\[\\e[0;92m\\]$ \\[\\e[0m\\]'`)
  bashConfig.unshift('source ~/.bashrc');

  bashConfig.unshift('#!/usr/bin/env bash');

  // Write to the bash configuration file
  Deno.writeTextFileSync(bashConfigPath, bashConfig.join("\n"));

  // Print the path of the bash configuration file
  console.log(bashConfigPath);
}

main();