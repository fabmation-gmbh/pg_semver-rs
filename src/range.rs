use std::collections::VecDeque;
use std::str::Split;
use std::{error::Error, fmt};

use pgx::{cstr_core::CStr, StringInfo};
use pgx::{error, PostgresHash, PostgresOrd, PostgresEq};
use pgx::prelude::*;
use pgx::*;
use pgx::inoutfuncs::InOutFuncs;
use semver::{Op, VersionReq, Comparator, Version};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct ErrRangeParsing;

impl Error for ErrRangeParsing {}

impl fmt::Display for ErrRangeParsing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse semver range")
    }
}

#[derive(Serialize, Deserialize, PostgresType)]
#[inoutfuncs]
pub struct SemverRange {
    human_rep: String,

    #[serde(skip_serializing)]
    req: VersionReq
}

impl InOutFuncs for SemverRange {
    fn input(input: &CStr) -> SemverRange {
        let input_str = match input.to_str() {
            Ok(data) => data,
            Err(error) => panic!("Unable to get &str representation from &CStr: {:?}", error),
        };

        let data = input_str.to_string();
        let range = match parse_range(data.clone()) {
            Ok(data) => data,
            Err(err_str) => error!("unable to parse range {}", err_str),
        };

        SemverRange {
            human_rep: data,
            req: range,
        }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.human_rep);
    }
}

fn parse_range(data: String) -> Result<VersionReq, ErrRangeParsing> {
    let mut chars = data.chars();
    let len = data.len();
    let start_op = get_comparator_op(&chars.nth(0).ok_or(ErrRangeParsing)?, true);
    let stop_op = get_comparator_op(&chars.nth(len-1).ok_or(ErrRangeParsing)?, false);

    let data = &data[1..(len-1)];
    let mut versions = data.split(",");
    let entries = versions.clone().count(); // NOTE: clone() does not allocate memory

    if entries == 0 {
        return Err(ErrRangeParsing)
    }

    let mut comp: Vec<Comparator> = Vec::new();
    let first_str = versions.nth(0).ok_or(ErrRangeParsing)?;

    let start_ver = Version::parse(first_str).or(Err(ErrRangeParsing))?;

    comp.push(Comparator {
        op: start_op,
        major: start_ver.major,
        minor: Some(start_ver.minor),
        patch: Some(start_ver.patch),
        pre: start_ver.pre,
    });

    if entries == 1 {
        return Ok(VersionReq { comparators: comp })
    }

    let last_str = versions.nth(1).ok_or(ErrRangeParsing)?;
    let stop_ver = Version::parse(last_str).or(Err(ErrRangeParsing))?;

    comp.push(Comparator {
        op: stop_op,
        major: stop_ver.major,
        minor: Some(stop_ver.minor),
        patch: Some(stop_ver.patch),
        pre: stop_ver.pre,
    });

    Ok(VersionReq { comparators: comp })
}

fn get_comparator_op(c: &char, is_start: bool) -> Op {
    match c {
        '[' if is_start => Op::GreaterEq,
        '(' if is_start => Op::Greater,
        ']' if !is_start => Op::LessEq,
        ')' if !is_start => Op::Less,
        _ => error!("invalid range"),
    }
}
