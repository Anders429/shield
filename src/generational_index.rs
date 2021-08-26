use array_init::array_init;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GenerationalIndex {
    pub(crate) index: usize,
    pub(crate) generation: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct AllocatableIndex {
    generation: usize,
    is_allocated: bool,
}

#[derive(Debug)]
struct FreeIndexQueue<const ENTITY_COUNT: usize> {
    start: usize,
    end: usize,
    indices: Box<[usize; ENTITY_COUNT]>,
    full: bool,
}

impl<const ENTITY_COUNT: usize> Default for FreeIndexQueue<ENTITY_COUNT> {
    fn default() -> Self {
        Self {
            start: 0,
            end: ENTITY_COUNT - 1,
            indices: Box::new(array_init(|index| index)),
            full: false,
        }
    }
}

impl<const ENTITY_COUNT: usize> FreeIndexQueue<ENTITY_COUNT> {
    fn allocate_index(&mut self) -> Option<usize> {
        if self.full {
            return None;
        }
        let index = unsafe {
            // SAFETY: `self.start` is invariantly guaranteed to be within the bounds of
            // `self.indices`.
            self.indices.get_unchecked(self.start)
        };
        if self.start == self.end {
            self.full = true;
        }
        self.start += 1;
        if self.start >= ENTITY_COUNT {
            self.start = 0;
        }
        Some(*index)
    }

    unsafe fn free_index(&mut self, index: usize) {
        self.full = false;
        self.end += 1;
        if self.end >= ENTITY_COUNT {
            self.end = 0;
        }
        *self.indices.get_unchecked_mut(self.end) = index;
    }
}

#[derive(Debug)]
pub(crate) struct Allocator<const ENTITY_COUNT: usize> {
    indices: Box<[AllocatableIndex; ENTITY_COUNT]>,
    free_indices: FreeIndexQueue<ENTITY_COUNT>,
}

impl<const ENTITY_COUNT: usize> Default for Allocator<ENTITY_COUNT> {
    fn default() -> Self {
        Self {
            indices: Box::new([AllocatableIndex::default(); ENTITY_COUNT]),
            free_indices: FreeIndexQueue::default(),
        }
    }
}

impl<const ENTITY_COUNT: usize> Allocator<ENTITY_COUNT> {
    pub(crate) fn allocate(&mut self) -> Option<GenerationalIndex> {
        let index = self.free_indices.allocate_index()?;
        let entity = unsafe {
            // SAFETY: The `index` obtained from `self.free_indices` is guaranteed to be within the
            // bounds of `self.indices`.
            self.indices.get_unchecked_mut(index)
        };
        let generation = entity.generation;
        debug_assert!(!entity.is_allocated, "Double allocation of index.");
        entity.is_allocated = true;
        Some(GenerationalIndex { index, generation })
    }

    pub(crate) unsafe fn deallocate_unchecked(
        &mut self,
        generational_index: GenerationalIndex,
    ) -> bool {
        let allocatable_index = self.indices.get_unchecked_mut(generational_index.index);
        if allocatable_index.generation != generational_index.generation
            || !allocatable_index.is_allocated
        {
            return false;
        }
        allocatable_index.generation = allocatable_index.generation.wrapping_add(1);
        allocatable_index.is_allocated = false;
        self.free_indices.free_index(generational_index.index);
        true
    }

    pub(crate) unsafe fn is_allocated_unchecked(
        &self,
        generational_index: GenerationalIndex,
    ) -> bool {
        let index = self.indices.get_unchecked(generational_index.index);
        index.generation == generational_index.generation && index.is_allocated
    }
}

#[cfg(test)]
mod tests {
    use super::{Allocator, FreeIndexQueue, GenerationalIndex};
    use claim::{assert_none, assert_some_eq};

    #[test]
    fn free_index_queue_allocate_available() {
        let mut queue = FreeIndexQueue::<1>::default();

        assert_some_eq!(queue.allocate_index(), 0);
    }

    #[test]
    fn free_index_queue_allocate_unavailable() {
        let mut queue = FreeIndexQueue::<1>::default();

        assert_some_eq!(queue.allocate_index(), 0);
        assert_none!(queue.allocate_index());
    }

    #[test]
    fn free_index_queue_allocate_multiple_available() {
        let mut queue = FreeIndexQueue::<2>::default();

        assert_some_eq!(queue.allocate_index(), 0);
        assert_some_eq!(queue.allocate_index(), 1);
    }

    #[test]
    fn free_index_queue_free() {
        let mut queue = FreeIndexQueue::<1>::default();

        assert_some_eq!(queue.allocate_index(), 0);

        unsafe {
            queue.free_index(0);
        }

        assert_some_eq!(queue.allocate_index(), 0);
    }

    #[test]
    fn allocator_allocate_available() {
        let mut allocator = Allocator::<1>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );
    }

    #[test]
    fn allocator_allocate_unavailable() {
        let mut allocator = Allocator::<1>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );
        assert_none!(allocator.allocate());
    }

    #[test]
    fn allocator_allocate_multiple() {
        let mut allocator = Allocator::<2>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );
        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 1,
                generation: 0
            }
        );
    }

    #[test]
    fn allocator_increments_generation() {
        let mut allocator = Allocator::<1>::default();

        let index = allocator.allocate();
        assert_some_eq!(
            index,
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );
        assert!(unsafe { allocator.deallocate_unchecked(index.unwrap()) });
        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 1
            }
        );
    }

    #[test]
    fn allocator_deallocate_wrong_generation() {
        let mut allocator = Allocator::<1>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );

        assert!(!unsafe {
            allocator.deallocate_unchecked(GenerationalIndex {
                index: 0,
                generation: 1,
            })
        });
    }

    #[test]
    fn allocator_deallocate_free_index() {
        let mut allocator = Allocator::<1>::default();

        assert!(!unsafe {
            allocator.deallocate_unchecked(GenerationalIndex {
                index: 0,
                generation: 0,
            })
        });
    }

    #[test]
    fn allocator_is_allocated_false() {
        let mut allocator = Allocator::<1>::default();

        assert!(!unsafe {
            allocator.is_allocated_unchecked(GenerationalIndex {
                index: 0,
                generation: 0,
            })
        });
    }

    #[test]
    fn allocator_is_allocated() {
        let mut allocator = Allocator::<1>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );

        assert!(unsafe {
            allocator.is_allocated_unchecked(GenerationalIndex {
                index: 0,
                generation: 0,
            })
        });
    }

    #[test]
    fn allocator_is_allocated_wrong_generation() {
        let mut allocator = Allocator::<1>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );

        assert!(!unsafe {
            allocator.is_allocated_unchecked(GenerationalIndex {
                index: 0,
                generation: 1,
            })
        });
    }

    #[test]
    fn allocator_is_allocated_wrong_index() {
        let mut allocator = Allocator::<2>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );

        assert!(!unsafe {
            allocator.is_allocated_unchecked(GenerationalIndex {
                index: 1,
                generation: 0,
            })
        });
    }

    #[test]
    fn allocator_is_allocated_after_deallocation() {
        let mut allocator = Allocator::<1>::default();

        assert_some_eq!(
            allocator.allocate(),
            GenerationalIndex {
                index: 0,
                generation: 0
            }
        );

        assert!(unsafe {
            allocator.deallocate_unchecked(GenerationalIndex {
                index: 0,
                generation: 0,
            })
        });

        assert!(!unsafe {
            allocator.is_allocated_unchecked(GenerationalIndex {
                index: 0,
                generation: 0,
            })
        });
    }
}
