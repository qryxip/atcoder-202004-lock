use cargo_lock::{Lockfile, Package, SourceId};
use itertools::Itertools as _;
use semver::Version;
use structopt::StructOpt;
use toml_edit::Document;

#[derive(StructOpt)]
struct Opt {}

fn main() {
    Opt::from_args();

    let Lockfile { mut packages, .. } =
        include_str!("./atcoder-cargo-lock.toml")
            .parse()
            .expect(concat!(
                "could not parse ",
                env!("CARGO_MANIFEST_DIR"),
                "/",
                file!(),
                "/atcoder-cargo-lock.toml",
            ));

    packages.retain(|Package { source, .. }| matches!(source, Some(s) if s.is_default_registry()));

    let mut output = packages
        .iter()
        .map(|package| {
            format!(
                "{} = {{ package = \"\", version = \"\", default-features = false }}\n",
                to_name_in_toml(package),
            )
        })
        .join("")
        .parse::<Document>()
        .expect("should be valid");

    for package in &packages {
        let name_in_toml = &to_name_in_toml(package);
        output[name_in_toml]["package"] = toml_edit::value(package.name.as_str());
        output[name_in_toml]["version"] = toml_edit::value(to_req(&package.version));
    }

    println!("{}", output.to_string().trim_end());
}

fn to_name_in_toml(package: &Package) -> String {
    format!(
        "{}-{}",
        package.name,
        package
            .version
            .to_string()
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
                _ => '-',
            })
            .collect::<String>()
    )
}

fn to_req(version: &Version) -> String {
    // https://doc.rust-lang.org/cargo/reference/resolver.html#version-metadata
    format!(
        "={}",
        Version {
            build: vec![],
            ..version.clone()
        },
    )
}
