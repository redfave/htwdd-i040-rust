use std::thread::{JoinHandle, available_parallelism, spawn};

fn main() {
    race_condition();

    let valid_addr: usize = main as usize;
    const MEM_LEN: usize = 100;
    inspect_mem(valid_addr, MEM_LEN);
    println!("Passed through presumably valid memory region");

    let invalid_addr: usize = usize::MAX;
    inspect_mem(invalid_addr, MEM_LEN); // supposed to crash
}

fn inspect_mem(start_addr: usize, len: usize) -> () {
    for i in 0..len {
        let idx_addr = start_addr + i;
        let ptr = idx_addr as *const usize;
        unsafe {
            println!("Address {:?}\t{:x}", ptr, *ptr);
        }
    }
}

#[allow(static_mut_refs)]
fn race_condition() -> () {
    static mut IDX: u16 = 0;
    let max_threads: usize = available_parallelism().unwrap().get();
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::with_capacity(max_threads);

    for i in 0..max_threads {
        let handle: JoinHandle<()> = spawn(move || {
            unsafe {
                IDX += 1;
                println!("Thread {}: IDX = {}", i, IDX);
            }
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
