#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate litecoin;

use alloc::string::ToString;
use alloc::vec;
use core::panic::PanicInfo;

use alloc_cortex_m::CortexMHeap;
// use panic_halt as _;
use litecoin::{Address, Network, PrivateKey};
use litecoin::secp256k1::ffi::types::AlignedType;
use litecoin::secp256k1::Secp256k1;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024 * 256; // 256 KB

#[entry]
fn main() -> ! {
    hprintln!("heap size {}", HEAP_SIZE).unwrap();

    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let size = Secp256k1::preallocate_size();
    hprintln!("secp buf size {}", size*16).unwrap();

    // Load a private key
    let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
    let pk = PrivateKey::from_wif(raw).unwrap();
    hprintln!("Seed WIF: {}", pk).unwrap();

    let mut buf_ful = vec![AlignedType::zeroed(); size];
    let secp = Secp256k1::preallocated_new(&mut buf_ful).unwrap();

    // Derive address
    let pubkey = pk.public_key(&secp);
    let address = Address::p2wpkh(&pubkey, Network::Litecoin).unwrap();
    hprintln!("Address: {}", address).unwrap();

    assert_eq!(address.to_string(), "bc1qpx9t9pzzl4qsydmhyt6ctrxxjd4ep549np9993".to_string());
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hprintln!("panic {:?}", info.message()).unwrap();
    debug::exit(debug::EXIT_FAILURE);
    loop {}
}
