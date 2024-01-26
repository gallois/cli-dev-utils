use crate::dev_utils::convert::*;

mod tests {
    use super::*;

    #[test]
    fn test_json2csv() {
        let result = json2csv("{\"a\": 1, \"b\": 2, \"c\": 3}");
        match result {
            Ok(s) => assert_eq!(s, "a,b,c\n1,2,3\n"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_json2yaml() {
        let result = json2yaml("{\"a\": 1, \"b\": 2, \"c\": 3}");
        match result {
            Ok(s) => assert_eq!(s, "a: 1\nb: 2\nc: 3\n"),
            Err(e) => panic!("{:#?}", e),
        }
        let data = r#"
        {
            "checked": false,
            "dimensions": {
                "width": 5,
                "height": 10
            },
            "id": 1,
            "name": "A green door",
            "price": 12.5,
            "tags": [
                "home",
                "green"
            ]
        }
        "#;
        let result = json2yaml(data);
        match result {
            Ok(s) => assert_eq!(s, "checked: false\ndimensions:\n  height: 10\n  width: 5\nid: 1\nname: A green door\nprice: 12.5\ntags:\n- home\n- green\n"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_csv2tsv() {
        let result = csv2tsv("a,b,c\n1,2,3");
        assert_eq!(result, "a\tb\tc\n1\t2\t3");
    }

    #[test]
    fn test_string2hex() {
        let result = string2hex("abc");
        assert_eq!(result, "616263");
    }

    #[test]
    fn test_hex2string() {
        let result = hex2string("616263");
        match result {
            Ok(s) => assert_eq!(s, "abc"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_text2nato() {
        let result = text2nato("abc");
        assert_eq!(result, "Alpha Bravo Charlie");
    }

    #[test]
    fn test_slugify() {
        let result = slugify("Hello World");
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_celsius2fahrenheit() {
        let result = celsius2fahrenheit("0");
        match result {
            Ok(s) => assert_eq!(s, 32.0),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_fahrenheit2celsius() {
        let result = fahrenheit2celsius("32");
        match result {
            Ok(s) => assert_eq!(s, 0.0),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_celsius2kelvin() {
        let result = celsius2kelvin("0");
        match result {
            Ok(s) => assert_eq!(s, 273.15),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_kelvin2celsius() {
        let result = kelvin2celsius("273.15");
        match result {
            Ok(s) => assert_eq!(s, 0.0),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_fahrenheit2kelvin() {
        let result = fahrenheit2kelvin("32");
        match result {
            Ok(s) => assert_eq!(s, 273.15),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_kelvin2fahrenheit() {
        let result = kelvin2fahrenheit("273.15");
        match result {
            Ok(s) => assert_eq!(s, 32.0),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_text2asciibinary() {
        let result = text2asciibinary("abc");
        match result {
            Ok(s) => assert_eq!(s, "01100001 01100010 01100011"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = text2asciibinary("á ê ç õ");
        match result {
            Ok(s) => assert_eq!(
                s,
                "11100001 00100000 11101010 00100000 11100111 00100000 11110101"
            ),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_asciibinary2text() {
        let result = asciibinary2text("01100001 01100010 01100011");
        match result {
            Ok(s) => assert_eq!(s, "abc"),
            Err(e) => panic!("{:#?}", e),
        }

        let result =
            asciibinary2text("11100001 00100000 11101010 00100000 11100111 00100000 11110101");
        match result {
            Ok(s) => assert_eq!(s, "á ê ç õ"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_kilometers2miles() {
        let result = kilometers2miles("0");
        match result {
            Ok(s) => assert_eq!(s, "0"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = kilometers2miles("1");
        match result {
            Ok(s) => assert_eq!(s, "0.621371"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_miles2kilometers() {
        let result = miles2kilometers("0");
        match result {
            Ok(s) => assert_eq!(s, "0"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = miles2kilometers("1");
        match result {
            Ok(s) => assert_eq!(s, "1.6093444978925633"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_pounds2kilos() {
        let result = pounds2kilos("0");
        match result {
            Ok(s) => assert_eq!(s, "0"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = pounds2kilos("1");
        match result {
            Ok(s) => assert_eq!(s, "0.453592"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_kilos2pounds() {
        let result = kilos2pounds("0");
        match result {
            Ok(s) => assert_eq!(s, "0"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = kilos2pounds("1");
        match result {
            Ok(s) => assert_eq!(s, "2.2046244201837775"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_arabic2roman() {
        let result = arabic2roman("1");
        match result {
            Ok(s) => assert_eq!(s, "I"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("2");
        match result {
            Ok(s) => assert_eq!(s, "II"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("3");
        match result {
            Ok(s) => assert_eq!(s, "III"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("4");
        match result {
            Ok(s) => assert_eq!(s, "IV"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("5");
        match result {
            Ok(s) => assert_eq!(s, "V"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("6");
        match result {
            Ok(s) => assert_eq!(s, "VI"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("10");
        match result {
            Ok(s) => assert_eq!(s, "X"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("11");
        match result {
            Ok(s) => assert_eq!(s, "XI"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("20");
        match result {
            Ok(s) => assert_eq!(s, "XX"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("50");
        match result {
            Ok(s) => assert_eq!(s, "L"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("100");
        match result {
            Ok(s) => assert_eq!(s, "C"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("500");
        match result {
            Ok(s) => assert_eq!(s, "D"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("1000");
        match result {
            Ok(s) => assert_eq!(s, "M"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("3030");
        match result {
            Ok(s) => assert_eq!(s, "MMMXXX"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("3038");
        match result {
            Ok(s) => assert_eq!(s, "MMMXXXVIII"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("0");
        match result {
            Ok(s) => assert_eq!(s, ""),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("-1");
        match result {
            Ok(s) => assert_eq!(s, ""),
            Err(e) => panic!("{:#?}", e),
        }
        let result = arabic2roman("foo");
        match result {
            Ok(s) => panic!("{:#?}", s),
            Err(e) => match e {
                ConversionError::NumberConversion(s) => {
                    assert_eq!(s, "Cannot convert foo to a number");
                }
                _ => panic!("{:#?}", e),
            },
        }
    }

    #[test]
    fn test_roman2arabic() {
        let result = roman2arabic("I");
        match result {
            Ok(s) => assert_eq!(s, "1"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("II");
        match result {
            Ok(s) => assert_eq!(s, "2"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("III");
        match result {
            Ok(s) => assert_eq!(s, "3"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("IV");
        match result {
            Ok(s) => assert_eq!(s, "4"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("V");
        match result {
            Ok(s) => assert_eq!(s, "5"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("VI");
        match result {
            Ok(s) => assert_eq!(s, "6"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("X");
        match result {
            Ok(s) => assert_eq!(s, "10"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("XI");
        match result {
            Ok(s) => assert_eq!(s, "11"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("XX");
        match result {
            Ok(s) => assert_eq!(s, "20"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("L");
        match result {
            Ok(s) => assert_eq!(s, "50"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("C");
        match result {
            Ok(s) => assert_eq!(s, "100"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("D");
        match result {
            Ok(s) => assert_eq!(s, "500"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("M");
        match result {
            Ok(s) => assert_eq!(s, "1000"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("MMMXXX");
        match result {
            Ok(s) => assert_eq!(s, "3030"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("MMMXXXVIII");
        match result {
            Ok(s) => assert_eq!(s, "3038"),
            Err(e) => panic!("{:#?}", e),
        }
        let result = roman2arabic("LMAOYOLOCOPTER");
        match result {
            Ok(s) => panic!("{:#?}", s),
            Err(e) => match e {
                ConversionError::NumberConversion(s) => {
                    assert_eq!(s, "Cannot convert LMAOYOLOCOPTER to a number");
                }
                _ => panic!("{:#?}", e),
            },
        }
    }

    #[test]
    fn test_to_ordinal() {
        assert_eq!(to_ordinal("1"), "1st");
        assert_eq!(to_ordinal("2"), "2nd");
        assert_eq!(to_ordinal("3"), "3rd");
        assert_eq!(to_ordinal("4"), "4th");
        assert_eq!(to_ordinal("10"), "10th");
        assert_eq!(to_ordinal("0"), "0th");
        assert_eq!(to_ordinal("-1"), "-1st");
        assert_eq!(to_ordinal("0.1"), "0.1");
        assert_eq!(to_ordinal("a"), "a");
        assert_eq!(to_ordinal("10001"), "10001st");
        assert_eq!(to_ordinal("10002"), "10002nd");
        assert_eq!(to_ordinal("10003"), "10003rd");
        assert_eq!(to_ordinal("10004"), "10004th");
    }
}
