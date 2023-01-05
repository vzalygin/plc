mod compile_error;

use std::error::Error;

use inkwell::{
    context::Context,
    module::{Module, Linkage}, targets::{Target, InitializationConfig, TargetMachine, FileType}, memory_buffer::MemoryBuffer,
};

use crate::parsing::Ast;

use self::compile_error::CompileError;

pub struct CompileInfo {
    pub ast: Ast,
}


fn generate_ir<'ctx>(ast: &'ctx Ast, ctx: &'ctx Context) -> Module<'ctx> {
    let module = ctx.create_module("main");
    let builder = ctx.create_builder();

    let i32_type = ctx.i32_type();
    let main_fn = module.add_function(
        "main",
        i32_type.fn_type(&[], false),
        Some(Linkage::External),
    );

    let basic_block = ctx.append_basic_block(main_fn, "entry");
    builder.position_at_end(basic_block);

    // fuckin magic

    let i32_zero = i32_type.const_int(0, false);
    builder.build_return(Some(&i32_zero));

    module
}

pub fn compile(info: CompileInfo) -> Result<MemoryBuffer, Box<dyn Error>> {
    let ctx = Context::create();
    let module = generate_ir(&info.ast, &ctx);

    Target::initialize_all(&InitializationConfig::default());
    
    let triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();

    let target = Target::from_triple(&triple)?;
    let target_machine =  match target.create_target_machine(
        &triple, 
        &cpu, 
        &features, 
        inkwell::OptimizationLevel::Default, 
        inkwell::targets::RelocMode::Default, 
        inkwell::targets::CodeModel::Default
    ) {
        Some(machine) => machine,
        None => return Err(Box::new(CompileError::Unknown)),
    };

    let buf = target_machine.write_to_memory_buffer(&module, FileType::Object)?; 

    Ok(buf)
}
