#[cfg(test)]
mod tests {
    use rs_utils::macros::chain_from;
    use serde_json::json;

    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize, Default)]
    struct V1 {
        a: i32,
    }

    impl From<Value> for V1 {
        fn from(value: Value) -> Self {
            serde_json::from_value(value).unwrap_or_default()
        }
    }

    struct V2 {
        b: i32,
    }

    impl From<V1> for V2 {
        fn from(value: V1) -> Self {
            return Self { b: value.a };
        }
    }

    #[test]
    fn chain_from_macro() {
        let corrupt_data = json!({});

        let v2 = chain_from!(corrupt_data, V1, V2);

        assert_eq!(v2.b, 0);
    }
}
