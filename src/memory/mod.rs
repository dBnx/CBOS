//! From: https://os.phil-opp.com/paging-introduction/#paging-on-x86-64
//! The `x86_64` architecture uses a 4-level page table and a page size of 4 KiB. Each page table,
//! independent of the level, has a fixed size of 512 entries. Each entry has a size of 8 bytes,
//! so each table is 512 * 8 B = 4 KiB large and thus fits exactly into one page.
//! [...] Bits 48 to 64 are discarded, which means that `x86_64` is not really 64-bit since it only
//! supports 48-bit addresses.
//! They are sign-extended to be future-compatible (else CPU exceptions)
//!
//! How does 4-level page table work for x86_64 systems:
//! CR3 holds the _physical_ addr of L4 Page Table.
//! Then the physical address of `address` is given by
//! L3PT = L4PT[ address[47:40] ]
//! L2PT = L3PT[ address[39:30] ]
//! L1PT = L2PT[ address[29:21] ]
//! Phys = L1PT[ address[21:12] ] + address[11:0]
//!
//! All addresses in the page table are physical.
//! Permissions on higher page tables restrict the lower permissions
//!
//! Bits 62-52 and 11-9 are freely usable by the OS.
//!

use bootloader::{
    bootinfo::{MemoryMap, MemoryRegionType},
    BootInfo,
};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

const FRAME_SIZE_NORMAL: usize = 4 * 1024;
const FRAME_SIZE_HUGE_2MB: usize = 512 * FRAME_SIZE_NORMAL;
const FRAME_SIZE_HUGE_1GB: usize = 512 * FRAME_SIZE_HUGE_2MB;

pub mod allocator;

pub fn init(boot_info: &'static BootInfo) {
    println!("Initialising page and heap allocator ...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_mapper(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let total_pages = frame_allocator.available_frames();
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let avail_pages = frame_allocator.available_frames();
    println!(
        "Available pages: {} | Heap uses {} pages and has {} kiB",
        avail_pages,
        total_pages - avail_pages,
        allocator::HEAP_SIZE / 1024
    );
    if let Some(region) = boot_info.memory_map.last() {
        let total_memory_kb = region.range.end_frame_number * 4;
        println!("Total physical memory: {} MiB", total_memory_kb / 1024);
    }
}

unsafe fn init_mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let l4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(l4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// # Safety
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let page_table_ptr = active_level_4_table_inner(physical_memory_offset);
    &mut *page_table_ptr // unsafe
}

fn active_level_4_table_inner(physical_memory_offset: VirtAddr) -> *mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    page_table_ptr
}

/// A `FrameAllocator` that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a `FrameAllocator` from the passed memory map.
    ///
    /// # Safety
    /// The frames not in the memory map must be unused
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Usable ergions
        let usable_regions = self
            .memory_map
            .iter()
            .filter(|region| region.region_type == MemoryRegionType::Usable);
        let addr_ranges =
            usable_regions.map(|region| region.range.start_addr()..region.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(FRAME_SIZE_NORMAL));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    pub fn available_frames(&self) -> usize {
        self.usable_frames().count() - self.next
    }
}

/// # Safety
/// May only return unused physical frames.
unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
