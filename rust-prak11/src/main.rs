use rust_prak11_lib::{Car, CarList};
use std::{sync::Arc, sync::RwLock, thread};

fn main() {
    use_custom_heap_list();
    // overflow_heap();
}

fn use_custom_heap_list() -> () {
    let car_1 = Car {
        brand: String::from("VW"),
        model: String::from("Passat"),
        hp: 30,
        cool: false,
    };

    let car_2 = Car {
        brand: String::from("BMW"),
        model: String::from("X6"),
        hp: 300,
        cool: true,
    };

    let car_3 = Car {
        brand: String::from("Fiat"),
        model: String::from("Multipla"),
        hp: 2,
        cool: false,
    };

    let car_4 = Car {
        brand: String::from("Mercedes"),
        model: String::from("AMG"),
        hp: 270,
        cool: true,
    };

    let car_5 = Car {
        brand: String::from("Mazda"),
        model: String::from("MX-5"),
        hp: 120,
        cool: false,
    };

    let list = CarList::Cons(
        car_1,
        Box::new(CarList::Cons(
            car_2,
            Box::new(CarList::Cons(
                car_3,
                Box::new(CarList::Cons(
                    car_4,
                    Box::new(CarList::Cons(car_5, Box::new(CarList::Nil))),
                )),
            )),
        )),
    );
     println!("Cars List {:?}", list);
}

fn overflow_heap() -> () {
    const MB_IN_CHARS: usize = (1 << 20) / 4; // 2^20 (1 MB in Byte). A char takes up 4 bytes.

    let char_arr_1_mb: Box<[char]> = vec!['a'; MB_IN_CHARS].into_boxed_slice();

    let counter_lock: Arc<RwLock<u32>> = Arc::new(RwLock::new(0));
    let heap_dumpsite: Arc<RwLock<Vec<Box<[char]>>>> = Arc::new(RwLock::new(Vec::new()));

    let threads_available: usize = thread::available_parallelism().unwrap().get();

    for _thread_index in 0..threads_available {
        let heap = Arc::clone(&heap_dumpsite);
        let arr = char_arr_1_mb.clone();
        let counter = Arc::clone(&counter_lock);

        thread::spawn(move || {
            loop {
                heap.write().unwrap().push(arr.clone());
                {
                    let mut counter_write = counter.write().unwrap();
                    *counter_write += 1;
                }
                println!("MB in use: {}", counter.read().unwrap());
            }
        });
    }
    std::thread::park();
}
