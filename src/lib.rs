#![allow(unused)]

mod flat_struct {
    #[derive(Debug, serde::Deserialize)]
    struct Data {
        name: String,
    }

    #[test]
    fn direct_from_invalid_raw() {
        let raw = "not JSON";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("expected ident", line: 1, column: 2),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_missing_data() {
        let raw = "{}";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `name`", line: 1, column: 2),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_wrong_data() {
        let raw = "{ \"name\": 13 }";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `13`, expected a string", line: 1, column: 12),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn via_value_from_invalid_raw() {
        let raw = "not JSON";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Err(
            Error("expected ident", line: 1, column: 2),
        )
        */
        assert!(value.is_ok());

        // let value = value.unwrap();
        // let got = serde_json::from_value::<Data>(value);
        // println!("{got:#?}");
        // assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_missing_data() {
        let raw = "{}";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {},
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `name`", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_wrong_data() {
        let raw = "{ \"name\": 13 }";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {
                "name": Number(13),
            },
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `13`, expected a string", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }
}

mod flat_many_struct {
    #[derive(Debug, serde::Deserialize)]
    struct Data {
        name: String,
        surname: String,
    }

    #[test]
    fn direct_from_invalid_raw() {
        let raw = "not JSON";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("expected ident", line: 1, column: 2),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_missing_data() {
        let raw = "{}";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `name`", line: 1, column: 2),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_wrong_data() {
        let raw = "{ \"name\": 13 }";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `13`, expected a string", line: 1, column: 12),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn via_value_from_invalid_raw() {
        let raw = "not JSON";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Err(
            Error("expected ident", line: 1, column: 2),
        )
        */
        assert!(value.is_ok());

        // let value = value.unwrap();
        // let got = serde_json::from_value::<Data>(value);
        // println!("{got:#?}");
        // assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_missing_data() {
        let raw = "{}";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {},
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `name`", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_wrong_data() {
        let raw = "{ \"name\": 13 }";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {
                "name": Number(13),
            },
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `13`, expected a string", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }
}

mod nested_struct {
    #[derive(Debug, serde::Deserialize)]
    struct Data {
        id: String,
        inner: Inner,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Inner {
        name: String,
        surname: String,
    }

    #[test]
    fn direct_from_invalid_raw() {
        let raw = "not JSON";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("expected ident", line: 1, column: 2),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_missing_base_data() {
        let raw = "{}";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `id`", line: 1, column: 2),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_missing_inner_data() {
        let raw = "{ \"id\": \"123\", \"inner\": {} }";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `name`", line: 1, column: 26),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_wrong_base_data() {
        let raw = "{ \"id\": 13 }";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `13`, expected a string", line: 1, column: 10),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn direct_from_valid_raw_wrong_inner_data() {
        let raw = "{ \"id\": \"13\", \"inner\": { \"name\": \"ABC\", \"surname\": 42 } }";
        let got = serde_json::from_str::<Data>(raw);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `42`, expected a string", line: 1, column: 53),
        )
        */
        assert!(got.is_ok())
    }

    #[test]
    fn via_value_from_invalid_raw() {
        let raw = "not JSON";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Err(
            Error("expected ident", line: 1, column: 2),
        )
        */
        assert!(value.is_ok());

        // let value = value.unwrap();
        // let got = serde_json::from_value::<Data>(value);
        // println!("{got:#?}");
        // assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_missing_base_data() {
        let raw = "{}";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {},
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `id`", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_missing_inner_data() {
        let raw = "{ \"id\": \"13\", \"inner\": { \"name\": \"ABC\" } }";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {
                "id": String("13"),
                "inner": Object {
                    "name": String("ABC"),
                },
            },
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("missing field `surname`", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_wrong_base_data() {
        let raw = "{ \"id\": 13 }";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {
                "id": Number(13),
            },
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `13`, expected a string", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }

    #[test]
    fn via_value_from_valid_raw_wrong_inner_data() {
        let raw = "{ \"id\": \"13\", \"inner\": { \"name\": \"ABC\", \"surname\": 42 } }";
        let value = serde_json::from_str::<serde_json::Value>(raw);
        println!("{value:#?}");
        /*
        Ok(
            Object {
                "id": String("13"),
                "inner": Object {
                    "name": String("ABC"),
                    "surname": Number(42),
                },
            },
        )
        */
        assert!(value.is_ok());

        let value = value.unwrap();
        let got = serde_json::from_value::<Data>(value);
        println!("{got:#?}");
        /*
        Err(
            Error("invalid type: integer `42`, expected a string", line: 0, column: 0),
        )
        */
        assert!(got.is_ok());
    }
}
