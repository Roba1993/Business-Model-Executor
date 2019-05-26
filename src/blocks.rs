use bme_macro::ExecutionBlock;

ExecutionBlock!(
    id: 1,
    name: Start,
    typ: Start,
    path: crate,

    fn execute() -> () {
        println!("#Start>");
        ()
    }
);


ExecutionBlock!(
    id: 2,
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

ExecutionBlock!(
    id: 3,
    name: StaticString,
    typ: Static,
    path: crate,

    fn execute(inp: String) -> (String) {
        (inp)
    }
);

ExecutionBlock!(
    id: 4,
    name: AddString,
    typ: Static,
    path: crate,

    fn execute(inp1: String, inp2: String) -> (String) {
        (format!("{}{}", inp1, inp2))
    }
);

ExecutionBlock!(
    id: 5,
    name: AddInteger,
    typ: Static,
    path: crate,

    fn execute(inp1: i64, inp2: i64) -> (i64) {
        (inp1 + inp2)
    }
);

ExecutionBlock!(
    id: 6,
    name: IntegerToString,
    typ: Static,
    path: crate,

    fn execute(inp: i64) -> (String) {
        (inp.to_string())
    }
);