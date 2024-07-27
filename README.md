# Did I Break It
[![crates.io](https://img.shields.io/crates/v/did_i_break_it)](https://crates.io/crates/did_i_break_it)
[![Continuous Integration (CI)](https://github.com/DeveloperC286/did_i_break_it/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/DeveloperC286/did_i_break_it/actions/workflows/continuous-integration.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A tooling for checking your [https://crates.io](https://crates.io) library's reverse dependencies with your local version.

__NOTE - Only Unix like environments are supported.__


## Upcoming
 * Enable API/download retrying if request fails.
 * Lean mode, which cleans up between each reverse dependency.


## Content
 * [Usage](#usage)
   + [Usage - Logging](#usage-logging)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
This tool will download all the reverse dependencies from [https://crates.io](https://crates.io) and attempt to compile it using a local version of your library, regardless of the version it specifies in its Cargo manifest.

This enables you to test you have not accidentally introduced any breaking changes in minor Semantic Versioning bumps.
In addition you can also estimate the impact of breaking changes you might introduce.

Simply change into the directory containing your local copy of your library and execute the `did_i_break_it` binary.
If you do not want to change into the directory you can use the argument `--local-crate <local-crate>` and provide the path to the local library.


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/did_i_break_it/-/releases](https://gitlab.com/DeveloperC/did_i_break_it/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```sh
git clone git@gitlab.com:DeveloperC/did_i_break_it.git
cd did_i_break_it/
cargo build --release
```

The compiled binary is present in `target/release/did_i_break_it`.


## Compiling via Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/did_i_break_it) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/did_i_break_it`.

```sh
cargo install did_i_break_it
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```sh
cargo install did_i_break_it --version 0.6.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```sh
cargo install did_i_break_it --version ^0
```

Will download the latest `0.*` release whether that is `0.6.0` or `0.9.2`.


## Unit Testing
The unit test suite has tests testing the crates.io API response format parsing.
Cargo is used to set up and run all the unit tests.

```sh
cargo test
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://github.com/DeveloperC286/did_i_break_it/issues](https://github.com/DeveloperC286/did_i_break_it/issues).
