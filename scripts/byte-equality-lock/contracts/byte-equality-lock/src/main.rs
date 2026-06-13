#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

use error::Error;
use ckb_std::ckb_constants::Source;

mod error;

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "library", test)))]
// By default, the following heap configuration is used:
// * 16KB fixed heap
// * 1.2MB(rounded up to be 16-byte aligned) dynamic heap
// * Minimal memory block in dynamic heap is 64 bytes
// For more details, please refer to ckb-std's default_alloc macro
// and the buddy-alloc alloc implementation.
ckb_std::default_alloc!(16384, 1258306, 64);

pub fn program_entry() -> i8 {
    ckb_std::debug!("This is a simple Byte equality Lock Script that passes when the bytes supplied at spend time = bytes baked in the lock.");
    ckb_std::debug!("Note that its insecure by design purely for learning purposes");

    match check_equality() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
    
}

fn check_equality() -> Result<(), Error> {

    let index = 0;

    let this_script = ckb_std::high_level::load_script()?;

    let this_script_args = this_script.args().raw_data();

    let witness_args = ckb_std::high_level::load_witness_args(index, Source::GroupInput)?;

    let witness_args_lock = match witness_args.lock().to_opt(){
        Some(bytes) => bytes.raw_data(),
        None => return Err(Error::WrongSecret),
    };

    if this_script_args == witness_args_lock {
        Ok(())
    }else {
        Err(Error::WrongSecret)
    }


}
