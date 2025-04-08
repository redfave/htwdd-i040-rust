fn main() {
    fix_1();
    fix_2();
    euler();
}

fn fix_1() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
    assert_eq!(v, 3);
    println!("Success!");
}

fn fix_2() {
    let v = {
        let mut x = 1;
        x += 2
    };
    assert_eq!(v, ());
    println!("Success!");
}

fn euler() {
    let mut e: f64 = 2.0; // 0! + 1!

    'out:
    for k in 2..u32::MAX {
        let mut factorial: u128 = 1;
        for i in 1..=k {
            factorial = match factorial.checked_mul(i as u128) {
                Some(value) => value,
                None => break 'out,
            };
        }

        e += 1.0 / factorial as f64;

        println!("k = {k}\t--> {e}")
    }
    println!("e = {e}")
}
