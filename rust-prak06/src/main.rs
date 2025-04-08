use std::fmt::Display;

fn main() {
    let mut a: i32 = 4;
    let mut b: i32 = 55;
    println!("Init a = {a}, b = {b}");
    gen_swap(&mut a, &mut b);
    println!("Swapped a = {a}, b = {b}");

    let mut foo: &str = "foo";
    let mut bar: &str = "bar";
    println!("Init foo = {foo}, bar = {bar}");
    gen_swap(&mut foo, &mut bar);
    println!("Swapped foo = {foo}, bar = {bar}");

    gen_out("foo", foo, "a", a);

    let mut arr_int = [4, 7, 2, 10, 0, 1, 5];
    println!("Array int: {:?}", arr_int);
    gen_swap_neighbors(&mut arr_int, 0);
    println!("Array int swapped: {:?}", arr_int);

    let mut arr_str = ["Hund", "Giraffe", "Affe", "Zebra", "Schackal", "Tiger", "Bär", "Schlange"];
    println!("Array str: {:?}", arr_str);
    bubble_sort(&mut arr_str);
    println!("Array str sorted: {:?}", arr_str);
}

fn gen_swap<T: Copy>(a: &mut T, b: &mut T) -> () {
    let tmp: T = *a;
    *a = *b;
    *b = tmp;
}

fn gen_out<T: Display, S: Display>(name_a: &str, a: T, name_b: &str, b: S) -> () {
    println!("'{name_a}' = {a}, '{name_b}' = {b}")
}

fn gen_swap_neighbors<T: Clone>(arr: &mut [T], pos: usize) -> () {
    if arr.len() < 2 || pos > arr.len() - 2 {
        return;
    }

    let tmp = arr[pos + 1].clone();
    arr[pos + 1] = arr[pos].clone();
    arr[pos] = tmp;
}

fn bubble_sort<T>(arr: &mut [T]) -> ()
where
    T: Clone + PartialOrd,
{
    for k in (0..=arr.len() - 1).rev() {
        for i in 0..k {
            if arr[i] > arr[i + 1] {
                gen_swap_neighbors(arr, i);
            }
        }
    }
}
