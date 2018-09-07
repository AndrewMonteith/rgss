use roblox::{RobloxApi, initalise as get_roblox_api};
use rg::values::{RgInstance, RgProperty};

pub struct SemanticsChecker {
    api: RobloxApi
}

pub type SemanticsResult = Result<(), String>;

fn get_string_val<'a>(inst: &'a RgInstance, val: &'a str) -> &'a str {
    inst.get_prop_value(val).unwrap().as_str()
}

macro_rules! check {
    ($cond: expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            return Err(format!(concat!($fmt, "\n"), $($arg)*));
        }
    }
}

impl SemanticsChecker {
    pub fn new() -> Result<SemanticsChecker, &'static str> {
        let api = get_roblox_api()?;

        Ok(SemanticsChecker { api })
    }

    pub fn check_instance(&self, inst: &RgInstance) -> SemanticsResult {
        self.check_instance_semantics(inst)?;

        for prop in inst.properties() {
            self.check_property_semantics(inst, prop)?;
        }

        for child in inst.children() {
            self.check_instance(child)?;
        }

        Ok(())
    }

    fn check_instance_semantics(&self, inst: &RgInstance) -> SemanticsResult {
        let class_name = get_string_val(inst, "_ClassName");

        let instance = self.api.get_instance(class_name);

        check!(instance.is_some(),
            "Class {} does not exist",
            class_name);

        let roblox_instance = instance.unwrap();

        check!(!roblox_instance.has_tag("notCreatable"),
            "Instance {} cannot be created",
            roblox_instance.get_name());

        Ok(())
    }

    fn check_property_semantics(&self, inst: &RgInstance, prop: &RgProperty) -> SemanticsResult {
        if prop.get_name() == "_ClassName" {
            return Ok(());
        }

        let roblox_instance = self.api.get_instance(
            get_string_val(inst, "_ClassName")).unwrap();

        let roblox_prop_opt = self.api.get_property(
            roblox_instance, prop.get_name());

        check!(roblox_prop_opt.is_some(),
            "Class {} with name {} does not have property {}",
            get_string_val(inst, "_ClassName"),
            get_string_val(inst, "Name"), 
            prop.get_name());

        let roblox_prop = roblox_prop_opt.unwrap();
        check!(roblox_prop.can_take_value(&self.api, prop.get_value()),
            "Property {} cannot be set with value {:?}",
            roblox_prop.get_name(),
            prop.get_value());

        check!(!roblox_prop.has_tag("readonly"),
            "The property {} is readonly",
            roblox_prop.get_name());

        Ok(())
    }
}