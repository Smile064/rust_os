#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée au niveau de Rust

use core::panic::PanicInfo;

/// Cette fonction est appelée à chaque panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // ne pas décorer le nom de cette fonction
pub extern "C" fn _start() -> ! {
   
    loop {}
}
