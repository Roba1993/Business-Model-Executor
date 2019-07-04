use bme_macro::ExecutionBlockHelper;

pub type Integer = i64;

impl crate::types::ExecutionType for Integer {
    fn get_name(&self) -> &'static str {
        "Integer"
    }

    fn from_json(&self, json: serde_json::Value) -> Box<crate::types::ExecutionType> {
        use std::str::FromStr;

        if let Ok(s) = serde_json::from_value::<Integer>(json.clone()) {
            return Box::new(s);
        }

        if let Ok(s) = serde_json::from_value::<String>(json) {
            if let Ok(f) = Integer::from_str(&s) {
                return Box::new(f);
            }
        }

        Box::new(0i64)
    }

    fn duplicate(&self) -> Box<crate::types::ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "lightgreen"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        Some("0")
    }

    fn get_multi_output(&self) -> bool {
        true
    }
}

pub fn add_blocks(logic: &mut crate::Logic) {
    logic.add_block(Box::new(IntegerAdd {}));
    logic.add_block(Box::new(IntegerSubtract {}));
    logic.add_block(Box::new(IntegerMultiply {}));
    logic.add_block(Box::new(IntegerDivide {}));
    logic.add_block(Box::new(IntegerToString {}));
}

ExecutionBlockHelper!(
    id: 64001,
    name: IntegerAdd,
    typ: Static,
    path: crate,

    fn execute(inp1: Integer, inp2: Integer) -> (Integer) {
        (inp1 + inp2)
    }
);

ExecutionBlockHelper!(
    id: 64002,
    name: IntegerSubtract,
    typ: Static,
    path: crate,

    fn execute(inp1: Integer, inp2: Integer) -> (Integer) {
        (inp1 - inp2)
    }
);

ExecutionBlockHelper!(
    id: 64003,
    name: IntegerMultiply,
    typ: Static,
    path: crate,

    fn execute(inp1: Integer, inp2: Integer) -> (Integer) {
        (inp1 * inp2)
    }
);

ExecutionBlockHelper!(
    id: 64004,
    name: IntegerDivide,
    typ: Static,
    path: crate,

    fn execute(inp1: Integer, inp2: Integer) -> (Integer) {
        (inp1 / inp2)
    }
);

ExecutionBlockHelper!(
    id: 64005,
    name: IntegerToString,
    typ: Static,
    path: crate,

    fn execute(inp1: Integer) -> (String) {
        (inp1.to_string())
    }
);