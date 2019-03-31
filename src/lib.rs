mod error;
pub mod blocks;

use error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block {
    pub blockId: u32,
    pub blockTypeId: u32,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Node {
    pub id: u32,
    pub nodeType: NodeType,
    pub connectionType: ConnectionType,
    pub value: Value,
    pub connectedBlockTypeId: Option<u32>,
    pub connectedBlockId: Option<u32>,
    pub connectedNodeId: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
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
pub enum Value {
    String(String),
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    blockId: u32,
    nodeId: u32,
    value: String,
}

pub fn execute_block(
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

pub fn exec_inputs(
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
            let exec_block = get_block(con_block_type_id)
                .ok_or("The given Block Type is not avilable")?;

            // based upon the block type we have different executions
            match exec_block.get_type() {
                ExecutionBlockType::Start => {
                    unimplemented!("Can't handle Start block outputs");
                }
                ExecutionBlockType::Static => {
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
                ExecutionBlockType::Normal => {
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
        get_block(block.blockTypeId).ok_or("The given Block Type is not avilable")?;

    let ret = exec_block.exec(results)?;

    // outputs of normal blocks need to be saved to registers

    Ok(ret)
}





pub trait ExecutionBlock: std::fmt::Debug {
    fn get_id(&self) -> u32;
    fn get_name(&self) -> &'static str;
    fn get_type(&self) -> ExecutionBlockType;
    fn exec(&self, input: Vec<crate::Register>) -> Result<Vec<crate::Register>>;

    fn get_inputs(&self) -> &'static [ConnectionType] {
        &[]
    }

    fn get_outputs(&self) -> &'static [ConnectionType] {
        &[]
    }

    fn get_json(&self) -> serde_json::Value {
        let mut nodes: Vec<serde_json::Value> = vec![];

        // based on the node type we generate default inputs / outputs
        match self.get_type() {
            ExecutionBlockType::Start => {
                nodes.push(serde_json::json!({ "id": 0, "io": "output", "type": "Execution" }));
            }
            ExecutionBlockType::Normal => {
                nodes.push(serde_json::json!({ "id": 0, "io": "input", "type": "Execution" }));
                nodes.push(serde_json::json!({ "id": 1, "io": "output", "type": "Execution" }));
            }
            _ => {}
        };

        // add the other defined input & outputs
        let mut inp = self.get_inputs().iter();
        let mut out = self.get_outputs().iter();
        let mut index = nodes.len();

        loop {
            // get the next input & output
            let i = inp.next();
            let o = out.next();

            // try to add the next input
            if let Some(n) = i {
                nodes.push(serde_json::json!({ "id": index, "io": "input", "type": n }));
                index += 1;
            }

            // try to add the next output
            if let Some(n) = o {
                nodes.push(serde_json::json!({ "id": index, "io": "output", "type": n }));
                index += 1;
            }

            // when input & output are completly loop over, end
            if i.is_none() && o.is_none() {
                break;
            }
        }

        serde_json::json!({
            "id": self.get_id(),
            "name": self.get_name(),
            "nodes": nodes,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExecutionBlockType {
    Start,
    Static,
    Normal,
}

pub struct Logic {
    blocks: Vec<Box<ExecutionBlock>>,
    connections: Vec<(ConnectionType, String, bool, String, String)>,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            blocks: vec![],
            connections: vec![],
        }
    }

    pub fn add_block(&mut self, block: Box<ExecutionBlock>) {
        self.blocks.push(block);
    }

    pub fn get_block(&self, block_type: u32) -> Option<&Box<ExecutionBlock>> {
        self.blocks.iter().find(|b| b.get_id() == block_type)
    }

    pub fn add_connection_type(
        &mut self,
        typ: ConnectionType,
        color: String,
        edit: bool,
        default: String,
        rule: String,
    ) {
        self.connections.push((typ, color, edit, default, rule));
    }

    pub fn get_connection_json(&self) -> Vec<serde_json::Value> {
        let mut cons = vec![];

        for c in &self.connections {
            cons.push(serde_json::json!({
                "type": c.0,
                "color": c.1,
                "valueEdit": c.2,
                "valueDefault": c.3,
                "valueCheck": c.4
            }));
        }

        cons
    }

    pub fn get_json(&self) -> serde_json::Value {
        // define the rules
        let rules = serde_json::json!({
            "strictInputOutput": true,
            "strictDifferentBlock": true,
            "strictConnections": true,
        });

        let blocks = self
            .blocks
            .iter()
            .map(|b| b.get_json())
            .collect::<Vec<serde_json::Value>>();

        // combine everythink to the logic block
        serde_json::json!({
            "rules": rules,
            "connections": self.get_connection_json(),
            "blocks": blocks,
        })
    }
}

pub fn get_block(block_type: u32) -> Option<&'static ExecutionBlock> {
    match block_type {
        0 => None,
        1 => Some(&blocks::Start {}),
        2 => Some(&blocks::ConsoleLog {}),
        3 => Some(&blocks::StaticString {}),
        _ => None,
    }
}