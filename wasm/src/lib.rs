use std::{ffi::c_void, mem};

use matters_lib::{Operation, Options};
use rand::SeedableRng;

const FLAG_ADDITION: u8 = 0b0000_0001;
const FLAG_SUBTRACTION: u8 = 0b0000_0010;
const FLAG_MULTIPLICATION: u8 = 0b0000_0100;
const FLAG_DIVISION: u8 = 0b0000_1000;

/// WASM calculation result.
#[repr(C)]
pub struct Result(*mut c_void);

#[no_mangle]
pub extern "C" fn generate_problems(
    problems: usize,
    max: u32,
    max_factor: u32,
    operations_flag: u8,
    allow_negative: u8,
) -> Result {
    let operations = get_operations(operations_flag);
    let mut rng = rand::rngs::SmallRng::seed_from_u64(random::seed());

    let options = Options {
        problems,
        max,
        max_factor,
        operations,
        allow_negative: allow_negative > 0,
    };
    let Ok(mut problems) = matters_lib::generate_problems(&options, &mut rng).map_err(|err| {
        logger::error(err.kind);
        err
    }) else {
        std::process::abort();
    };

    let ptr = problems.as_mut_ptr().cast();

    mem::forget(problems);

    Result(ptr)
}

fn get_operations(flags: u8) -> Vec<Operation> {
    let mut ops = Vec::new();

    if flags & FLAG_ADDITION > 0 {
        ops.push(Operation::Addition);
    }
    if flags & FLAG_SUBTRACTION > 0 {
        ops.push(Operation::Subtraction);
    }
    if flags & FLAG_MULTIPLICATION > 0 {
        ops.push(Operation::Multiplication);
    }
    if flags & FLAG_DIVISION > 0 {
        ops.push(Operation::Division);
    }

    ops
}

/// Allocate memory of `size` bytes.
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

/// Deallocate all memory.
///
/// # Safety
///
/// This will deallocate `size` amount of memory pointed to by `ptr`.
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void, size: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, size);
}
#[allow(dead_code)]
mod logger {
    use matters_lib::error::ErrorKind;

    mod js_logger {
        use super::ErrorKind;

        #[link(wasm_import_module = "logger")]
        extern "C" {
            pub fn info(data: u32);
            pub fn error(code: ErrorKind);
        }
    }

    pub fn info(data: u32) {
        unsafe { js_logger::info(data) }
    }

    pub fn error(code: ErrorKind) {
        unsafe { js_logger::error(code) }
    }
}

mod random {
    mod js_random {
        #[link(wasm_import_module = "random")]
        extern "C" {
            pub fn seed() -> u64;
        }
    }

    pub fn seed() -> u64 {
        unsafe { js_random::seed() }
    }
}
