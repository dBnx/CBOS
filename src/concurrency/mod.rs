use alloc::boxed::Box;
pub type Task = dyn 'static + FnOnce() + Send + Sync;

#[repr(C)]
pub struct TCB {
    stack: Box<[u8]>,
    work: Box<Task>,
    // RIP, RSP, CR3, other registers...?
}
