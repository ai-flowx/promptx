use std::error::Error;

#[derive(Clone, Default)]
pub struct LLM {}

impl LLM {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
