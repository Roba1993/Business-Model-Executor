use crate::error::Result;
use crate::{ExecutionBlock, ExecutionBlockType, Value};

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

    fn get_inputs(&self) -> &'static [&'static str] {
        &["String"]
    }

    fn intern_execute(&self, input: Vec<Value>) -> Result<Vec<Value>> {
        // get the first value
        if let Some(i) = input.get(0) {
            // print it
            match i {
                Value::String(s) => println!("#> {}", s),
                _ => println!("#> null"),
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

    fn get_inputs(&self) -> &'static [&'static str] {
        &["String"]
    }

    fn get_outputs(&self) -> &'static [&'static str] {
        &["String"]
    }

    fn intern_execute(&self, input: Vec<Value>) -> Result<Vec<Value>> {
        Ok(input)
    }
}

#[derive(Debug)]
pub struct AddString {}
impl ExecutionBlock for AddString {
    fn get_id(&self) -> u32 {
        4
    }

    fn get_name(&self) -> &'static str {
        "Add String"
    }

    fn get_type(&self) -> ExecutionBlockType {
        ExecutionBlockType::Static
    }

    fn get_inputs(&self) -> &'static [&'static str] {
        &["String", "String"]
    }

    fn get_outputs(&self) -> &'static [&'static str] {
        &["String"]
    }

    fn intern_execute(&self, input: Vec<Value>) -> Result<Vec<Value>> {
        let value = format!(
            "{}{}",
            input
                .get(0)
                .ok_or("String input 1 not available")?
                .get_string()
                .ok_or("Value is not a String")?,
            input
                .get(1)
                .ok_or("String input 2 not available")?
                .get_string()
                .ok_or("Value is not a String")?
        );

        let mut input = input;
        input.clear();
        input.push(Value::String(value));

        Ok(input)
    }
}
