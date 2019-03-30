mod block;
mod error;

use error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Block {
    blockId: u32,
    blockTypeId: u32,
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Node {
    id: u32,
    nodeType: NodeType,
    connectionType: ConnectionType,
    value: Value,
    connectedBlockTypeId: Option<u32>,
    connectedBlockId: Option<u32>,
    connectedNodeId: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
enum NodeType {
    Input,
    Output,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ConnectionType {
    Execution,
    String,
    Integer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
enum Value {
    String(String),
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    blockId: u32,
    nodeId: u32,
    value: String,
}

fn main() {
    // define the rule
    let mut logic = block::Logic::new();
    logic.add_connection_type(
        ConnectionType::Execution,
        "black".to_string(),
        false,
        "".to_string(),
        "".to_string(),
    );
    logic.add_connection_type(
        ConnectionType::String,
        "purple".to_string(),
        true,
        "Text".to_string(),
        "".to_string(),
    );
    logic.add_connection_type(
        ConnectionType::Integer,
        "green".to_string(),
        true,
        "0".to_string(),
        "".to_string(),
    );
    logic.add_block(Box::new(block::Start {}));
    logic.add_block(Box::new(block::ConsoleLog {}));
    logic.add_block(Box::new(block::StaticString {}));

    println!(
        "{}",
        serde_json::to_string_pretty(&logic.get_json()).unwrap()
    );
    println!("");

    // parse the code from json to the objects
    let json = r#"[{"blockId":9,"blockTypeId":1,"position":{"x":240,"y":220},"nodes":[{"id":1,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":10,"connectedBlockTypeId":2,"connectedNodeId":1}]},{"blockId":10,"blockTypeId":2,"position":{"x":499,"y":204},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":9,"connectedBlockTypeId":1,"connectedNodeId":1},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":12,"connectedBlockTypeId":2,"connectedNodeId":1},{"id":3,"nodeType":"input","connectionType":"String","value":null,"connectedBlockId":11,"connectedBlockTypeId":3,"connectedNodeId":2}]},{"blockId":11,"blockTypeId":3,"position":{"x":237,"y":327},"nodes":[{"id":1,"nodeType":"input","connectionType":"String","value":"Hello World","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":2,"nodeType":"output","connectionType":"String","value":null,"connectedBlockId":10,"connectedBlockTypeId":2,"connectedNodeId":3}]},{"blockId":12,"blockTypeId":2,"position":{"x":699,"y":205},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":10,"connectedBlockTypeId":2,"connectedNodeId":2},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":14,"connectedBlockTypeId":2,"connectedNodeId":1},{"id":3,"nodeType":"input","connectionType":"String","value":"Text","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null}]},{"blockId":14,"blockTypeId":2,"position":{"x":911,"y":214},"nodes":[{"id":1,"nodeType":"input","connectionType":"Execution","value":null,"connectedBlockId":12,"connectedBlockTypeId":2,"connectedNodeId":2},{"id":2,"nodeType":"output","connectionType":"Execution","value":null,"connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":3,"nodeType":"input","connectionType":"String","value":null,"connectedBlockId":15,"connectedBlockTypeId":3,"connectedNodeId":2}]},{"blockId":15,"blockTypeId":3,"position":{"x":700,"y":332},"nodes":[{"id":1,"nodeType":"input","connectionType":"String","value":"The End","connectedBlockId":null,"connectedBlockTypeId":null,"connectedNodeId":null},{"id":2,"nodeType":"output","connectionType":"String","value":null,"connectedBlockId":14,"connectedBlockTypeId":2,"connectedNodeId":3}]}]"#;
    let code: Vec<Block> = serde_json::from_str(json).unwrap();
    //println!("{}", serde_json::to_string_pretty(&code).unwrap());

    // set up the register
    let mut register: HashMap<(u32, u32), String> = HashMap::new();

    // find the start block
    let start = code.iter().find(|&b| b.blockTypeId == 1).unwrap();
    println!("startBlock: {:?}", start);

    // start the code execution
    let res = execute_block(code.clone(), &mut register, start.blockId);
    println!("\nres: {:?}", res);
}

fn execute_block(
    code: Vec<Block>,
    register: &mut HashMap<(u32, u32), String>,
    next_block_id: u32,
) -> Result<()> {
    // get the block for the block id
    let block = code
        .iter()
        .find(|&b| b.blockId == next_block_id)
        .ok_or("No Block with the given id avilable")?;
    println!("\nblock: {:?}", block);

    // handle the input of the node and execute the node
    exec_inputs(&code, register, block)?;

    let next_node = block
        .nodes
        .iter()
        .find(|&n| n.nodeType == NodeType::Output && n.connectionType == ConnectionType::Execution)
        .ok_or("Execution ended (No next Execution Node")?;

    let next_block_id = next_node
        .connectedBlockId
        .ok_or("Execution ended (No next Execution Node connected)")?;
    println!("\nnext_block_id: {:?}", next_block_id);

    execute_block(code, register, next_block_id)?;

    Ok(())
}

fn exec_inputs(
    code: &[Block],
    register: &mut HashMap<(u32, u32), String>,
    block: &Block,
) -> Result<Vec<Register>> {
    // get all input nodes for this block which are not of type execution
    let inputs = block
        .nodes
        .iter()
        .filter(|n| n.nodeType == NodeType::Input && n.connectionType != ConnectionType::Execution)
        .collect::<Vec<&Node>>();

    let mut results: Vec<Register> = vec![];

    // handle all input nodes
    for n in inputs {
        // when another block is connected
        if n.connectedBlockId.is_some()
            && n.connectedNodeId.is_some()
            && n.connectedBlockTypeId.is_some()
        {
            // get the other block and node id
            let con_block_id = n.connectedBlockId.ok_or("Is always okay")?;
            let con_node_id = n.connectedNodeId.ok_or("Is always okay")?;
            let con_block_type_id = n.connectedBlockTypeId.ok_or("Is always okay")?;
            println!(
                "\ncon_block_id: {:?} con_node_id: {:?} con_block_id: {:?}",
                con_block_id, con_node_id, con_block_type_id
            );

            // get the exec_block
            let exec_block = block::get_block(con_block_type_id)
                .ok_or("The given Block Type is not avilable")?;

            // based upon the block type we have different executions
            match exec_block.get_type() {
                block::ExecutionBlockType::Start => {
                    unimplemented!("Can't handle Start block outputs");
                }
                block::ExecutionBlockType::Static => {
                    // get the other block
                    let inp_block = code
                        .iter()
                        .find(|&b| b.blockId == con_block_id)
                        .ok_or("No Block with the given id avilable")?;

                    let values = exec_inputs(code, register, inp_block)?;

                    println!("\nvalues: {:?} ", values);

                    results.push(
                        values
                            .into_iter()
                            .find(|v| v.nodeId == con_node_id)
                            .ok_or("No value available")?,
                    );
                }
                block::ExecutionBlockType::Normal => {
                    let value = register
                        .get(&(con_block_id, con_node_id))
                        .ok_or("Value not avilable in register")?;

                    results.push(Register {
                        blockId: con_block_id,
                        nodeId: con_node_id,
                        value: value.clone(),
                    });
                }
            };

        // when no other block is connected
        } else {
            let value = match &n.value {
                Value::String(s) => s.clone(),
                _ => "null".to_string(),
            };

            results.push(Register {
                blockId: block.blockId,
                nodeId: n.id,
                value,
            });
        }
    }

    let exec_block =
        block::get_block(block.blockTypeId).ok_or("The given Block Type is not avilable")?;

    let ret = exec_block.exec(results)?;

    // outputs of normal blocks need to be saved to registers

    Ok(ret)
}
