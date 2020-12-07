use crate::builtin::env::RispEnv;
use crate::ir::llvm_type::int32_type;
use crate::ir::operate::{build_alloca, build_int32_value, build_load, build_store, call_function};
use crate::ir::string::codegen_string;
use crate::{LLVMValueWrapper, RispCallback, RispErr, RispExp};
use llvm_sys::LLVMValue;
use std::io;
use std::ptr::null_mut;

#[allow(dead_code)]
pub fn wrap_llvm_value(value: f64, llvm_ref: LLVMValueWrapper) -> RispExp {
    RispExp::Number(value, llvm_ref)
}

#[allow(dead_code)]
pub fn unwrap_object(exp: &RispExp) -> *mut LLVMValue {
    match *exp {
        RispExp::Number(_f, (llvm_ref, _)) => llvm_ref,
        _ => panic!("failed to unwrap object: {}", exp),
    }
}

pub fn load_llvm_value(env: &RispEnv, exp: &RispExp) -> *mut LLVMValue {
    match exp {
        RispExp::Number(_, value_ref) => {
            if !value_ref.1 {
                build_load(env.llvm_builder, value_ref.0, "")
            } else {
                value_ref.0
            }
        }
        _ => 0 as *mut LLVMValue
    }
}

pub fn eval_number(env: &RispEnv, f: f64) -> RispExp {
    let llvm_input = build_alloca(env.llvm_builder, int32_type(), "");
    build_store(env.llvm_builder, build_int32_value(f), llvm_input);
    // let llvm_input = build_load(env.llvm_builder, llvm_input, "");
    RispExp::Number(f, (llvm_input, false))
}

// printf keyword
pub fn eval_printf_fn(
    env: &mut RispEnv,
    func: &RispCallback,
    args_eval: Result<Vec<RispExp>, RispErr>,
) -> Result<RispExp, RispErr> {
    let args_eval = args_eval?;
    let first_arg = args_eval.first();
    let mut llvm_val: *mut LLVMValue = null_mut();

    // emit IR
    if let Some(RispExp::Number(_, llvm_ref)) = first_arg {
        let llvm_ref = *llvm_ref;

        let printf = env.built_ins["printf"];
        let llvm_value = build_alloca(env.llvm_builder, int32_type(), "");

        build_store(env.llvm_builder, llvm_ref.0, llvm_value);

        let print_int = build_load(env.llvm_builder, llvm_value, "");
        let printf_args = vec![codegen_string(env, "Result: %d\n", ""), print_int];

        call_function(env.llvm_builder, printf, printf_args, "");
        llvm_val = llvm_ref.0;
    }

    // eval print
    func(env, &args_eval, Some(llvm_val))
}

// input keyword
pub fn eval_input_fn(env: &mut RispEnv, func: &RispCallback) -> Result<RispExp, RispErr> {
    let input_fn = env.built_ins["input"];

    let llvm_input = build_alloca(env.llvm_builder, int32_type(), "input");
    let input_args = vec![codegen_string(env, "%u", ""), llvm_input];

    // emit IR
    call_function(env.llvm_builder, input_fn, input_args, "");

    // eval
    func(env, &[], Some(llvm_input))
}

// arithmetic
pub fn eval_arithmetic(
    env: &mut RispEnv,
    _op: &str,
    func: &RispCallback,
    args_eval: Result<Vec<RispExp>, RispErr>,
) -> Result<RispExp, RispErr> {
    let risp_args = args_eval.unwrap();
    func(env, &risp_args, None)
}

// utils
fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(num, _) => Ok(*num),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }
    input.trim().to_string()
}

pub fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    args.iter().map(|x| parse_single_float(x)).collect()
}
