use bme_macro::ExecutionBlockHelper;
use crate::blocks::integer::Integer;

impl crate::types::ExecutionType for String {
    fn get_name(&self) -> &'static str {
        "String"
    }

    fn from_json(&self, json: serde_json::Value) -> Box<crate::types::ExecutionType> {
        let s = serde_json::from_value::<String>(json);

        if let Ok(s) = s {
            return Box::new(s);
        }

        Box::new(String::new())
    }

    fn duplicate(&self) -> Box<crate::types::ExecutionType> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> &'static str {
        "green"
    }

    fn get_edit_default(&self) -> Option<&'static str> {
        Some("Text")
    }

    fn get_multi_output(&self) -> bool {
        true
    }
}

pub fn add_blocks(logic: &mut crate::Logic) {
    logic.add_block(Box::new(StringAdd {}));
    logic.add_block(Box::new(StringLength {}));
    logic.add_block(Box::new(StringTrim {}));
    logic.add_block(Box::new(StringToLowercase {}));
    logic.add_block(Box::new(StringToUppercase {}));
    logic.add_block(Box::new(StringInsert {}));
}

ExecutionBlockHelper!(
    id: 68001,
    name: StringAdd,
    typ: Static,
    path: crate,

    fn execute(inp1: String, inp2: String) -> (String) {
        (format!("{}{}", inp1, inp2))
    }
);

ExecutionBlockHelper!(
    id: 68002,
    name: StringLength,
    typ: Static,
    path: crate,

    fn execute(inp1: String) -> (Integer) {
        (inp1.len() as Integer)
    }
);

ExecutionBlockHelper!(
    id: 68003,
    name: StringTrim,
    typ: Static,
    path: crate,

    fn execute(inp1: String) -> (String) {
        (inp1.trim().to_string())
    }
);

ExecutionBlockHelper!(
    id: 68004,
    name: StringToLowercase,
    typ: Static,
    path: crate,

    fn execute(inp1: String) -> (String) {
        (inp1.to_lowercase())
    }
);

ExecutionBlockHelper!(
    id: 68005,
    name: StringToUppercase,
    typ: Static,
    path: crate,

    fn execute(inp1: String) -> (String) {
        (inp1.to_uppercase())
    }
);

ExecutionBlockHelper!(
    id: 68006,
    name: StringInsert,
    typ: Static,
    path: crate,

    fn execute(inp1: String, index: Integer, inp2: String) -> (String) {
        let mut inp1 = inp1;
        let mut index = index as usize;
        
        if(index > inp1.len()) {
            index = inp1.len();
        }

        inp1.insert_str(index, &inp2);
        (inp1)
    }
);