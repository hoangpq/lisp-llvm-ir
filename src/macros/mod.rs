#[macro_use]
pub mod macros {
    macro_rules! c_str {
        ($s:expr) => {
            concat!($s, "\0").as_ptr() as *const i8
        };
    }

    #[macro_export]
    macro_rules! c_string {
        ($w:expr) => {
            crate::CString::new($w).unwrap()
        };
    }

    #[macro_export]
    macro_rules! string_from_raw {
        ($w:expr) => {
            unsafe { crate::CString::from_raw($w).into_string().unwrap() }
        };
    }

    #[macro_export]
    macro_rules! llvm_integer {
        ($value:expr) => {
            crate::ir::const_value::const_int(int32_type(), $value)
        };
    }
}
