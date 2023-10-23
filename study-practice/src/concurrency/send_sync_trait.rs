use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SendBox(*mut u8);

// `*const u8` cannot be sent between threads safely
unsafe impl Send for SendBox {}

fn send_impl() {
    let box1 = SendBox(5 as *mut u8);
    let handle1 = thread::spawn(move || {
        println!("in send impl {:?}", box1);
    });
    handle1.join().unwrap();
}

#[derive(Debug)]
struct SyncBox(*const u8);
unsafe impl Send for SyncBox {}
unsafe impl Sync for SyncBox {}

fn sync_impl() {
    let box1 = SyncBox(5 as *const u8);
    let val = Arc::new(Mutex::new(box1));
    let handle1 = thread::spawn(move || {
        let v = val.lock().unwrap();
        println!("in sync box: {:?}", v);
    });
    handle1.join().unwrap();
}

pub fn practice() {
    send_impl();
    sync_impl();
}
