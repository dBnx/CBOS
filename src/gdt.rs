use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

/// Loads the global descriptor table
pub fn init_gdt() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;
    println!("Load GDT ...");
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, SegmentSelectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            SegmentSelectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct SegmentSelectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/// Hardcoded interrupt stack tables
///
/// Needed if e.g. kernel stack overflows
/// -> double fault exception, but the CPU pushes the new stackframe onto the the full stack
/// -> tripple fault which means hardware reset
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum IST_INDEX {
    DOUBLE_FAULT = 0,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[IST_INDEX::DOUBLE_FAULT as usize] = {
            const STACK_SIZE: usize = 5 * 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}
