use std::iter::Filter;

fn main() {
    let n = 5;
    let larger_than_predicate = |&&v: &&i32| -> bool { v > n };
    let lst: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res: Vec<&i32> = lst.iter().filter(larger_than_predicate).collect();
    println!("{:?}", res);

    let seasons = vec!["spring", "summer", "autumn", "winter"];
    let seasons_iter = seasons.clone().into_iter();
    for my_season in seasons_iter {
        println!("I love the {my_season}!");
    }

    for my_season in seasons.clone().into_iter() {
        println!("I love the {my_season}!");
    }

    let mut seasons_iter = seasons.clone().into_iter();
    while let Some(my_season) = seasons_iter.next() {
        println!("I love the {my_season}!");
    }

    let mut seasons_iter = seasons
        .clone()
        .into_iter()
        .map(|season| season.to_uppercase());
    while let Some(my_season) = seasons_iter.next() {
        println!("I love the {my_season}!");
    }

    let mut seasons_iter = seasons
        .clone()
        .into_iter()
        .map(|season| season
            .replace("r", "")
            .replace("s",""));

    while let Some(my_season) = seasons_iter.next() {
        println!("I love the {my_season}!");
    }
}
