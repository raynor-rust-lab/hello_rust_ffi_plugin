use std::{time::Duration, usize};

#[no_mangle]
pub extern "C" fn sum(a: usize, b: usize) -> usize {
    a + b + 1
}

#[no_mangle]
pub extern "C" fn sum_long_running(a: usize, b: usize) -> usize {
    std::thread::sleep(Duration::from_secs(5));
    a + b + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
}