//use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub enum TaskInput {
    Toggle(bool),
}

#[derive(Debug)]
pub enum TaskOutput {
    Delete(DynamicIndex),
}

#[derive(Debug)]
pub enum AppMsg {
    DeleteEntry(DynamicIndex),
	DeleteAll,
    AddDir(String),
	SelectFile(i32),
}