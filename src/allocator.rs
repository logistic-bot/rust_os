use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

/// Start of heap
pub const HEAP_START: usize = 0x4444_4444_0000;
/// Size of heap
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB FIXME: Automatically determine an appropriate size, or dynamically grow the heap

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Implements a simple bump allocator
pub mod bump;

/// Initialize the heap.
///
/// Maps heap pages to physical frames, and initializes the allocator.
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // Map heap pages to physical frames
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    /// New Locked object. Allows for trait implementation. Wraps around spin::Mutex.
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    /// Lock inner mutex
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align the given address upwards to the given alignment.
fn align_up(addr: usize, alignment: usize) -> usize {
    let remainder = addr % alignment;
    if remainder == 0 {
        addr
    } else {
        addr - remainder + alignment
    }
}
