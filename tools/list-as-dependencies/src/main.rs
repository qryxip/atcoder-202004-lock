use cargo_lock::{Lockfile, Package};
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

    let mut output = (0..packages.len())
        .map(|i| {
            format!(
                "package{:<02} = {{ package = \"\", version = \"\", default-features = false }}\n",
                i,
            )
        })
        .join("")
        .parse::<Document>()
        .expect("should be valid");

    for (i, Package { name, version, .. }) in packages.iter().enumerate() {
        let name_in_toml = &format!("package{:<02}", i);
        output[name_in_toml]["package"] = toml_edit::value(name.as_str());
        output[name_in_toml]["version"] = toml_edit::value(to_req(version));
    }

    println!("{}", output.to_string().trim_end());
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
