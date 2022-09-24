/// For debugging only! Requires that the timer interrupts is set.
pub fn sleep_for_some_time(iterations: usize) {
    for _ in 0..iterations {
        x86_64::instructions::hlt();
        //volatile::Volatile::new(0).read(); // preventoptimizations
    }
}
