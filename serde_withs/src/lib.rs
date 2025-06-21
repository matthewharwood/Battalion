use serde_with::{DeserializeFromStr, SerializeDisplay};
use surrealdb::sql::Thing;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, SerializeDisplay, DeserializeFromStr)]
pub struct ThingString(pub Thing);

impl fmt::Display for ThingString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ThingString {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Thing::from_str(s).map(Self).map_err(|_| String::from("invalid Thing"))
    }
}

impl From<Thing> for ThingString {
    fn from(t: Thing) -> Self {
        Self(t)
    }
}

impl From<ThingString> for Thing {
    fn from(w: ThingString) -> Self {
        w.0
    }
}

