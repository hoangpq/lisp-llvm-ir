use crate::builtin::ir::{
    eval_number, get_input, load_llvm_value, parse_list_of_floats,
};
use crate::ir::block::append_basic_block_in_context;
use crate::ir::llvm_type::{function_type, int32_type};
use crate::ir::operate::build_position_at_end;
use crate::{add_function, create_input_fn, create_printf, RispErr, RispExp};
use llvm_sys::core::{
    LLVMBuildAdd, LLVMBuildSub, LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext,
    LLVMDisposeBuilder, LLVMDumpModule, LLVMModuleCreateWithName, LLVMPrintModuleToFile,
};
use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef};
use llvm_sys::{LLVMBasicBlock, LLVMValue};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RispEnv {
    pub data: HashMap<String, RispExp>,
    pub llvm_context: LLVMContextRef,
    pub llvm_module: LLVMModuleRef,
    pub llvm_builder: LLVMBuilderRef,
    pub built_ins: HashMap<&'static str, *mut LLVMValue>,
}

impl RispEnv {
    unsafe fn new(data: HashMap<String, RispExp>) -> Self {
        let llvm_context = LLVMContextCreate();
        let mut env = RispEnv {
            data,
            llvm_context,
            llvm_module: LLVMModuleCreateWithName(c_str!("main_module")),
            llvm_builder: LLVMCreateBuilderInContext(llvm_context),
            built_ins: HashMap::new(),
        };
        env.setup_builtin();
        let (_, _) = env.setup_main();
        env
    }

    #[allow(dead_code)]
    pub fn setup_builtin(&mut self) {
        // print function
        self.built_ins
            .insert("printf", create_printf(self.llvm_module));
        self.built_ins
            .insert("input", create_input_fn(self.llvm_module));
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.llvm_module) }
    }

    #[allow(dead_code)]
    pub fn emit_file(&self, path: &str) {
        let mut error: *mut i8 = 0 as *mut i8;
        let buf: *mut *mut i8 = &mut error;
        let result = unsafe {
            LLVMPrintModuleToFile(self.llvm_module, c_string!(path).as_ptr() as *const _, buf)
        };

        if result > 0 {
            println!("{}", string_from_raw!(error));
        }
    }

    pub fn setup_main(&mut self) -> (*mut LLVMBasicBlock, *mut LLVMValue) {
        let fn_type = function_type(int32_type(), &mut []);
        let main_function = add_function(self.llvm_module, fn_type, "main");
        let block = append_basic_block_in_context(self.llvm_context, main_function, "entry");
        build_position_at_end(self.llvm_builder, block);

        (block, main_function)
    }
}

pub fn default_env() -> RispEnv {
    let mut data: HashMap<String, RispExp> = HashMap::new();

    data.insert(
        "+".to_string(),
        RispExp::Func(
            "+".to_string(),
            |env: &RispEnv,
             args: &[RispExp],
             _llvm_ref: Option<*mut LLVMValue>|
             -> Result<RispExp, RispErr> {
                let floats = parse_list_of_floats(args)?;
                let ret = floats.iter().fold(0.0, |sum, a| sum + a);

                if args.len() < 2 {
                    panic!("Missing argument!");
                }

                // IR
                let (arg1, arg2) = unsafe {
                    (
                        load_llvm_value(env, args.get_unchecked(0)),
                        load_llvm_value(env, args.get_unchecked(1)),
                    )
                };

                let llvm_ref =
                    unsafe { LLVMBuildAdd(env.llvm_builder, arg1, arg2, c_str!("add_ret")) };

                Ok(RispExp::Number(ret, (llvm_ref, true)))
            },
        ),
    );

    data.insert(
        "-".to_string(),
        RispExp::Func(
            "-".to_string(),
            |env: &RispEnv,
             args: &[RispExp],
             _llvm_ref: Option<*mut LLVMValue>|
             -> Result<RispExp, RispErr> {
                let floats = parse_list_of_floats(args)?;

                let first = *floats
                    .first()
                    .ok_or(RispErr::Reason("expected at least on number".to_string()))?;

                let s_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
                let ret = first - s_rest;

                if args.len() < 2 {
                    panic!("Missing argument!");
                }

                // IR
                let (arg1, arg2) = unsafe {
                    (
                        load_llvm_value(env, args.get_unchecked(0)),
                        load_llvm_value(env, args.get_unchecked(1)),
                    )
                };

                let llvm_ref =
                    unsafe { LLVMBuildSub(env.llvm_builder, arg1, arg2, c_str!("sub_ret")) };

                // let llvm_ref = build_load(env.llvm_builder, llvm_ref, "");

                Ok(RispExp::Number(ret, (llvm_ref, true)))
            },
        ),
    );

    data.insert(
        "*".to_string(),
        RispExp::Func(
            "*".to_string(),
            |env: &RispEnv,
             args: &[RispExp],
             llvm_ref: Option<*mut LLVMValue>|
             -> Result<RispExp, RispErr> {
                let floats = parse_list_of_floats(args)?;
                let ret = floats.iter().fold(1.0, |sum, a| sum * a);

                Ok(match llvm_ref {
                    Some(l_ref) => RispExp::Number(ret, (l_ref, false)),
                    None => eval_number(env, ret),
                })
            },
        ),
    );

    data.insert(
        "/".to_string(),
        RispExp::Func(
            "/".to_string(),
            |env: &RispEnv,
             args: &[RispExp],
             llvm_ref: Option<*mut LLVMValue>|
             -> Result<RispExp, RispErr> {
                let floats = parse_list_of_floats(args)?;
                let ret = floats.iter().fold(1.0, |sum, a| sum / a);

                Ok(match llvm_ref {
                    Some(l_ref) => RispExp::Number(ret, (l_ref, false)),
                    None => eval_number(env, ret),
                })
            },
        ),
    );

    data.insert(
        "printf".to_string(),
        RispExp::Func(
            "printf".to_string(),
            |_env: &RispEnv,
             args: &[RispExp],
             _llvm_ref: Option<*mut LLVMValue>|
             -> Result<RispExp, RispErr> {
                Ok(match args.first() {
                    Some(arg) => {
                        println!("{}", arg);
                        arg.clone()
                    }
                    _ => RispExp::Null
                })
            },
        ),
    );

    data.insert(
        "input".to_string(),
        RispExp::Func(
            "input".to_string(),
            |_env: &RispEnv,
             _args: &[RispExp],
             llvm_ref: Option<*mut LLVMValue>|
             -> Result<RispExp, RispErr> {
                let input = get_input("Type something...");

                Ok(match input.parse::<f64>() {
                    Ok(f) => RispExp::Number(f, (llvm_ref.unwrap(), false)),
                    Err(_) => RispExp::Null,
                })
            },
        ),
    );

    unsafe { RispEnv::new(data) }
}

impl Drop for RispEnv {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.llvm_builder);
            LLVMContextDispose(self.llvm_context);
        }
    }
}
