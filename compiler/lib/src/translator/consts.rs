use x64asm::instruction::Operand;

pub const START_LABEL: &str = "_start";
pub const OP_STACK_LABEL: &str = "op_stack";
pub const OP_STACK_BASE_LABEL: &str = "op_stack_base";
pub const OP_SIZE: Operand = Operand::Dword;
pub const OP_SIZE_BYTES: i64 = 4;
pub const OP_STACK_SIZE: i64 = OP_SIZE_BYTES * 1024;
