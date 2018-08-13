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
    _properties: Vec<RgProperty>,
    _children: Vec<RgInstance>,
}

impl RgProperty {
    pub fn new(name: String, value: Value) -> RgProperty {
        RgProperty { name, value }
    }

    pub fn new_string(name: String, value: String) -> RgProperty {
        RgProperty {name, value: Value::StringLiteral(value)}
    }

    pub fn get_name(&self) -> &str  {
        &self.name
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }
}

impl RgInstance {
    pub fn new(_properties: Vec<RgProperty>, _children: Vec<RgInstance>) -> RgInstance {
        RgInstance { _properties, _children }
    }

    pub fn children(&self) -> &Vec<RgInstance> {
        &self._children
    }
    
    pub fn properties(&self) -> &Vec<RgProperty> {
        &self._properties
    }

    pub fn get_prop_value(&self, name: &str) -> Option<&Value> {
        self.properties().iter()
            .find(|prop| prop.name == name)   
            .map(|prop| &prop.value)
    }
}

#[derive(PartialEq, Debug)]
pub enum Value {
    StringLiteral(String),
    Number(f64),
    Boolean(bool),
    Color(i64, i64, i64),
    UDim2(f64, f64, f64, f64),
    EnumMember(String),
}

impl Value {
   pub fn get_type(&self) -> &'static str {
        match self {
            Value::StringLiteral(_) => "String",
            Value::Number(_) => "Number",
            Value::Boolean(_) => "Boolean",
            Value::Color(_, _, _) => "Color",
            Value::UDim2(_, _, _, _) => "UDim2",
            Value::EnumMember(_) => "EnumMember",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::StringLiteral(ref val) => val,
            _ => panic!("Not a string {:?}", self),
        }
    }
}