use pgx::prelude::*;
use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PostgresType)]
pub struct Semver {
    version: Version
}


#[pg_extern]
fn to_semver(version: &str) -> Semver {
    let raw_version = Version::parse(version).unwrap();

    Semver{
        version: raw_version,
    }
}
