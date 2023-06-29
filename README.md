# node-shell

A CLI to bring scripts, binaries, and packager commands to your fingertips (inspired by nix-shell)

## Installation

Download the latest [release](https://github.com/vandesm14/node-shell/releases) for your platform. This will contain a binary and a shell script.
The shell script is used to set up your environment, while the binary does the heavy-lifying. The shell script is `node-shell`, so you can set that as an alias or into your `$PATH`.

## Usage

Run `node-shell` in any directory that has a `package.json` or a `node_modules` folder. **node-shell** will find all of the binaries and scripts that you would normally proxy through your package manager, and add them to the top-level of a temporary [BASH](https://www.gnu.org/software/bash/) shell.

![](./.github/demo.gif)
