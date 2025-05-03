use x86_64::{structures::paging::PageTable, VirtAddr};

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    // read physical frame
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    // get virtual memory offset where the level 4 page table frame is mapped
    let virt = physical_memory_offset + phys.as_u64();
    // convert virtual addr to *mut PageTable raw ptr
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    // return mutable reference from raw pointer
    unsafe { &mut *page_table_ptr }
}
