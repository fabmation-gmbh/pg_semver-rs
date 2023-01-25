use pgx::{cstr_core::CStr, StringInfo};
use pgx::error;
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
