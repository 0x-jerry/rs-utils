use serde_json::Value;

pub struct Migration {
    pub version: i64,
    pub migrate: fn(data: Value) -> Value,
}

pub fn do_migrate<T: Versioned, U: Versioned>(data: T, migrations: Vec<Migration>) -> U {
    let mut result: Value = data.into();

    for migration in migrations {
        let version = result.get("version").unwrap().as_i64().unwrap();

        if migration.version > version {
            result = (migration.migrate)(result);
        }
    }

    return result.into();
}

pub trait Versioned: From<Value> + Into<Value> {
    fn get_version(&self) -> i64;
}
