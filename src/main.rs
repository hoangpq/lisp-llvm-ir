extern crate clap;
extern crate llvm_sys;

use std::ffi::CString;

#[macro_use]
mod macros;

mod builtin;
mod ir;

use llvm_sys::core::*;

use std::fmt;
use std::num::ParseFloatError;

use llvm_sys::{LLVMModule, LLVMType, LLVMValue};

use crate::ir::llvm_type::{int32_type, int8_type, pointer_type};
use crate::ir::operate::{add_function, build_ret};

use crate::builtin::env::{default_env, RispEnv};
use crate::builtin::ir::{eval_arithmetic, eval_input_fn, eval_number, eval_printf_fn};

type RispCallback = fn(&RispEnv, &[RispExp], Option<*mut LLVMValue>) -> Result<RispExp, RispErr>;
// (ref, loaded)
type LLVMValueWrapper = (*mut LLVMValue, bool);

// three kinds of values
#[derive(Clone)]
pub enum RispExp {
    Null,
    Symbol(String),
    Number(f64, LLVMValueWrapper),
    List(Vec<RispExp>),
    Func(String, RispCallback), // bam
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

pub fn function_type_var_arg(ret_type: *mut LLVMType, args: &mut [*mut LLVMType]) -> *mut LLVMType {
    unsafe { LLVMFunctionType(ret_type, args.as_mut_ptr(), 0, 1) }
}

pub fn create_printf(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut args_type_list = vec![pointer_type()];
    let printf_type = function_type_var_arg(int8_type(), &mut args_type_list);

    add_function(module, printf_type, "printf")
}

pub fn create_input_fn(module: *mut LLVMModule) -> *mut LLVMValue {
    let mut args_type_list = vec![pointer_type()];
    let fn_type = function_type_var_arg(int32_type(), &mut args_type_list);

    add_function(module, fn_type, "scanf")
}

impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            RispExp::Symbol(s) => s.clone(),
            RispExp::Number(n, _) => n.to_string(),
            RispExp::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            RispExp::Func(f_name, _) => format!("Function {}", f_name),
            RispExp::Null => "null".to_string(),
        };

        write!(f, "{}", str)
    }
}

fn tokenize(expr: &str) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(env: &RispEnv, tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get token".to_string()))?;

    match &token[..] {
        "(" => read_seq(env, rest),
        ")" => Err(RispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(env, token), rest)),
    }
}

fn read_seq<'a>(env: &RispEnv, tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let mut res: Vec<RispExp> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(RispErr::Reason("could not find closing `)`".to_string()))?;

        if next_token == ")" {
            return Ok((RispExp::List(res), rest));
        }
        let (exp, new_xs) = parse(env, &xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(_env: &RispEnv, token: &str) -> RispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(value) => RispExp::Number(value, (llvm_integer!(value as u64), false)),
        Err(_) => RispExp::Symbol(token.to_string()).clone(),
    }
}

fn get_symbol(env: &RispEnv, k: &String) -> Result<RispExp, RispErr> {
    // println!("Symbol: {}", k);
    env.data
        .get(k)
        .ok_or(RispErr::Reason(format!("unexpected symbol k='{}'", k)))
        .map(|x| x.clone())
}

fn eval_function(
    env: &mut RispEnv,
    f_name: &String,
    func: &RispCallback,
    arg_forms: &[RispExp],
) -> Result<RispExp, RispErr> {
    let args_eval = arg_forms
        .iter()
        .map(|x| eval(x, env))
        .collect::<Result<Vec<RispExp>, RispErr>>();

    match &f_name[..] {
        "+" => eval_arithmetic(env, f_name, func, args_eval),
        "-" => eval_arithmetic(env, f_name, func, args_eval),
        "*" => eval_arithmetic(env, f_name, func, args_eval),
        "/" => eval_arithmetic(env, f_name, func, args_eval),
        "printf" => eval_printf_fn(env, func, args_eval),
        "input" => eval_input_fn(env, func),
        _ => Err(RispErr::Reason("function not found".to_string())),
    }
}

fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        RispExp::Symbol(k) => get_symbol(env, k),
        RispExp::Number(val, _) => Ok(eval_number(env, *val)),
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("expected a non-empty list".to_string()))?;

            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;

            // if the first one is function
            match first_eval {
                RispExp::Func(f_name, func) => eval_function(env, &f_name, &func, arg_forms),
                _ => Err(RispErr::Reason("first form must be a function".to_string())),
            }
        }
        RispExp::Func(_, _) => Err(RispErr::Reason("unexpected form".to_string())),
        RispExp::Null => Ok(RispExp::Null),
    }
}

fn parse_eval(env: &mut RispEnv, exp: &str) -> Result<RispExp, RispErr> {
    let (parsed_exp, _) = parse(env, &tokenize(exp))?;
    let exp = eval(&parsed_exp, env)?;

    if let RispExp::Number(_, llvm_ref) = exp {
        build_ret(env.llvm_builder, llvm_ref.0);
    }

    Ok(exp)
}

fn main() -> Result<(), RispErr> {
    let env = &mut default_env();
    let _test1 = "(printf (- (input) 29))";
    let _test2 = r#"(printf (- (input) (+ (input) 29)))"#;
    let _test3 = r#"(printf (- 92 (input)))"#;

    if let Err(error) = parse_eval(env, _test2) {
        println!("{:?}", error);
    };

    env.emit_file("output.ll");

    Ok(())
}
