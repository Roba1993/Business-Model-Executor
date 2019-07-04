
use crate::blocks::float::Float;
use bme_macro::ExecutionBlockHelper;
#[derive(Debug, PartialEq, Clone)]
pub struct FloatVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl crate::types::ExecutionType for FloatVector3 {
    fn get_name(&self) -> &'static str {
        "FloatVector3"
    }

    fn from_json(&self, _json: serde_json::Value) -> Box<crate::types::ExecutionType> {
        Box::new(FloatVector3::new(0.0, 0.0, 0.0))
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

impl FloatVector3 {
    pub fn new(x: Float, y: Float, z: Float) -> FloatVector3 {
        FloatVector3 { x, y, z }
    }
}

pub fn add_blocks(logic: &mut crate::Logic) {
    logic.add_block(Box::new(CreateFloatVector3 {}));
    logic.add_block(Box::new(SplitFloatVector3 {}));
}

ExecutionBlockHelper!(
    id: 67000,
    name: CreateFloatVector3,
    typ: Static,
    path: crate,

    fn execute(x: Float, y: Float, z: Float) -> (FloatVector3) {
        (FloatVector3 {x, y, z})
    }
);

/*
ExecutionBlockHelper!(
    id: 67001,
    name: FloatVector3Add,
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
);*/

ExecutionBlockHelper!(
    id: 67009,
    name: SplitFloatVector3,
    typ: Static,
    path: crate,

    fn execute(inp: FloatVector3) -> (Float, Float, Float) {
        (inp.x, inp.y, inp.z)
    }
);