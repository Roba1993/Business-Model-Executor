pub trait ExecutionType: downcast_rs::Downcast + std::fmt::Debug {
    fn get_name(&self) -> &'static str;

    fn from_json(&self, json: serde_json::Value) -> Box<ExecutionType>;

    fn duplicate(&self) -> Box<ExecutionType>;

    fn get_color(&self) -> &'static str {
        "blue"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        None
    }

    fn get_multi_output(&self) -> bool {
        false
    }
}
downcast_rs::impl_downcast!(ExecutionType);


#[derive(Debug, PartialEq, Clone)]
pub struct Execution {}

impl Execution {
    pub fn new() -> Execution {
        Execution {}
    }
}

impl ExecutionType for Execution {
    fn get_name(&self) -> &'static str {
        "Execution"
    }

    fn from_json(&self, _json: serde_json::Value) -> Box<ExecutionType> {
        Box::new(Execution {})
    }

    fn duplicate(&self) -> Box<ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "dark"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        None
    }
}
