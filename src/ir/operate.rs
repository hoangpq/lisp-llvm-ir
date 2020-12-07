use llvm_sys::core::*;
use llvm_sys::*;

use crate::ir::llvm_type::int32_type;
use crate::ir::string::const_int;
use crate::RispEnv;
use std::os::raw::c_char;

#[allow(dead_code)]
pub fn build_ret(builder: *mut LLVMBuilder, llvm_value: *mut LLVMValue) -> *mut LLVMValue {
    unsafe { LLVMBuildRet(builder, llvm_value) }
}

#[allow(dead_code)]
pub fn build_const_gep(llvm_const_value: *mut LLVMValue) -> *mut LLVMValue {
    let mut args = vec![const_int(int32_type(), 0), const_int(int32_type(), 0)];
    unsafe { LLVMConstInBoundsGEP(llvm_const_value, args.as_mut_ptr(), args.len() as u32) }
}

#[allow(dead_code)]
pub fn build_position_at_end(builder: *mut LLVMBuilder, block: *mut LLVMBasicBlock) {
    unsafe {
        LLVMPositionBuilderAtEnd(builder, block);
    };
}

#[allow(dead_code)]
pub fn build_alloca(
    builder: *mut LLVMBuilder,
    llvm_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildAlloca(builder, llvm_type, c_string!(name).as_ptr()) }
}

#[allow(dead_code)]
pub fn build_store(
    builder: *mut LLVMBuilder,
    value: *mut LLVMValue,
    target: *mut LLVMValue,
) -> *mut LLVMValue {
    unsafe { LLVMBuildStore(builder, value, target) }
}

pub fn build_load(
    builder: *mut LLVMBuilder,
    llvm_value: *mut LLVMValue,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMBuildLoad(builder, llvm_value, c_string!(name).as_ptr()) }
}

pub fn build_int32_value(value: f64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(LLVMInt32Type(), value as u64, 0) }
}

#[allow(dead_code)]
pub fn llvm_int_value(env: &RispEnv, name: *const c_char, value: f64) -> *mut LLVMValue {
    let val = unsafe {
        LLVMBuildAlloca(
            env.llvm_builder,
            LLVMInt32TypeInContext(env.llvm_context),
            name,
        )
    };

    unsafe {
        LLVMBuildStore(
            env.llvm_builder,
            LLVMConstInt(LLVMInt32TypeInContext(env.llvm_context), value as u64, 0),
            val,
        );
    }

    val
}

pub fn add_function(
    target_module: *mut LLVMModule,
    function_type: *mut LLVMType,
    name: &str,
) -> *mut LLVMValue {
    unsafe { LLVMAddFunction(target_module, c_string!(name).as_ptr(), function_type) }
}

pub fn call_function(
    builder: *mut LLVMBuilder,
    function: *mut LLVMValue,
    mut args: Vec<*mut LLVMValue>,
    name: &str,
) -> *mut LLVMValue {
    unsafe {
        LLVMBuildCall(
            builder,
            function,
            args.as_mut_ptr(),
            args.len() as u32,
            c_string!(name).as_ptr(),
        )
    }
}
