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
}
downcast_rs::impl_downcast!(ExecutionType);

impl ExecutionType for String {
    fn get_name(&self) -> &'static str {
        "String"
    }

    fn from_json(&self, json: serde_json::Value) -> Box<ExecutionType> {
        let s = serde_json::from_value::<String>(json);

        if let Ok(s) = s {
            return Box::new(s);
        }

        Box::new(String::new())
    }

    fn duplicate(&self) -> Box<ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "green"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        Some("")
    }
}

impl ExecutionType for i64 {
    fn get_name(&self) -> &'static str {
        "i64"
    }

    fn from_json(&self, json: serde_json::Value) -> Box<ExecutionType> {
        use std::str::FromStr;

        if let Ok(s) = serde_json::from_value::<i64>(json.clone()) {
            return Box::new(s);
        }

        if let Ok(s) = serde_json::from_value::<String>(json) {
            if let Ok(f) = i64::from_str(&s) {
                return Box::new(f);
            }
        }

        Box::new(0i64)
    }

    fn duplicate(&self) -> Box<ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "lightgreen"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        Some("0")
    }
}

impl ExecutionType for f64 {
    fn get_name(&self) -> &'static str {
        "f64"
    }

    fn from_json(&self, json: serde_json::Value) -> Box<ExecutionType> {
        use std::str::FromStr;

        if let Ok(s) = serde_json::from_value::<f64>(json.clone()) {
            return Box::new(s);
        }

        if let Ok(s) = serde_json::from_value::<String>(json) {
            if let Ok(f) = f64::from_str(&s) {
                return Box::new(f);
            }
        }

        Box::new(0.0f64)
    }

    fn duplicate(&self) -> Box<ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "darkgreen"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        Some("0.0")
    }
}

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
