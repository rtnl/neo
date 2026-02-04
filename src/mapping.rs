use crate::operation::Operation;
use str_macro::str;

pub trait Mapping {
    fn get_operation_aliases(&self, operation: Operation) -> Vec<String>;
}

#[derive(Debug)]
pub struct MappingDefault {}

impl Mapping for MappingDefault {
    fn get_operation_aliases(&self, operation: Operation) -> Vec<String> {
        match operation {
            Operation::FileRead => vec![str!("file-read")],
            Operation::FileCopy => vec![str!("file-copy")],
        }
    }
}
