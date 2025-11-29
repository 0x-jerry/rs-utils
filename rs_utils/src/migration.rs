use anyhow::{Ok, Result};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use thiserror::Error;

pub struct Migration {
    pub version: u32,
    pub migrate: fn(data: Value) -> Result<Value>,
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Migrate failed: {0}!")]
    Failed(&'static str),
}

pub fn do_migrate<U: Versioned>(data: Value, migrations: Vec<Migration>) -> Result<U> {
    let mut result = data;

    for migration in migrations {
        let version = result
            .get("version")
            .map(|v| v.as_u64())
            .flatten()
            .unwrap_or(0) as u32;

        if migration.version > version {
            result = (migration.migrate)(result)?;
        }
    }

    let result = U::from_value(result)?;

    return Ok(result);
}

pub trait Versioned: DeserializeOwned + Serialize + Default {
    fn get_version(&self) -> u32;

    fn from_value(value: Value) -> Result<Self> {
        Ok(serde_json::from_value(value)?)
    }

    fn to_value(&self) -> Value {
        serde_json::to_value(self).expect("Serialize to json failed!")
    }

    fn from_value_or_default(value: Value) -> Self {
        Self::from_value(value).unwrap_or_default()
    }
}
