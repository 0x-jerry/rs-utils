#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rs_utils::macros::{Versioned, migration};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
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

    impl From<V1> for V2 {
        fn from(value: V1) -> Self {
            Self {
                b: value.a,
                ..Default::default()
            }
        }
    }

    #[derive(Serialize, Deserialize, SmartDefault, Versioned)]
    struct V3 {
        #[default = 3]
        version: u32,
        c: i32,
    }

    impl From<V2> for V3 {
        fn from(value: V2) -> Self {
            Self {
                c: value.b,
                ..Default::default()
            }
        }
    }

    #[allow(dead_code)]
    type LatestConfig = V3;

    #[test]
    fn migrate_configuration() -> Result<()> {
        let data = json!({ "version": 1, "a": 32 });

        let data = migration!(data, V1, V2, V3)?;

        assert_eq!(data.c, 32);
        assert_eq!(data.version, 3);
        Ok(())
    }

    #[test]
    fn migrate_from_middle() -> Result<()> {
        let data = json!({ "version": 2, "b": 33 });

        let data = migration!(data, V1, V2, V3)?;

        assert_eq!(data.c, 33);
        assert_eq!(data.version, 3);
        Ok(())
    }

    #[test]
    fn migrate_with_corrupt_data() -> Result<()> {
        let corrupt_data = json!({
            "a": 32
        });

        let data = migration!(corrupt_data, V1, V2, V3)?;

        assert_eq!(data.c, 0);
        assert_eq!(data.version, 3);

        Ok(())
    }
}
