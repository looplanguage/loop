use crate::compiler::instructions::print_instructions;
use crate::compiler::CompilerState;
use crate::lib::config::CONFIG;
use crate::lib::exception::Exception;
use crate::lib::jit::CodeGen;
use crate::lib::object::Object;
use crate::vm::VMState;
use crate::{compiler, lexer, parser, vm};
use chrono::Utc;
use colored::Colorize;
use inkwell::context::Context;
use inkwell::passes::PassManager;
use inkwell::OptimizationLevel;
use std::cell::RefCell;
use std::rc::Rc;

type ExecuteCodeReturn = (
    Result<Rc<RefCell<Object>>, String>,
    Option<CompilerState>,
    Option<VMState>,
);

pub fn execute_code(
    code: &str,
    compiler_state: Option<&CompilerState>,
    vm_state: Option<&VMState>,
) -> ExecuteCodeReturn {
    let l = lexer::build_lexer(code);
    let mut parser = parser::build_parser(l);

    let program = parser.parse();

    if !parser.errors.is_empty() {
        for err in parser.errors {
            if let Exception::Parser(msg) = err {
                println!("ParserException: {}", msg);
            }
        }

        panic!("Parser exceptions occurred!")
    }

    let mut comp = compiler::build_compiler(compiler_state);
    let error = comp.compile(program);

    if error.is_err() {
        let message = format!("CompilerError: {}", error.err().unwrap().pretty_print());
        println!("{}", message.as_str().red());
        return (Err(message), None, None);
    }

    if CONFIG.debug_mode {
        print_instructions(comp.scope().instructions.clone());
    }

    let mut vm = vm::build_vm(comp.get_bytecode(), vm_state, "MAIN".to_string());

    let started = Utc::now();

    let context = Context::create();
    let module = context.create_module("program");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None);

    if execution_engine.is_err() {
        println!("Error during start of JIT engine!");
        return (Err(execution_engine.err().unwrap().to_string()), None, None);
    }

    let execution_engine = execution_engine.ok().unwrap();

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let mut codegen: Option<CodeGen> = None;

    if CONFIG.jit_enabled {
        codegen = Option::from(CodeGen {
            context: &context,
            module: &module,
            builder: context.create_builder(),
            execution_engine,
            fpm: &fpm,
            last_popped: None,
            jumps: Vec::new(),
            section_depth: Vec::new(),
        });
    }

    let ran = vm.run(codegen);

    let duration = Utc::now().signed_duration_since(started);

    if ran.is_err() {
        panic!("{}", ran.err().unwrap());
    }

    if CONFIG.enable_benchmark {
        let formatted = duration.to_string().replace("PT", "");
        println!("Execution Took: {}", formatted);
    }

    (ran, Some(comp.get_state()), Some(vm.get_state()))
}
