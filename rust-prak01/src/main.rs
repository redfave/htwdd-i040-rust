use cached::proc_macro::cached;

fn main() {
    const MAX_N_FIB: u128 = 186;
    let res_fib = fib(MAX_N_FIB);
    println!("Fibonacci: {res_fib} for max n = {MAX_N_FIB}");

    const MAX_N_COL: u32 = 1_000_000;
    const MIN_N_COL: u32 = 2;

    let mut col_len: Vec<u128> = Vec::new();

    for i in MIN_N_COL as u128 ..MAX_N_COL as u128 {
        col_len.push(col(i).len() as u128);
    }
    println!("Collatz: {:?}", col_len.iter().max())
}

#[cached]
fn fib(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[cached]
fn col(n: u128) -> Vec<u128> {
    if n == 1 {
        return vec![1];
    }
    let mut res: Vec<u128> = vec![n];

    let tail: Vec<u128> = if n % 2 == 0 {
        col(n / 2)
    } else {
        col(3 * n + 1)
    };

    res.extend(tail);
    return res;
}
