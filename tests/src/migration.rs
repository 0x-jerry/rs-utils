use anyhow::Result;
use rs_utils::{
    macros::Versioned,
    migration::{Migration, Versioned, do_migrate},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smart_default::SmartDefault;

#[derive(Serialize, Deserialize, SmartDefault, Versioned)]
struct V1 {
    #[default = 1]
    version: u32,
    a: i32,
}

#[derive(Serialize, Deserialize, SmartDefault, Versioned)]
struct V2 {
    #[default = 2]
    version: u32,
    b: i32,
}

#[derive(Serialize, Deserialize, SmartDefault, Versioned)]
struct V3 {
    #[default = 3]
    version: u32,
    c: i32,
}

type LatestConfig = V3;

#[allow(dead_code)]
fn migrate_data(data: Value) -> Result<LatestConfig> {
    let migrations = vec![
        Migration {
            version: 2,
            migrate: |value| {
                let v1 = V1::from_value_or_default(value);

                let v2 = V2 {
                    version: 2,
                    b: v1.a,
                };

                return Ok(v2.to_value());
            },
        },
        Migration {
            version: 3,
            migrate: |value| {
                let v1 = V2::from_value_or_default(value);

                let v2 = V3 {
                    version: 3,
                    c: v1.b,
                };

                return Ok(v2.to_value());
            },
        },
    ];

    let t = do_migrate(data, migrations)?;

    return Ok(t);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn migrate_configuration() -> Result<()> {
        let data = json!({ "version": 1, "a": 32 });

        let data = migrate_data(data)?;

        assert_eq!(data.c, 32);
        assert_eq!(data.version, 3);
        Ok(())
    }

    #[test]
    fn migrate_from_middle() -> Result<()> {
        let data = json!({ "version": 2, "b": 33 });

        let data = migrate_data(data)?;

        assert_eq!(data.c, 33);
        assert_eq!(data.version, 3);
        Ok(())
    }

    #[test]
    fn migrate_with_corrupt_data() -> Result<()> {
        let corrupt_data = json!({
            "a": 32
        });

        let data = migrate_data(corrupt_data)?;

        assert_eq!(data.c, 0);
        assert_eq!(data.version, 3);

        Ok(())
    }
}
