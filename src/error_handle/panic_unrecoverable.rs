fn call_panic_self() {
    // thread 'main' panicked at 'test panic error.', src/error_handle/panic_unrecoverable.rs:2:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    panic!("test panic error.");
}

fn cause_lib_panic() {
    let v = vec![12, 23, 43];
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10', src/error_handle/panic_unrecoverable.rs:9:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // RUST_BACKTRACE=1 cargo run可以看到完整的堆栈
    v[10];
}

pub fn panic_unrecover_error_study() {
    // call_panic_self();
    cause_lib_panic();
}
