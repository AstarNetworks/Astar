//! Plasm Node CLI

#![warn(missing_docs)]

use sc_cli::VersionInfo;

fn main() -> Result<(), sc_cli::error::Error> {
    let version = VersionInfo {
        name: "Plasm Node",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "plasm-node",
        author: "Takumi Yamashita <takumi@stake.co.jp>",
        description: "PlasmChain Node",
        support_url: "https://github.com/staketechnologies/Plasm/issues/new",
        copyright_start_year: 2019,
    };

    plasm_cli::run(std::env::args(), version)
}
