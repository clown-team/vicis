use crate::codegen::{
    function::basic_block::BasicBlockId,
    register::{Reg, VReg},
};
use id_arena::Id;
use std::fmt;

pub type InstructionId<Data> = Id<Instruction<Data>>;

pub trait InstructionData: Clone + fmt::Debug {
    fn input_vregs(&self) -> Vec<VReg>;
    fn output_vregs(&self) -> Vec<VReg>;
    fn input_regs(&self) -> Vec<Reg>;
    fn output_regs(&self) -> Vec<Reg>;
    fn rewrite(&mut self, vreg: VReg, reg: Reg);
    fn is_copy(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Instruction<Data: InstructionData> {
    pub id: Option<InstructionId<Data>>,
    pub data: Data,
    pub parent: BasicBlockId,
}

impl<Data: InstructionData> Instruction<Data> {
    pub fn new(data: Data, parent: BasicBlockId) -> Self {
        Self {
            id: None,
            data,
            parent,
        }
    }
}
