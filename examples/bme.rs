fn main() {
    // define the rule
    let logic = bme::Logic::default();

    println!(
        "{}",
        serde_json::to_string_pretty(&logic.get_json()).unwrap()
    );
    println!("");

    // parse the code from json to the objects
    let json = r#"[{"blockId":1,"blockTypeId":1,"position":{"x":351,"y":100},"nodes":[{"id":0,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":2,"connectedBlockTypeId":2,"connectedNodeId":0}]},{"blockId":2,"blockTypeId":2,"position":{"x":583,"y":96},"nodes":[{"id":0,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":1,"connectedBlockTypeId":1,"connectedNodeId":0},{"id":1,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":3,"connectedBlockTypeId":2,"connectedNodeId":0},{"id":2,"nodeType":"input","connectionType":"String","value":null,"connectedBlockId":4,"connectedBlockTypeId":4,"connectedNodeId":3}]},{"blockId":5,"blockTypeId":3,"position":{"x":116,"y":222},"nodes":[{"id":2,"nodeType":"input","connectionType":"String","value":"Hello","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":3,"nodeType":"output","connectionType":"String","value":null,"connectedBlockId":4,"connectedBlockTypeId":4,"connectedNodeId":2}]},{"blockId":4,"blockTypeId":4,"position":{"x":378,"y":224},"nodes":[{"id":2,"nodeType":"input","connectionType":"String","value":null,"connectedBlockId":5,"connectedBlockTypeId":3,"connectedNodeId":3},{"id":3,"nodeType":"output","connectionType":"String","value":null,"connectedBlockId":2,"connectedBlockTypeId":2,"connectedNodeId":2},{"id":4,"nodeType":"input","connectionType":"String","value":"World","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null}]},{"blockId":3,"blockTypeId":2,"position":{"x":894,"y":87},"nodes":[{"id":0,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":2,"connectedBlockTypeId":2,"connectedNodeId":1},{"id":1,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":2,"nodeType":"input","connectionType":"String","value":"!!!","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null}]}]"#;

    let mut exe = bme::Executer::new(json.to_string());
    exe.execute(vec![]).unwrap();
}