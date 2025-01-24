//! The ZKVM target covers [Risc0](https://github.com/risc0/risc0) and [SP1](https://github.com/succinctlabs/sp1) programs 
//! as they have the same target OS names and syscall signatures.

use core::mem::MaybeUninit;
use crate::Error;

pub use crate::util::{inner_u32, inner_u64};

extern "C" {
    fn sys_rand(recv_buf: *mut u8, words: usize);
}

pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    unsafe {
        sys_rand(dest.as_mut_ptr() as *mut u8, dest.len());
    }

    Ok(())
}
