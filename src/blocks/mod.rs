pub mod integer;
pub mod float;
pub mod float_vec3;
pub mod string;

use bme_macro::ExecutionBlockHelper;

pub fn add_blocks(logic: &mut crate::Logic) {
    logic.add_block(Box::new(Comment {}));
    logic.add_block(Box::new(ConsolePrint {}));

    integer::add_blocks(logic);
    float::add_blocks(logic);
    float_vec3::add_blocks(logic);
    string::add_blocks(logic);
}

ExecutionBlockHelper!(
    id: 60_000,
    name: Comment,
    typ: Comment,
    path: crate,

    fn execute() -> () {
        ()
    }
);

ExecutionBlockHelper!(
    id: 60_001,
    name: ConsolePrint,
    typ: Normal,
    path: crate,

    fn execute(inp: String) -> () {
        if cfg!(target_arch = "wasm32") {
            web_sys::console::log_1(&format!("#> {}", inp).into());
        }
        else {
            println!("#> {}", inp);
        }
        ()
    }
);