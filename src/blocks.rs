use crate::error::Result;
use crate::{ExecutionBlock, ExecutionBlockType, ConnectionType};

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
            if i.nodeId == 0 {
                i.nodeId = 1;
            }
        }

        println!("#> {:?}", &input);

        Ok(input)
    }
}
