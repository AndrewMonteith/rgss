mod atoms {
    include!(concat!(env!("OUT_DIR"), "./rg_grammar.rs"));
}

pub type ParseError = atoms::ParseError;

use rg::values::RgNode;

pub fn parse_str(contents: &str) -> Result<RgNode, ParseError> {
    atoms::instance(contents)
}


#[cfg(test)]
mod can_parse {
    use rg::values::{Value, RgProperty as Property, RgNode};
    use super::atoms;

    fn check_equality_of_value(parsed: Result<Value, atoms::ParseError>, expected: Value) {
        match parsed {
            Ok(v) => assert_eq!(v, expected),
            Err(e) => panic!(format!("Expected result {:?} but got result {:?}", expected, e))
        }
    }

    mod strings_that {
        use super::*;

        macro_rules! successful_string_test {
            ($test_name:ident, $text: expr) => {
                #[test]
                fn $test_name() {
                    check_equality_of_value(atoms::string_literal(format!("\"{}\"", $text).as_ref()),
                        Value::StringLiteral(String::from($text)))
                }
            }
        }

        macro_rules! failed_string_test {
            ($test_name:ident, $text: expr) => {
                #[test]
                #[should_panic]
                fn $test_name() {
                    atoms::string_literal($text).unwrap();
                }
            }
        }

        successful_string_test!(empty, "");
        successful_string_test!(are_scaled_double_quote, "\\\"");
        successful_string_test!(are_escaped_single_quote, "'");
        successful_string_test!(are_random_charaters, "Hi t2..h'ere 123 ]d]sd['");

        failed_string_test!(unclosed_string_literal, "\"");
        failed_string_test!(unknown_escaped_character, "\\j");
    }

    mod numbers_that {
        use super::*;

        macro_rules! successful_number_test {
            ($test_name:ident, $number: expr) => {
                #[test]
                fn $test_name() {
                    check_equality_of_value(atoms::number($number.to_string().as_ref()),
                        Value::Number($number as f64))
                }
            }
        }

        macro_rules! failed_number_test {
            ($test_name: ident, $number_str: expr) => {
                #[test] #[should_panic]
                fn $test_name() {
                    atoms::number($number_str).unwrap();
                }
            }
        }

        successful_number_test!(is_zero, 0);
        successful_number_test!(positive_integer, 12);
        successful_number_test!(negative_integer, 12);
        successful_number_test!(positive_decimal, 1.25);
        successful_number_test!(negative_decimal, -1.53239054);

        failed_number_test!(no_mantissa, "1.");
        failed_number_test!(weird_character, "12a.3");
    }

    mod udim2_that {
        use super::*;

        macro_rules! successful_udim2_test {
            ($test_name: ident, $udim_str:expr, $xsc:expr, $xoff:expr, $ysc:expr, $yoff:expr) => {
                #[test]
                fn $test_name() {
                    check_equality_of_value(atoms::udim2($udim_str),
                        Value::UDim2($xsc as f64, $xoff as f64, $ysc as f64, $yoff as f64));
                }
            }
        }

        successful_udim2_test!(at_origin, "UDim2(0,0,0,0)", 0,0,0,0);
        successful_udim2_test!(all_numbers, "UDim2(0.75, 200, -0.5, -20)", 0.75, 200, -0.5, -20);
        successful_udim2_test!(with_padding, "   UDim2 ( 100 , 0.2 , -25 ,  1.4 ) ", 100, 0.2, -25, 1.4);
    }

    mod booleans_that {
        use super::*;

        #[test]
        fn is_true() {
            check_equality_of_value(atoms::boolean("true"), Value::Boolean(true));
        }

        #[test]
        fn is_false() {
            check_equality_of_value(atoms::boolean("false"), Value::Boolean(false));
        }
    }

    mod colors {
        use super::*;
        macro_rules! col {
            ($r: expr, $g: expr, $b: expr) => { Value::Color($r, $g, $b) }
        }

        mod hex_that {
            use super::*;

            macro_rules! test_hex_col {
                ($test_name: ident, $hex: expr, $col: expr) => {
                    #[test]
                    fn $test_name() {
                        check_equality_of_value(atoms::hex_color($hex), $col)
                    }
                }
            }

            test_hex_col!(is_white, "#FFFFFF", col!(255, 255, 255));
            test_hex_col!(is_black, "#000000", col!(0,0,0));
            test_hex_col!(is_red, "#FF0000", col!(255, 0, 0));
            test_hex_col!(is_blue, "#00FF00", col!(0, 255, 0));
            test_hex_col!(is_green, "#0000FF", col!(0, 0, 255));

            #[test]
            #[should_panic]
            fn hex_digit_out_of_range() {
                atoms::hex_color("#G0FF00").unwrap();
            }
        }

        mod rgb_that {
            use super::*;

            macro_rules! test_rgb_col {
                ($test_name: ident, $rgb_text:expr, $col:expr) => {
                    #[test]
                    fn $test_name() {
                        check_equality_of_value(atoms::rgb_color($rgb_text), $col)
                    }
                }
            }

            test_rgb_col!(is_black, "RGB(0,0,0)", col!(0,0,0));
            test_rgb_col!(is_red, "RGB(255,0,0)", col!(255, 0, 0));
            test_rgb_col!(is_green, "RGB(0,255,0)", col!(0, 255, 0));
            test_rgb_col!(is_blue, "RGB(0,0,255)", col!(0, 0, 255));
            test_rgb_col!(is_white, "RGB(255,255,255)", col!(255, 255, 255));
            test_rgb_col!(padding_before_first_bracket, "RGB (0,0,0)", col!(0,0,0));
            test_rgb_col!(padding_before_first_comma, "RGB (0 ,0,0)", col!(0,0,0));
            test_rgb_col!(padding_after_first_comma, "RGB (0 , 0,0)", col!(0,0,0));
            test_rgb_col!(padding_before_second_comma, "RGB (0 , 0  ,0)", col!(0,0,0));
            test_rgb_col!(padding_after_second_comma, "RGB (0 , 0  ,  0)", col!(0,0,0));
            test_rgb_col!(padding_before_end_bracket, "RGB (0 , 0  ,  0  )", col!(0,0,0));
            test_rgb_col!(padding_after_end_bracket, "RGB (0 , 0  ,  0  )  ", col!(0,0,0));

            #[test]
            #[should_panic]
            fn negative_rgb_vals() {
                atoms::rgb_color("RGB(-10, 0, 0)").unwrap();
            }
        }
    }

    mod assignments_that {
        use super::*;

        macro_rules! test_property_assignment {
            ($test_name: ident, $text_to_parse:expr, $expected_prop:expr, $expected_val:expr) => {
                #[test]
                fn $test_name() {
                    let prop = atoms::prop_assignment($text_to_parse).unwrap();

                    assert_eq!(prop, RgNode::Property(Property::new($expected_prop.to_string(), $expected_val)));
                }
            }
        }

        macro_rules! test_property_assignment_fail {
            ($test_name: ident, $text_to_parse:expr) => {
                #[test] #[should_panic]
                fn $test_name() {
                    atoms::prop_assignment($text_to_parse).unwrap();
                }
            }
        }

        test_property_assignment!(has_number_value, "BackgroundTransparency:0",
                "BackgroundTransparency", Value::Number(0f64));

        test_property_assignment!(has_boolean_value, "Visible:false",
                "Visible", Value::Boolean(false));

        test_property_assignment!(has_rgb_value, "TextColor3:RGB(255,255,255)",
                "TextColor3", Value::Color(255, 255, 255));

        test_property_assignment!(has_hex_value, "TextColor3:#FF00FF",
                "TextColor3", Value::Color(255, 0, 255));

        test_property_assignment!(has_string_value, "Text:\"Hi there\"",
            "Text", Value::StringLiteral("Hi there".to_owned()));

        test_property_assignment!(padding_between_text, "   Text: \n \" Hi \n there \"   ",
            "Text", Value::StringLiteral(" Hi \n there ".to_owned()));

        test_property_assignment_fail!(no_value, "Text:");
        test_property_assignment_fail!(no_prop, ":Hi");
    }

    mod code_fragments {
        use super::*;

        macro_rules! should_compile {
            ($test_name: ident, $contents: expr) => {
                #[test]
                fn $test_name() {
                    let parsed = atoms::file($contents);

                    if !parsed.is_ok() {
                        panic!(format!("Expected success but got {:?}", parsed));
                    }
                }
            }
        }

        should_compile!(empty_instance, "Instance {}");
        should_compile!(nested_empty, "Instance {Instance{}}");
        should_compile!(welcome_gui, r#"ScreenGui {
    Name: "WelcomeGui"
    Enabled: true

    TextLabel {
        Text: "Welcome"
        BackgroundColor3: #FF00FF
        TextStrokeTransparency: 0.2
        BorderSizePixel: 0
    }

    TextButton "Continue" {
        Text: "Continue"
        BackgroundTransparency: 1
        TextColor3: RGB(255, 20, 120)
    }

    Frame{}
}"#);
    }
}