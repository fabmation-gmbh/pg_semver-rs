use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use pgx::{cstr_core::CStr, StringInfo};
use pgx::{error, PostgresHash, PostgresOrd, PostgresEq};
use pgx::prelude::*;
use pgx::*;
use pgx::inoutfuncs::InOutFuncs;
use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PostgresType)]
#[derive(PostgresEq, PostgresOrd, PostgresHash)]
#[inoutfuncs]
pub struct Semver {
    version: Version
}

impl InOutFuncs for Semver {
    fn input(input: &CStr) -> Semver {
        let input_str = match input.to_str() {
            Ok(data) => data,
            Err(error) => panic!("Unable to get &str representation from &CStr: {:?}", error),
        };
        let version = match Version::parse(input_str) {
            Ok(data) => data,
            Err(err) => error!("unable to parse version: {:?}", err),
        };

        Semver { version }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{}", self.version));
    }
}

impl PartialEq for Semver {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl Eq for Semver {}

impl Ord for Semver {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialOrd for Semver {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Semver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}

#[pg_extern]
fn to_semver(version: &str) -> Semver {
    let ver = match Version::parse(version) {
        Ok(data) => data,
        Err(err) => error!("unable to parse version: {:?}", err),
    };

    Semver{
        version: ver,
    }
}
