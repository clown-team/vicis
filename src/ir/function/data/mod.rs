use crate::ir::{
    function::{
        basic_block::{BasicBlock, BasicBlockId},
        instruction::{Instruction, InstructionId},
    },
    value::{Value, ValueId},
};
use id_arena::Arena;

pub struct Data {
    pub values: Arena<Value>,
    pub instructions: Arena<Instruction>,
    pub basic_blocks: Arena<BasicBlock>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            values: Arena::new(),
            instructions: Arena::new(),
            basic_blocks: Arena::new(),
        }
    }

    pub fn create_block(&mut self) -> BasicBlockId {
        self.basic_blocks.alloc(BasicBlock::new())
    }

    pub fn create_inst(&mut self, mut inst: Instruction) -> InstructionId {
        let id = self.instructions.alloc_with_id(|id| {
            inst.id = Some(id);
            inst
        });
        self.set_inst_users(id);
        id
    }

    pub fn create_value(&mut self, inst: Value) -> ValueId {
        self.values.alloc(inst)
    }

    pub fn block_ref(&self, id: BasicBlockId) -> &BasicBlock {
        &self.basic_blocks[id]
    }

    // TODO: Is this the right way?
    pub fn block_ref_mut(&mut self, id: BasicBlockId) -> &mut BasicBlock {
        &mut self.basic_blocks[id]
    }

    pub fn inst_ref(&self, id: InstructionId) -> &Instruction {
        &self.instructions[id]
    }

    pub fn inst_ref_mut(&mut self, id: InstructionId) -> &mut Instruction {
        &mut self.instructions[id]
    }

    pub fn value_ref(&self, id: ValueId) -> &Value {
        &self.values[id]
    }

    pub fn value_ref_mut(&mut self, id: ValueId) -> &mut Value {
        &mut self.values[id]
    }

    // For `Instruction`s

    fn set_inst_users(&mut self, id: InstructionId) {
        let args = self.instructions[id]
            .operand
            .args()
            .into_iter()
            .filter_map(|&arg| match &self.values[arg] {
                Value::Instruction(id) => Some(*id),
                _ => None,
            })
            .collect::<Vec<InstructionId>>();
        for arg in args {
            self.instructions[arg].users.insert(id);
        }
    }

    pub fn remove_uses(&mut self, id: InstructionId) {
        let args = self.instructions[id]
            .operand
            .args()
            .into_iter()
            .filter_map(|&arg| match &self.values[arg] {
                Value::Instruction(id) => Some(*id),
                _ => None,
            })
            .collect::<Vec<InstructionId>>();
        for arg in args {
            self.instructions[arg].users.remove(&id);
        }
    }
}
