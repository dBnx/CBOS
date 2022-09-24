pub fn hlt() {
    x86_64::instructions::hlt();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
