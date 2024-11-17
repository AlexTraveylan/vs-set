use crate::strategy::{CopyStrategy, PythonStrategy, TypeScriptStrategy};
use std::boxed::Box;

pub struct StrategyFactory;

impl StrategyFactory {
    pub fn create_strategy(language: &str) -> Box<dyn CopyStrategy> {
        match language {
            "python" => Box::new(PythonStrategy),
            "typescript" => Box::new(TypeScriptStrategy),
            _ => panic!("Langage non supporté: {}", language),
        }
    }
}
