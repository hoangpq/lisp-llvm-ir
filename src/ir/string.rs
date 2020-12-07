use llvm_sys::core::*;
use llvm_sys::LLVMLinkage::*;
use llvm_sys::*;

use crate::ir::llvm_type::*;
use crate::ir::operate::build_const_gep;
use crate::ir::scope::*;
use crate::RispEnv;

#[allow(dead_code)]
pub fn const_int(llvm_type: *mut LLVMType, value: u64) -> *mut LLVMValue {
    unsafe { LLVMConstInt(llvm_type, value, 0) }
}

#[allow(dead_code)]
pub fn const_string_in_context(context: *mut LLVMContext, input_str: String) -> *mut LLVMValue {
    let temp_str = input_str
        .replace("\\n", "\u{0000A}")
        .replace("\\r", "\u{000D}")
        + "\0";
    let byte = temp_str.as_bytes();
    let length = byte.len() as u32;
    unsafe { LLVMConstStringInContext(context, byte.as_ptr() as *const _, length, 1) }
}

#[allow(dead_code)]
pub fn codegen_string(lc: &mut RispEnv, input_str: &str, name: &str) -> *mut LLVMValue {
    let str_val = const_string_in_context(lc.llvm_context, input_str.to_string());
    let global_str_val = add_global(lc.llvm_module, type_of(str_val), name);
    set_linkage(global_str_val, LLVMPrivateLinkage);
    set_initializer(global_str_val, str_val);
    set_global_constant(global_str_val);
    set_unnamed_address(global_str_val);

    return build_const_gep(global_str_val);
}

#[allow(dead_code)]
pub fn codegen_string_gep(lc: &mut RispEnv, input_str: &str, name: &str) -> *mut LLVMValue {
    let str_val = const_string_in_context(lc.llvm_context, input_str.to_string());
    let global_str_val = add_global(lc.llvm_module, type_of(str_val), name);
    set_linkage(global_str_val, LLVMPrivateLinkage);
    set_initializer(global_str_val, str_val);
    set_global_constant(global_str_val);
    set_unnamed_address(global_str_val);

    let mut args = [const_int(int32_type(), 0), const_int(int32_type(), 0)];

    unsafe {
        return LLVMBuildGEP(
            lc.llvm_builder,
            global_str_val,
            args.as_mut_ptr(),
            args.len() as u32,
            c_string!(name).as_ptr(),
        );
    }
}
