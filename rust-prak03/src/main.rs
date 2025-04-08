use cached::proc_macro::cached;
use std::convert::TryFrom;

const SIEVE_START: u32 = 2;
const SIEVE_END: u32 = 100_000;
const ACKERMANN_M: u8 = 3;
const ACKERMANN_N: u8 = 11;

fn main() {
    let primes: Vec<u32> = eratosthenes(SIEVE_START, SIEVE_END);
    println!("Primes until {SIEVE_END}: {:?}", primes);
    
    let ackermann_result = ackermann(ACKERMANN_M.into(), ACKERMANN_N.into());
    println!("Ackermann for M = {ACKERMANN_M} and N = {ACKERMANN_N} is {ackermann_result}");

}

fn eratosthenes(sieve_start: u32, sieve_end: u32) -> Vec<u32> {
    let mut primes_index: Vec<bool> = vec![true; usize::try_from(sieve_end).unwrap()];

    for i in sieve_start..sieve_end {
        if primes_index[usize::try_from(i).unwrap()] == true {
            for j in i + 1..sieve_end {
                if primes_index[usize::try_from(j).unwrap()] == true {
                    primes_index[usize::try_from(j).unwrap()] =
                        if j % i == 0 { false } else { true }
                }
            }
        }
    }

    let primes: Vec<u32> = primes_index
        .iter()
        .enumerate()
        .filter(|&prime_index| *prime_index.1 == true)
        .map(|(idx, _)| u32::try_from(idx).unwrap())
        .collect();
    return primes;
}

#[cached]
fn ackermann(m: u32, n: u32) -> u32 {
    if m == 0 {
        return n + 1;
    } else if n == 0 {
        return ackermann(m - 1, 1);
    } else {
        return ackermann(m - 1, ackermann(m, n - 1));
    }
}
