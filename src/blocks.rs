use crate::error::Result;
use crate::{ExecutionBlock, ExecutionBlockType, ConnectionType, Value};

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

    fn intern_execute(&self, _input: Vec<Value>) -> Result<Vec<Value>> {
        println!("#Start>");

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

    fn intern_execute(&self, input: Vec<Value>) -> Result<Vec<Value>> {
        // get the first value
        if let Some(i) = input.get(0) {
            // print it
            match i {
                Value::String(s) => println!("#> {}", s),
                _ => println!("#> null")
            }
        }
        // if no value is available, we print a plain promt
        else {
            println!("#>");
        }

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

    fn intern_execute(&self, input: Vec<Value>) -> Result<Vec<Value>> {
        Ok(input)
    }
}
