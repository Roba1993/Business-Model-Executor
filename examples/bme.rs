use bme::{ConnectionType};
use std::collections::HashMap;

fn main() {
    // define the rule
    let mut logic = bme::Logic::new();
    logic.add_connection_type(
        bme::ConnectionType::Execution,
        "black".to_string(),
        false,
        "".to_string(),
        "".to_string(),
    );
    logic.add_connection_type(
        bme::ConnectionType::String,
        "purple".to_string(),
        true,
        "Text".to_string(),
        "".to_string(),
    );
    logic.add_connection_type(
        bme::ConnectionType::Integer,
        "green".to_string(),
        true,
        "0".to_string(),
        "".to_string(),
    );
    logic.add_block(Box::new(bme::Start {}));
    logic.add_block(Box::new(bme::ConsoleLog {}));
    logic.add_block(Box::new(bme::StaticString {}));

    println!(
        "{}",
        serde_json::to_string_pretty(&logic.get_json()).unwrap()
    );
    println!("");

    // parse the code from json to the objects
    let json = r#"[{"blockId":9,"blockTypeId":1,"position":{"x":240,"y":220},"nodes":[{"id":1,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":10,"connectedBlockTypeId":2,"connectedNodeId":1}]},{"blockId":10,"blockTypeId":2,"position":{"x":499,"y":204},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":9,"connectedBlockTypeId":1,"connectedNodeId":1},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":12,"connectedBlockTypeId":2,"connectedNodeId":1},{"id":3,"nodeType":"input","connectionType":"String","value":null,"connectedBlockId":11,"connectedBlockTypeId":3,"connectedNodeId":2}]},{"blockId":11,"blockTypeId":3,"position":{"x":237,"y":327},"nodes":[{"id":1,"nodeType":"input","connectionType":"String","value":"Hello World","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":2,"nodeType":"output","connectionType":"String","value":null,"connectedBlockId":10,"connectedBlockTypeId":2,"connectedNodeId":3}]},{"blockId":12,"blockTypeId":2,"position":{"x":699,"y":205},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":10,"connectedBlockTypeId":2,"connectedNodeId":2},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":14,"connectedBlockTypeId":2,"connectedNodeId":1},{"id":3,"nodeType":"input","connectionType":"String","value":"Text","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null}]},{"blockId":14,"blockTypeId":2,"position":{"x":911,"y":214},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":12,"connectedBlockTypeId":2,"connectedNodeId":2},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":3,"nodeType":"input","connectionType":"String","value":null,"connectedBlockId":15,"connectedBlockTypeId":3,"connectedNodeId":2}]},{"blockId":15,"blockTypeId":3,"position":{"x":700,"y":332},"nodes":[{"id":1,"nodeType":"input","connectionType":"String","value":"The End","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":2,"nodeType":"output","connectionType":"String","value":null,"connectedBlockId":14,"connectedBlockTypeId":2,"connectedNodeId":3}]}]"#;
    let code: Vec<bme::Block> = serde_json::from_str(json).unwrap();
    //println!("{}", serde_json::to_string_pretty(&code).unwrap());

    // set up the register
    let mut register: HashMap<(u32, u32), String> = HashMap::new();

    // find the start block
    let start = code.iter().find(|&b| b.blockTypeId == 1).unwrap();
    println!("startBlock: {:?}", start);

    // start the code execution
    let res = bme::execute_block(code.clone(), &mut register, start.blockId);
    println!("\nres: {:?}", res);
}