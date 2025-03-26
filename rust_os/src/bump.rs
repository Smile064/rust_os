use crate::{align_up, Locked};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

/// Allocateur Bump (Allocation linéaire sans fragmentation)
pub struct BumpAllocator {
    heap_start: usize,    // Début de la plage mémoire
    heap_end: usize,      // Fin de la plage mémoire
    next: usize,          // Prochaine adresse disponible
    allocations: usize,   // Nombre d'allocations actives
}

impl BumpAllocator {
    /// Crée un nouvel allocateur vide
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initialise l'allocateur avec une plage mémoire donnée
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        assert!(heap_size > 0, "Heap size must be greater than zero.");
        assert!(heap_start.checked_add(heap_size).is_some(), "Heap size overflow.");

        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    /// Alloue une région de mémoire selon le layout spécifié
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();
        let alloc_start = align_up(bump.next, layout.align()); // Adresse alignée
        
        let alloc_end = alloc_start.checked_add(layout.size());
        if alloc_end.is_none() || alloc_end.unwrap() > bump.heap_end {
            return ptr::null_mut(); // Échec si dépasse la mémoire disponible
        }

        bump.next = alloc_end.unwrap(); // Avance le pointeur
        bump.allocations += 1; // Incrémente le compteur d'allocations
        alloc_start as *mut u8
    }

    /// Libère une allocation (réinitialise si tout est désalloué)
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();
        if bump.allocations > 0 {
            bump.allocations -= 1; // Décrémente le compteur
        }

        if bump.allocations == 0 {
            bump.next = bump.heap_start; // Réinitialise si aucune allocation active
        }
    }
}
