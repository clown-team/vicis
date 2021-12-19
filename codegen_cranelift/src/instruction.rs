use super::LowerCtx;
use cranelift::{
    frontend::FunctionBuilder,
    prelude::{Block, InstBuilder, Value},
};
use cranelift_module::Module;
use rustc_hash::FxHashMap;
use vicis_core::ir::{
    function::{
        basic_block::BasicBlockId,
        instruction::{InstructionId, IntBinary, Operand, Ret},
        Function,
    },
    types::TypeId,
    value::ValueId,
    value::{ConstantData, ConstantInt, Value as LlvmValue},
};

pub struct InstCompiler<'a, M: Module> {
    pub lower_ctx: &'a LowerCtx<'a, M>,
    pub llvm_func: &'a Function,
    pub builder: &'a mut FunctionBuilder<'a>,
    pub blocks: FxHashMap<BasicBlockId, Block>,
    pub insts: FxHashMap<InstructionId, Value>,
}

impl<'a, M: Module> InstCompiler<'a, M> {
    pub fn compile(&mut self, inst_id: InstructionId) {
        let inst = self.llvm_func.data.inst_ref(inst_id);

        match inst.operand {
            Operand::IntBinary(IntBinary {
                ty,
                args: [lhs, rhs],
                ..
            }) => {
                let lhs = self.value(lhs, ty);
                let rhs = self.value(rhs, ty);
                let val = self.builder.ins().iadd(lhs, rhs);
                self.insts.insert(inst_id, val);
            }
            Operand::Ret(Ret { val: Some(val), ty }) => {
                let val = self.value(val, ty);
                self.builder.ins().return_(&[val]);
            }
            _ => {}
        };
    }

    pub fn create_block_for(&mut self, block_id: BasicBlockId) -> Block {
        let block = self.builder.create_block();
        self.blocks.insert(block_id, block);
        block
    }

    fn value(&mut self, val_id: ValueId, ty: TypeId) -> Value {
        match self.llvm_func.data.value_ref(val_id) {
            LlvmValue::Constant(ConstantData::Int(ConstantInt::Int32(i))) => self
                .builder
                .ins()
                .iconst(self.lower_ctx.into_clif_ty(ty), *i as i64),
            LlvmValue::Argument(idx) => {
                let entry = self.llvm_func.layout.get_entry_block().unwrap();
                let entry = self.blocks[&entry];
                self.builder.block_params(entry)[*idx]
            }
            LlvmValue::Instruction(inst_id) => self.insts[inst_id],
            _ => todo!(),
        }
    }
}
