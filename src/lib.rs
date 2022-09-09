#[macro_export]
macro_rules! NO_IMPL {
    () => {
        panic!("NOT YET IMPLEMENTED, EXITING...")
    };
}

pub fn bit(a: u8, n: u8) -> u8 {
    return a & (1 << n);
}