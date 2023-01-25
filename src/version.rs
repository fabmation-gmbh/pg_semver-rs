use pgx::{cstr_core::CStr, StringInfo};
use pgx::prelude::*;
use pgx::inoutfuncs::InOutFuncs;
use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PostgresType)]
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
        let version = Version::parse(input_str).unwrap();

        Semver { version }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{}", self.version));
    }
}


#[pg_extern]
fn to_semver(version: &str) -> Semver {
    let raw_version = Version::parse(version).unwrap();

    Semver{
        version: raw_version,
    }
}
