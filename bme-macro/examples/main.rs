use bme_macro::make_answer;

make_answer!(
    id: 4,
    name: AddString,
    description: "Adding two Strings",
    typ: Static,

    fn execute(inp1: String, inp2: String) -> (String) {
        println!("Inputs {}, {}", inp1, inp2);

        (String::from("Result"))
    }
);

fn main() {
    println!("{}", answer());

    
}