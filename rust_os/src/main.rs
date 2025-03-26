#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée au niveau de Rust

extern crate alloc;

use alloc::alloc::{alloc, dealloc, Layout};
use core::ptr;
use core::panic::PanicInfo;

#[global_asm]
extern "C" {
    fn _start();
}

// Struct de l'allocateur simple
struct SimpleAllocator {
    start: *mut u8,
    end: *mut u8,
    current: *mut u8,
}

impl SimpleAllocator {
    pub fn new(start: *mut u8, size: usize) -> Self {
        let end = unsafe { start.add(size) };
        Self {
            start,
            end,
            current: start,
        }
    }
    
    pub unsafe fn allocate(&mut self, layout: Layout) -> *mut u8 {
        let alignment = layout.align();
        let size = layout.size();

        // Arrondir `current` pour respecter l'alignement
        let current_ptr = self.current as usize;
        let aligned_ptr = (current_ptr + (alignment - 1)) & !(alignment - 1);
        let new_ptr = aligned_ptr as *mut u8;

        // Vérifier si on dépasse la mémoire disponible
        if new_ptr.add(size) > self.end {
            return ptr::null_mut();
        }

        self.current = new_ptr.add(size); // Mise à jour du pointeur
        new_ptr
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8, _layout: Layout) {
        // Pas de gestion de la libération ici pour simplifier
    }
}

// Implémentation de `GlobalAlloc` pour utiliser notre allocateur
unsafe impl alloc::alloc::GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocate(ptr, layout)
    }
}

/// Cette fonction est appelée à chaque panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // ne pas décorer le nom de cette fonction
pub extern "C" fn _start() -> ! {

        // Initialisation de l'allocateur
        let memory_size = 1024; // 1 Ko de mémoire
        let memory_start = 0x1000 as *mut u8; // Adresse de base fictive
    
        let mut allocator = SimpleAllocator::new(memory_start, memory_size);
    
        unsafe {
            // Allocation de mémoire
            let layout = Layout::from_size_align(128, 8).unwrap();
            let ptr = allocator.alloc(layout);
            if !ptr.is_null() {
                // Faire quelque chose avec la mémoire allouée
            }
        }
    loop {}
}
