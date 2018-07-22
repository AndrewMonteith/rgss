#[derive(PartialEq, Debug)]
pub enum RgNode {
    Property(RgProperty),
    Instance(RgInstance),
}

#[derive(PartialEq, Debug)]
pub struct RgProperty {
    name: String,
    value: Value,
}

#[derive(PartialEq, Debug)]
pub struct RgInstance {
    properties: Vec<RgProperty>,
    children: Vec<RgInstance>,
}

impl RgProperty {
    pub fn new(name: String, value: Value) -> RgProperty {
        RgProperty { name, value }
    }
}

impl RgInstance {
    pub fn new(properties: Vec<RgProperty>, children: Vec<RgInstance>) -> RgInstance {
        RgInstance { properties, children }
    }
}

#[derive(PartialEq, Debug)]
pub enum Value {
    StringLiteral(String),
    Number(f64),
    Boolean(bool),
    Color(i64, i64, i64),
    UDim2(f64, f64, f64, f64),
}

impl Value {
    pub fn new_string(s: &str) -> Value {
        Value::StringLiteral(s.to_owned())
    }

    pub fn new_number(f: f64) -> Value {
        Value::Number(f)
    }

    pub fn new_boolean(s: &str) -> Value {
        Value::Boolean(s == "true")
    }

    pub fn new_hex_color(s: &str) -> Value {
        let rgb = s.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|v: &[char]| i64::from_str_radix(v.iter().collect::<String>().as_ref(), 16).unwrap())
            .collect::<Vec<i64>>();

        return Value::new_rgb_col(rgb);
    }

    pub fn new_rgb_col(v: Vec<i64>) -> Value {
        Value::Color(v[0], v[1], v[2])
    }

    pub fn new_udim2(v: Vec<f64>) -> Value {
        Value::UDim2(v[0], v[1], v[2], v[3])
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Value::StringLiteral(_) => "String",
            Value::Number(_) => "Number",
            Value::Boolean(_) => "Boolean",
            Value::Color(_, _, _) => "Color",
            Value::UDim2(_, _, _, _) => "UDim2"
        }
    }
}