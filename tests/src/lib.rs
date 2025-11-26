#[cfg(test)]
mod tests {
    use rs_utils::{
        config::{Migration, do_migrate},
        derive::Versioned,
    };

    use serde::{Deserialize, Serialize};

    #[test]
    fn migrate_configuration() {
        #[derive(Serialize, Deserialize, Versioned)]
        struct V1 {
            version: i32,
            a: i32,
        }

        #[derive(Serialize, Deserialize, Versioned)]
        struct V2 {
            version: i32,
            b: i32,
        }

        let v1 = V1 { version: 1, a: 32 };

        let migrations = vec![Migration {
            version: 2,
            migrate: |v1| {
                let v1: V1 = v1.into();

                let v2 = V2 {
                    version: 2,
                    b: v1.a,
                };

                return v2.into();
            },
        }];

        let v2: V2 = do_migrate(v1, migrations);

        assert_eq!(v2.b, 32);
    }
}
