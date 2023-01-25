use pgx::prelude::*;
use serde::{ser::{Serialize, Serializer, SerializeStruct}, de::{Deserialize, Deserializer, Error, SeqAccess, Visitor}};
use semver::{Version, Prerelease, BuildMetadata};

#[derive(PostgresType)]
pub struct Semver {
    major: u64,
    minor: u64,
    patch: u64,
    label: String,

    version: Version
}

impl Serialize for Semver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
       let mut state = serializer.serialize_struct("Semver", 4)?;

        state.serialize_field("major", &self.major)?;
        state.serialize_field("minor", &self.minor)?;
        state.serialize_field("patch", &self.patch)?;
        state.serialize_field("label", &self.label)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for Semver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Major,
            Minor,
            Patch,
            Label,
        }

        struct SemverVisitor;

        impl<'de> Visitor<'de> for SemverVisitor {
            type Value = Semver;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Semver")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Semver, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let major = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let minor = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;
                let patch = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &self))?;
                let label = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(3, &self))?;

                // TODO: Add pre and build
                let version = Version {
                    major,
                    minor,
                    patch,
                    pre: Prerelease::EMPTY,
                    build: BuildMetadata::EMPTY,
                };

                Ok(Semver{ major, minor, patch, label, version })
            }
        }

        const FIELDS: &'static [&'static str] = &["major", "minor", "patch", "label"];
        deserializer.deserialize_struct("Semver", FIELDS, SemverVisitor)
    }
}

#[pg_extern]
fn to_semver(version: &str) -> Semver {
    let raw_version = Version::parse(version).unwrap();

    Semver{
        major: raw_version.major,
        minor: raw_version.minor,
        patch: raw_version.patch,
        label: concat_str(raw_version.pre.as_str(), raw_version.build.as_str()),

        version: raw_version,
    }
}

fn concat_str(a: &str, b: &str) -> String {
    a.to_string() + b
}
