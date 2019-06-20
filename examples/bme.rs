fn main() {
    // define the rule
    let logic = bme::Logic::default();

    println!(
        "{}",
        serde_json::to_string_pretty(&logic.get_json()).unwrap()
    );
    println!("");

    // parse the code from json to the objects
    let json = r#"[{"blockId":1,"blockTypeId":1,"position":{"x":169,"y":205},"nodes":[{"id":1,"nodeType":"output","connectionType":"Execution","value":null,"connections":[{"type":"Execution","startBlock":1,"endBlock":3,"startNode":1,"endNode":1}]}]},{"blockId":3,"blockTypeId":2,"position":{"x":530,"y":164},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connections":[{"type":"Execution","startBlock":1,"endBlock":3,"startNode":1,"endNode":1}]},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connections":[{"type":"Execution","startBlock":3,"endBlock":6,"startNode":2,"endNode":1}]},{"id":3,"nodeType":"input","connectionType":"String","value":null,"connections":[{"type":"String","startBlock":5,"endBlock":3,"startNode":2,"endNode":3}]}]},{"blockId":5,"blockTypeId":3,"position":{"x":155,"y":381},"nodes":[{"id":1,"nodeType":"input","connectionType":"String","value":"Test","connections":[]},{"id":2,"nodeType":"output","connectionType":"String","value":null,"connections":[{"type":"String","startBlock":5,"endBlock":3,"startNode":2,"endNode":3},{"type":"String","startBlock":5,"endBlock":6,"startNode":2,"endNode":3}]}]},{"blockId":6,"blockTypeId":2,"position":{"x":879,"y":225},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connections":[{"type":"Execution","startBlock":3,"endBlock":6,"startNode":2,"endNode":1}]},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connections":[]},{"id":3,"nodeType":"input","connectionType":"String","value":null,"connections":[{"type":"String","startBlock":5,"endBlock":6,"startNode":2,"endNode":3}]}]}]"#;

    let mut exe = bme::Executer::new(json.to_string());
    println!("\n{:?}", exe.execute(vec![]).unwrap());
}