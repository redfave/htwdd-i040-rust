fn main() {
    fix_1();
    fix_2();
    fix_3();
    // fix_4();
    let mut a = 2;
    let mut b = 5;
    swap_i32(&mut a, &mut b);
    println!("a: {a} | b: {b}");

    let mut foo: String = String::from("foooo");
    let mut bar: String = String::from("baaar");
    swap_string(&mut foo, &mut bar);
    println!("foo: {foo} | bar: {bar}");

    let (foo_abs, foo_rel) = vowel_stats_ownership_param(foo); // foo moved and not useable anynmore
    println!("{foo_abs} with {foo_rel}%");

    let (bar_abs, bar_rel) = vowel_stats_borrow_param(&bar);
    println!("{bar_abs} with {bar_rel}%"); // bar just borrowed and still useable

    println!("bar: {bar}");

}

fn fix_1() {
    let sachsen = String::from("HTW Dresden");
    let tbq = &sachsen;
    println!("{sachsen}, {tbq}");
}

fn fix_2() {
    let sachsen = String::from("HTW Dresden");
    let tbq = sachsen.clone();
    println!("{sachsen}, {tbq}");
}

fn fix_3() {
    let sachsen = String::from("HTW Dresden");
    let tbq = sachsen.to_string();
    println!("{sachsen}, {tbq}");
}

// fn fix_4() {
//     // Find at least four variants to make it work
//     let sachsen = String::from("HTW Dresden");
//     let tbq = "HTW Dresden";
//     println!("{sachsen}, {tbq}");
// }

// fn broken() {
//     // Find at least four variants to make it work
//     let sachsen = String::from("HTW Dresden");
//     let tbq = sachsen;
//     println!("{sachsen}, {tbq}");
// }

fn swap_i32(a: &mut i32, b: &mut i32) -> () {
    // let tmp: i32 = *b;
    // *b = *a;
    // *a = tmp;
    (*a, *b) = (*b, *a); // simple and elegant one liner
}

fn swap_string(a: &mut String, b: &mut String) -> () {
    let tmp: String = b.to_string();
    *b = a.to_string();
    *a = tmp.to_string();
}

fn vowel_stats_ownership_param(sequence: String) -> (i32, f32) {
    let mut voewels: i32 = 0;

    for letter in sequence.to_lowercase().as_str().chars() {
        match letter {
            'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' | 'y' =>  { voewels += 1},
            _ => {}
        }
    }
    (voewels, 100 as f32 * voewels as f32 / sequence.len() as f32)
}

fn vowel_stats_borrow_param(sequence: &String) -> (i32, f32) {
    let mut voewels: i32 = 0;

    for letter in sequence.to_lowercase().as_str().chars() {
        match letter {
            'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' | 'y' =>  { voewels += 1},
            _ => {}
        }
    }
    (voewels, 100 as f32 * voewels as f32 / sequence.len() as f32)
}
