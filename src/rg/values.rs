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