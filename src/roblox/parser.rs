extern crate serde_json;

use self::serde_json::Value;
use std::collections::HashSet;
use roblox::api::*;

// Helper macros since we asume JSON decoded fine

macro_rules! get {($v:expr, $k:expr) => { $v.get($k).unwrap() }}

macro_rules! as_vec {($v:expr) => { $v.as_array().unwrap() }}

macro_rules! as_str {($v:expr) => { $v.as_str().unwrap() }}

macro_rules! own_str {($v:expr, $k:expr) => {as_str!(get!($v, $k)).to_owned()} }

fn parse_tags(val: &Value) -> HashSet<String> {
    let mut tags = HashSet::new();

    for tag in as_vec!(get!(val, "tags")) {
        tags.insert(as_str!(tag).to_owned());
    }

    tags
}

fn parse_class(api: &mut RobloxApi, class: &Value) {
    let name = own_str!(class, "Name");
    let tags = parse_tags(class);
    let superclass: Option<String> = if name=="Instance" {None} else {Some(own_str!(class, "Superclass"))};

    api.add_instance(Instance::new(name, tags, superclass));
}

fn parse_value_type(prop: &Value) -> PropertyType {
    let vt = as_str!(get!(prop, "ValueType"));
    match vt {
        "float" | "int" | "double" => PropertyType::Number,
        "bool" => PropertyType::Boolean,
        "Color3" => PropertyType::Color,
        "string" | "Content" => PropertyType::String,
        "UDim2" => PropertyType::UDim2,
        _ => PropertyType::Other(vt.to_owned())
    }
}

fn parse_property(api: &mut RobloxApi, prop: &Value) {
    let name = own_str!(prop, "Name");
    let tags = parse_tags(prop);
    let val_type = parse_value_type(prop);

    let class = as_str!(get!(prop, "Class"));

    api.get_instance(class).unwrap().add_property(
        Property::new(name, tags, val_type)
    )
}

fn parse_enum(api: &mut RobloxApi, enm: &Value) {
    let name = own_str!(enm, "Name");
    let tags = parse_tags(enm);

    api.add_enum(Enum::new(name, tags));
}

fn parse_enum_item(api: &mut RobloxApi, member: &Value) {
    let belongs_to = as_str!(get!(member, "Enum"));

    api.get_enum(belongs_to).unwrap()
        .add_member(own_str!(member, "Name"));
}

fn parse_api_blob(api: &mut RobloxApi, blob: &Value) {
    match as_str!(get!(blob, "type")) {
        "Class" => parse_class(api,blob),
        "Property" => parse_property(api, blob),
        "Enum" => parse_enum(api, blob),
        "EnumItem" => parse_enum_item(api, blob),
        _ => {}
    }
}

pub fn load_api(data: &str) -> RobloxApi {
    let mut api = RobloxApi::new();

    let val: Value = serde_json::from_str(data).unwrap();

    for blob in as_vec!(val) {
        parse_api_blob(&mut api, blob);
    }

    api
}