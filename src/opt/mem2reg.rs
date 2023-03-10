use crate::ir::*;
use crate::opt::FunctionPass;
use crate::*;

pub type Mem2reg = FunctionPass<Mem2regInner>;

#[derive(Default, Clone, Copy, Debug)]
pub struct Mem2regInner {}

impl Optimize<FunctionDefinition> for Mem2regInner {
    fn optimize(&mut self, _code: &mut FunctionDefinition) -> bool {
        todo!("Homework: Register Promotion")
    }
}
