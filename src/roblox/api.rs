use std::collections::HashSet;
use rg::Value;

pub struct RobloxApi {
    instances: Vec<Instance>,
    enums: Vec<Enum>,
}

impl RobloxApi {
    pub fn new() -> RobloxApi {
        RobloxApi { instances: vec![], enums: vec![] }
    }

    pub fn add_instance(&mut self, inst: Instance) {
        self.instances.push(inst)
    }

    pub fn add_enum(&mut self, enm: Enum) {
        self.enums.push(enm)
    }

    pub fn get_instance(&mut self, name: &str) -> Option<&mut Instance> {
        self.instances.iter_mut()
            .find(|inst| get_name(*inst) == name)
    }

    pub fn has_instance(&self, name: &str) -> bool {
        self.instances.iter()
            .find(|inst| get_name(*inst) == name)
            .is_some()
    }

    pub fn get_enum(&mut self, name: &str) -> Option<&mut Enum> {
        self.enums.iter_mut()
            .find(|enm| get_name(*enm) == name)
    }

    pub fn has_enum(&self, name: &str) -> bool {
        self.instances.iter()
            .find(|enm| get_name(*enm) == name)
            .is_some()
    }
}

struct ApiCore {
    name: String,
    tags: HashSet<String>,
}

trait RobloxCore {
    fn get_core(&self) -> &ApiCore;
}

fn get_name<T: RobloxCore>(api: &T) -> &str {
    &api.get_core().name
}

fn has_tag<T: RobloxCore>(api: &T, tag: &str) -> bool {
    api.get_core().tags.contains(tag)
}

macro_rules! impl_roblox_core {
    ($s: ident) => {
        impl RobloxCore for $s {
            fn get_core(&self) -> &ApiCore { &self.core }
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
    core: ApiCore,
    val_type: PropertyType,
}

impl_roblox_core!(Property);
impl Property {
    pub fn new(name: String, tags: HashSet<String>, val_type: PropertyType) -> Property {
        Property { core: ApiCore { name, tags }, val_type}
    }

    fn can_take_value(&self, val: &Value) -> bool {
        match &self.val_type {
            PropertyType::String => val.get_type() == "String",
            PropertyType::Number => val.get_type() == "Number",
            PropertyType::Boolean => val.get_type() == "Boolean",
            PropertyType::Color => val.get_type() == "Color",
            PropertyType::UDim2 => val.get_type() == "UDim2",
            PropertyType::Other(enum_member) => panic!("Still need to do enum members")
        }
    }
}

pub struct Instance {
    core: ApiCore,
    superclass: Option<String>,
    properties: Vec<Property>,
}

impl_roblox_core!(Instance);
impl Instance {
    pub fn new(name: String, tags: HashSet<String>, superclass: Option<String>) -> Instance {
        Instance { core: ApiCore { name, tags }, superclass, properties: vec![] }
    }

    pub fn add_property(&mut self, prop: Property) {
        self.properties.push(prop)
    }

    pub fn has_property(&self, prop: &str) -> bool {
        self.get_property(prop).is_some()
    }

    fn get_property(&self, prop: &str) -> Option<&Property> {
        self.properties.iter()
            .find(|p| get_name(*p) == prop)
    }
}

pub struct Enum {
    core: ApiCore,
    members: HashSet<String>,
}

impl_roblox_core!(Enum);
impl Enum {
    pub fn new(name: String, tags: HashSet<String>) -> Enum {
        Enum { core: ApiCore { name, tags }, members: HashSet::new() }
    }

    pub fn add_member(&mut self, member: String) {
        self.members.insert(member);
    }
}

