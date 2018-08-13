use std::collections::HashSet;
use rg::Value;

pub struct RobloxApi {
    instances: Vec<Instance>,
    enums: Vec<Enum>,
}

impl RobloxApi {
    pub fn get_property<'a>(&'a self, inst: &'a Instance, prop: &'a str) -> Option<&Property> {
        inst.get_property(prop).or_else(|| {
            match inst.get_superclass() {
                Some(ref sp) => self.get_property(
                    self.get_instance(sp).unwrap(), prop),
                None => None,
            }
        })
    }

    pub fn add_instance(&mut self, inst: Instance) {
        self.instances.push(inst)
    }

    pub fn get_instance(&self, name: &str) -> Option<&Instance> {
        self.instances.iter()
            .find(|inst| inst.get_name() == name)
    }
    
    pub fn get_instance_mut(&mut self, name: &str) -> Option<&mut Instance> {
        self.instances.iter_mut()
            .find(|inst| inst.get_name() == name)
    }

    pub fn add_enum(&mut self, enm: Enum) {
        self.enums.push(enm)
    }

    pub fn get_enum(&self, name: &str) -> Option<&Enum> {
        self.enums.iter()
            .find(|enm| enm.get_name() == name)
    }

    pub fn get_enum_mut(&mut self, name: &str) -> Option<&mut Enum> {
        self.enums.iter_mut()
            .find(|enm| enm.get_name() == name)
    }

    pub fn new() -> RobloxApi {
        RobloxApi { instances: vec![], enums: vec![] }
    }
}

macro_rules! impl_base_methods {
    ($s: ident) => {
        impl $s {
            pub fn get_name<'a>(&'a self) -> &'a str { &self.name }

            pub fn has_tag(&self, tag: &str) -> bool {
                self.tags.contains(tag)
            }
        }
    }
}

pub enum PropertyType {
    String,
    Number,
    Boolean,
    Color,
    UDim2,
    Other(String), // can contain enum name
}

pub struct Property {
    name: String, tags: HashSet<String>,
    val_type: PropertyType,
}

impl_base_methods!(Property);
impl Property {
    pub fn new(name: String, tags: HashSet<String>, val_type: PropertyType) -> Property {
        Property { name, tags, val_type}
    }

    pub fn can_take_value<'a>(&'a self, api: &'a RobloxApi, val: &'a Value) -> bool {
        match &self.val_type {
            PropertyType::String => val.get_type() == "String",
            PropertyType::Number => val.get_type() == "Number",
            PropertyType::Boolean => val.get_type() == "Boolean",
            PropertyType::Color => val.get_type() == "Color",
            PropertyType::UDim2 => val.get_type() == "UDim2",
            PropertyType::Other(ref enum_name) => {
                let roblox_enum = api.get_enum(enum_name).unwrap();

                match val {
                    Value::StringLiteral(ref member) | Value::EnumMember(ref member)
                        => roblox_enum.has_member(member),
                    _ => false,
                }
            }
        }
    }
}

pub struct Instance {
    name: String, tags: HashSet<String>,
    superclass: Option<String>,
    properties: Vec<Property>,
}

impl_base_methods!(Instance);
impl Instance {
    pub fn new(name: String, tags: HashSet<String>, superclass: Option<String>) -> Instance {
        Instance { name, tags, superclass, properties: vec![] }
    }

    pub fn add_property(&mut self, prop: Property) {
        self.properties.push(prop)
    }

    fn get_property(&self, prop: &str) -> Option<&Property> {
        self.properties.iter()
            .find(|p| (*p).get_name() == prop)
    }

    pub fn get_superclass(&self) -> &Option<String> {
        &self.superclass
    }

}

pub struct Enum {
    name: String, tags: HashSet<String>,
    members: HashSet<String>,
}

impl_base_methods!(Enum);
impl Enum {
    pub fn new(name: String, tags: HashSet<String>) -> Enum {
        Enum { name, tags, members: HashSet::new() }
    }

    pub fn add_member(&mut self, member: String) {
        self.members.insert(member);
    }

    pub fn has_member(&self, member: &str) -> bool {
        self.members.contains(member)
    }
}

