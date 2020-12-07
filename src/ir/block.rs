use llvm_sys::core::*;
use llvm_sys::*;

#[allow(dead_code)]
pub fn append_basic_block_in_context(
    context: *mut LLVMContext,
    function: *mut LLVMValue,
    function_name: &str,
) -> *mut LLVMBasicBlock {
    unsafe { LLVMAppendBasicBlockInContext(context, function, c_string!(function_name).as_ptr()) }
}
