use anyhow::Result;
use rs_utils::{
    config::{Migration, Versioned, do_migrate},
    derive::Versioned,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Default, Versioned)]
struct V1 {
    version: i32,
    a: i32,
}

#[derive(Serialize, Deserialize, Default, Versioned)]
struct V2 {
    version: i32,
    b: i32,
}

#[allow(dead_code)]
fn migrate_data(data: Value) -> Result<V2> {
    let migrations = vec![Migration {
        version: 2,
        migrate: |value| {
            let v1 = V1::from_value_or_default(value);

            let v2 = V2 {
                version: 2,
                b: v1.a,
            };

            return Ok(v2.to_value());
        },
    }];

    let t: V2 = do_migrate(data, migrations)?;

    return Ok(t);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn migrate_configuration() -> Result<()> {
        let data = json!({ "version": 1, "a": 32 });

        let v2: V2 = migrate_data(data)?;

        assert_eq!(v2.b, 32);
        Ok(())
    }

    #[test]
    fn migrate_with_corrupt_data() -> Result<()> {
        let corrupt_data = json!({});

        let v2: V2 = migrate_data(corrupt_data)?;

        assert_eq!(v2.b, 0);

        Ok(())
    }
}
