use std::ops::Div;

fn main() {

print_reciprocal();
obfuscation_example();

}

fn print_reciprocal() -> () {
    for i in -3..=5 {
        let print_res: String;

        if let Some(res) = reciprocal(i) {
            print_res = res.to_string();
        } else {
            print_res = String::from("n. d.");
        }
        println!("1/{i} = {print_res}")
    }
}

fn reciprocal(n: i64) -> Option<f64> {
    match n {
        0 => None,
        _ => Some(1.0.div(n as f64)),
    }
}

fn obfuscation_example() -> () {
        for number in 1..=18 {
        print!(
            "{}",
            match number {
                1 => "E",
                n if i32::abs(6 - n % 10) == 1 => "E",
                n if n % 10 == 6 => "S",
                n if n % 10 == 8 => "L",
                14 => "W",
                n if n % 5 == 4 => " ",
                13 => " ",
                2 => "I",
                3 => "N",
                10 => "A",
                11 => "U",
                12 => "S",
                _ => "",
            }
        );
    }
    println!("");
}
