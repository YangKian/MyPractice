#[cfg(test)]
mod test {
    use crate::leptjson::{LeptType, LeptValue};
    use crate::Result::{
        LeptParseExpectValue, LeptParseInvalidValue, LeptParseOk, LeptpParseRootNotSingular,
    };
    use crate::leptjson::LeptType::LeptNumber;

    #[test]
    fn test_parse_null() {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(String::from("null")), LeptParseOk);
    }

    #[test]
    fn test_parse_expect_value() {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(String::from("")), LeptParseExpectValue);

        v.value = LeptType::LeptFalse;
        assert_eq!(v.parse(String::from(" ")), LeptParseExpectValue);
    }

    #[test]
    fn test_parse_invalid_value() {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(String::from("nul")), LeptParseInvalidValue);

        v.value = LeptType::LeptFalse;
        assert_eq!(v.parse(String::from("?")), LeptParseInvalidValue);
    }

    #[test]
    fn test_parse_root_not_singular() {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(String::from("null x")), LeptpParseRootNotSingular);
    }

    #[test]
    fn test_parse_true() {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(String::from("true")), LeptParseOk);
    }

    #[test]
    fn test_parse_false() {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(String::from("false")), LeptParseOk);
    }

    fn test_number(expect: f64, json: &str) {
        let mut v = LeptValue::new(LeptType::LeptFalse);
        assert_eq!(v.parse(json.to_string()), LeptParseOk);
        assert_eq!(v.value, LeptNumber(expect));
    }

    #[test]
    fn test_parse_number() {
        test_number(0.0, "0")
    }
}
