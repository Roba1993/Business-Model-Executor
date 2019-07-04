use bme_macro::ExecutionBlockHelper;

pub type Float = f64;

impl crate::types::ExecutionType for Float {
    fn get_name(&self) -> &'static str {
        "Float"
    }

    fn from_json(&self, json: serde_json::Value) -> Box<crate::types::ExecutionType> {
        use std::str::FromStr;

        if let Ok(s) = serde_json::from_value::<Float>(json.clone()) {
            return Box::new(s);
        }

        if let Ok(s) = serde_json::from_value::<String>(json) {
            if let Ok(f) = Float::from_str(&s) {
                return Box::new(f);
            }
        }

        Box::new(0.0f64)
    }

    fn duplicate(&self) -> Box<crate::types::ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "darkgreen"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        Some("0.0")
    }

    fn get_multi_output(&self) -> bool {
        true
    }
}

pub fn add_blocks(logic: &mut crate::Logic) {
    logic.add_block(Box::new(FloatAdd {}));
    logic.add_block(Box::new(FloatSubtract {}));
    logic.add_block(Box::new(FloatMultiply {}));
    logic.add_block(Box::new(FloatDivide {}));
    logic.add_block(Box::new(FloatToString {}));
}


ExecutionBlockHelper!(
    id: 67001,
    name: FloatAdd,
    typ: Static,
    path: crate,

    fn execute(inp1: Float, inp2: Float) -> (Float) {
        (inp1 + inp2)
    }
);

ExecutionBlockHelper!(
    id: 67002,
    name: FloatSubtract,
    typ: Static,
    path: crate,

    fn execute(inp1: Float, inp2: Float) -> (Float) {
        (inp1 - inp2)
    }
);

ExecutionBlockHelper!(
    id: 67003,
    name: FloatMultiply,
    typ: Static,
    path: crate,

    fn execute(inp1: Float, inp2: Float) -> (Float) {
        (inp1 * inp2)
    }
);

ExecutionBlockHelper!(
    id: 67004,
    name: FloatDivide,
    typ: Static,
    path: crate,

    fn execute(inp1: Float, inp2: Float) -> (Float) {
        (inp1 / inp2)
    }
);

ExecutionBlockHelper!(
    id: 64005,
    name: FloatToString,
    typ: Static,
    path: crate,

    fn execute(inp1: Float) -> (String) {
        (inp1.to_string())
    }
);