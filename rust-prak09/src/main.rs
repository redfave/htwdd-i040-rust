use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    threading_channels();
    // threading_ddos();
}

fn threading_channels() -> () {
    let (tx_channel_1_no_1, rx_channel_1_no_1) = mpsc::channel::<i32>();
    let (tx_channel_2_no_1, rx_channel_2_no_1) = mpsc::channel::<i32>();

    let task_produce = move || {
        for i in 1..=1000 {
            let res = tx_channel_1_no_1.send(i);
            if res.is_err() {
                println!("Error while sending on channel 1: {:?}", res.err());
            }
        }
        drop(tx_channel_1_no_1);
    };

    let task_add = move || {
        loop {

            let summand_1 =  match rx_channel_1_no_1.recv() {
                Ok(value) => value,
                Err(_) => break,
            };

            let summand_2  = match  rx_channel_1_no_1.recv() {
                Ok(value) => value,
                Err(_) => break,
            }; 

                let multiplication_result = summand_1 * summand_2;
                let send_result = tx_channel_2_no_1.send(multiplication_result);

                if send_result.is_err() {
                    println!("Error while sending on channel 2: {:?}", send_result.err());
                }
            };

            drop(tx_channel_2_no_1);
    };

    let task_multiply = move || {
        loop {
            let factor_1 = match rx_channel_2_no_1.recv() {
                Ok(value) => value as i64,
                Err(_) => break,
            };
            let factor_2 = match rx_channel_2_no_1.recv() {
                    Ok(value) => value as i64,
                Err(_) => break,
            };
            
                let res_multiplication: i64 = factor_1 * factor_2;
                println!("Received multiplication result: {res_multiplication}");
            }
        };
    

    let thread_1 = thread::spawn(task_add);
    let thread_2 = thread::spawn(task_multiply);
    let thread_3 = thread::spawn(task_produce);
    let _ = thread_3.join();
    let _ = thread_2.join();
    let _ = thread_1.join();
    println!("Finished all threads");
}

fn threading_ddos() -> () {
    for i in 1..=i32::max_value() {
        thread::spawn(move || {
            println!("New thread with ID {i} created!");
            thread::sleep(Duration::from_secs(i32::max_value().try_into().unwrap()));
            println!("Thread with ID {i} terminated!");
        });
    }
}
