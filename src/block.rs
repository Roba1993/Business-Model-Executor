use crate::error::Result;
use crate::ConnectionType;

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
        1 => Some(&Start {}),
        2 => Some(&ConsoleLog {}),
        3 => Some(&StaticString {}),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Start {}
impl ExecutionBlock for Start {
    fn get_id(&self) -> u32 {
        1
    }

    fn get_name(&self) -> &'static str {
        "Start"
    }

    fn get_type(&self) -> ExecutionBlockType {
        ExecutionBlockType::Start
    }

    fn exec(&self, input: Vec<crate::Register>) -> Result<Vec<crate::Register>> {
        println!("#> Start Executed");
        println!("#> {:?}", input);

        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct ConsoleLog {}
impl ExecutionBlock for ConsoleLog {
    fn get_id(&self) -> u32 {
        2
    }

    fn get_name(&self) -> &'static str {
        "Console Log"
    }

    fn get_type(&self) -> ExecutionBlockType {
        ExecutionBlockType::Normal
    }

    fn get_inputs(&self) -> &'static [ConnectionType] {
        &[ConnectionType::String]
    }

    fn exec(&self, input: Vec<crate::Register>) -> Result<Vec<crate::Register>> {
        println!("#> Console Log Executed");
        println!("#> {:?}", input);

        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct StaticString {}
impl ExecutionBlock for StaticString {
    fn get_id(&self) -> u32 {
        3
    }

    fn get_name(&self) -> &'static str {
        "Static String"
    }

    fn get_type(&self) -> ExecutionBlockType {
        ExecutionBlockType::Static
    }

    fn get_inputs(&self) -> &'static [ConnectionType] {
        &[ConnectionType::String]
    }

    fn get_outputs(&self) -> &'static [ConnectionType] {
        &[ConnectionType::String]
    }

    fn exec(&self, input: Vec<crate::Register>) -> Result<Vec<crate::Register>> {
        println!("#> Static String Executed");
        println!("#> {:?}", input);

        let mut input = input;

        for i in &mut input {
            if i.nodeId == 1 {
                i.nodeId = 2;
            }
        }

        println!("#> {:?}", &input);

        Ok(input)
    }
}
