pub mod blocks;
mod error;

use error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Block {
    #[serde(alias = "blockId")]
    block_id: u32,
    #[serde(alias = "blockTypeId")]
    block_type_id: u32,
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Node {
    id: u32,
    #[serde(alias = "nodeType")]
    node_type: String,
    #[serde(alias = "connectionType")]
    connection_type: String,
    value: Value,
    #[serde(alias = "connectedBlockTypeId")]
    connected_block_type_id: Option<u32>,
    #[serde(alias = "connectedBlockId")]
    connected_block_id: Option<u32>,
    #[serde(alias = "connectedNodeId")]
    connected_node_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Unknown,
}

impl Value {
    pub fn get_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    block_id: u32,
    node_id: u32,
    value: Value,
}

pub trait ExecutionBlock: std::fmt::Debug {
    fn get_id(&self) -> u32;
    fn get_name(&self) -> &'static str;
    fn get_type(&self) -> ExecutionBlockType;
    fn intern_execute(&self, input: Vec<Value>) -> Result<Vec<Value>>;

    fn get_inputs(&self) -> &'static [&'static str] {
        &[]
    }

    fn get_outputs(&self) -> &'static [&'static str] {
        &[]
    }

    fn execute(&self, input: Vec<crate::Register>, block_id: u32) -> Result<Vec<crate::Register>> {
        let inp = input.into_iter().map(|r| r.value).collect::<Vec<Value>>();

        // execute the block
        let inp = self.intern_execute(inp)?;

        let out = inp
            .into_iter()
            .enumerate()
            .map(|(i, v)| Register {
                block_id: block_id,
                node_id: ((i as u32) * 2) + 3,
                value: v,
            })
            .collect::<Vec<Register>>();

        Ok(out)
    }

    fn get_json(&self) -> serde_json::Value {
        let mut nodes: Vec<serde_json::Value> = vec![];

        // based on the node type we generate default inputs / outputs
        match self.get_type() {
            ExecutionBlockType::Start => {
                nodes.push(serde_json::json!({ "id": 0, "io": "output", "type": "Execution", "name": "Next" }));
            }
            ExecutionBlockType::Normal => {
                nodes.push(serde_json::json!({ "id": 0, "io": "input", "type": "Execution", "name": "Run" }));
                nodes.push(serde_json::json!({ "id": 1, "io": "output", "type": "Execution", "name": "Next" }));
            }
            ExecutionBlockType::Static => {}
        };

        // add the other defined input & outputs
        let mut inp = self.get_inputs().iter();
        let mut out = self.get_outputs().iter();
        let mut index = 2;

        loop {
            // get the next input & output
            let i = inp.next();
            let o = out.next();

            // try to add the next input
            if let Some(n) = i {
                nodes.push(serde_json::json!({ "id": index, "io": "input", "type": n }));
            }

            // try to add the next output
            if let Some(n) = o {
                nodes.push(serde_json::json!({ "id": index+1, "io": "output", "type": n }));
            }

            index += 2;

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
    connections: Vec<(String, String, bool, String, String)>,
}

impl Logic {
    pub fn empty() -> Logic {
        Logic {
            blocks: vec![],
            connections: vec![
                (
                    "Execution".to_string(),
                    "black".to_string(),
                    false,
                    "".to_string(),
                    "".to_string(),
                ),
                (
                    "String".to_string(),
                    "purple".to_string(),
                    true,
                    "Text".to_string(),
                    "".to_string(),
                ),
                (
                    "Integer".to_string(),
                    "green".to_string(),
                    true,
                    "0".to_string(),
                    "".to_string(),
                ),
            ],
        }
    }

    pub fn add_block(&mut self, block: Box<ExecutionBlock>) {
        self.blocks.push(block);
    }

    pub fn get_block(&self, id: u32) -> Option<&Box<ExecutionBlock>> {
        self.blocks.iter().find(|b| b.get_id() == id)
    }

    pub fn get_blocks_by_type(&self, typ: ExecutionBlockType) -> Vec<&Box<ExecutionBlock>> {
        self.blocks
            .iter()
            .filter(|b| b.get_type() == typ)
            .collect::<Vec<&Box<ExecutionBlock>>>()
    }

    pub fn add_connection_type(
        &mut self,
        typ: String,
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

impl Default for Logic {
    fn default() -> Self {
        let mut logic = Logic::empty();

        logic.add_block(Box::new(blocks::Start {}));
        logic.add_block(Box::new(blocks::ConsoleLog {}));
        logic.add_block(Box::new(blocks::StaticString {}));
        logic.add_block(Box::new(blocks::AddString {}));

        logic
    }
}

pub struct Executer {
    logic: Logic,
    raw_code: String,
    code: Vec<Block>,
    code_ok: bool,
    register: HashMap<(u32, u32), Value>,
}

impl Executer {
    pub fn new(code: String) -> Executer {
        Executer {
            logic: Logic::default(),
            raw_code: code,
            code: vec![],
            code_ok: false,
            register: HashMap::new(),
        }
    }

    pub fn analyze(&mut self) -> Result<()> {
        self.code = serde_json::from_str(&self.raw_code)?;

        // todo
        // - Check for only 1 Block Start Type
        // - Check for only 1 Start Block
        // - Check if all connections used in blocks exist
        // - Check if all connections use in blocks are valid

        self.code_ok = true;
        Ok(())
    }

    pub fn execute(&mut self) -> Result<()> {
        if !self.code_ok {
            self.analyze()?;
        }

        self.register.clear();

        let start_block = self.find_start_block()?;

        self.execute_block(start_block.block_id)?;

        Ok(())
    }

    fn find_start_block(&self) -> Result<&Block> {
        let blocks = self.logic.get_blocks_by_type(ExecutionBlockType::Start);
        let start_block = blocks
            .get(0)
            .ok_or("No start blocks defined in the logic")?;

        let block = self
            .code
            .iter()
            .find(|&b| b.block_type_id == start_block.get_id())
            .ok_or("No start Block available")?;

        Ok(block)
    }

    fn execute_block(&mut self, next_block_id: u32) -> Result<()> {
        // handle the input of the node and execute the node
        let reg = self.exec_inputs(next_block_id)?;

        // insert the result into the register
        for r in reg {
            self.register.insert((r.block_id, r.node_id), r.value);
        }

        // get the block for the block id
        let block = self
            .code
            .iter()
            .find(|&b| b.block_id == next_block_id)
            .ok_or("No Block with the given id avilable")?;

        // get the next node if available
        if let Some(next_node) = block
            .nodes
            .iter()
            .find(|&n| n.node_type == "output" && n.connection_type == "Execution")
        {
            // get the next block id if avilable
            if let Some(next_block_id) = next_node.connected_block_id {
                // when available, start again
                self.execute_block(next_block_id)?;
            }
        }

        // When this point is reached, the programm has ended sucessfully
        Ok(())
    }

    fn exec_inputs(&self, block_id: u32) -> Result<Vec<Register>> {
        // get the block for the block id
        let block = self
            .code
            .iter()
            .find(|&b| b.block_id == block_id)
            .ok_or("No Block with the given id avilable")?;

        // get all input nodes for this block which are not of type execution
        let inputs = block
            .nodes
            .iter()
            .filter(|n| n.node_type == "input" && n.connection_type != "Execution")
            .collect::<Vec<&Node>>();

        let mut results: Vec<Register> = vec![];

        // handle all input nodes
        for n in inputs {
            // when another block is connected
            if n.connected_block_id.is_some()
                && n.connected_node_id.is_some()
                && n.connected_block_type_id.is_some()
            {
                // get the other block and node id
                let con_block_id = n.connected_block_id.ok_or("Is always okay")?;
                let con_node_id = n.connected_node_id.ok_or("Is always okay")?;
                let con_block_type_id = n.connected_block_type_id.ok_or("Is always okay")?;

                // get the exec_block
                let exec_block = self
                    .logic
                    .get_block(con_block_type_id)
                    .ok_or("The given Block Type is not avilable")?;

                // based upon the block type we have different executions
                match exec_block.get_type() {
                    ExecutionBlockType::Start => {
                        unimplemented!("Can't handle Start block outputs");
                    }
                    ExecutionBlockType::Static => {
                        let values = self.exec_inputs(con_block_id)?;

                        results.push(
                            values
                                .into_iter()
                                .find(|v| v.node_id == con_node_id)
                                .ok_or("No value available")?,
                        );
                    }
                    ExecutionBlockType::Normal => {
                        let value = self
                            .register
                            .get(&(con_block_id, con_node_id))
                            .ok_or("Value not avilable in register")?;

                        results.push(Register {
                            block_id: con_block_id,
                            node_id: con_node_id,
                            value: value.clone(),
                        });
                    }
                };

            // when no other block is connected
            } else {
                results.push(Register {
                    block_id: block.block_id,
                    node_id: n.id,
                    value: n.value.clone(),
                });
            }
        }

        let exec_block = self
            .logic
            .get_block(block.block_type_id)
            .ok_or("The given Block Type is not avilable")?;

        let ret = exec_block.execute(results, block_id)?;

        Ok(ret)
    }
}
