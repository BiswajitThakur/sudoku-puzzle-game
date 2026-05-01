use std::fmt::Display;

use sudoku_puzzle_game::{
    game::{create_game, hide_answer_mask, hide_answer_mask_mut},
    utils::GemeType,
};

fn main() {
    let mut game = create_game(9, 1000000).unwrap();
    let lvl = hide_answer_mask_mut(&mut game, GemeType::Difficult);
    print(game);
    println!("---------------------");
    print(bool_int(lvl));
}

fn print<T: Display>(arr: Vec<Vec<T>>) {
    for i in arr {
        for j in i {
            print!("{}, ", j);
        }
        println!("")
    }
}

fn bool_int(v: Vec<Vec<bool>>) -> Vec<Vec<u8>> {
    v.into_iter()
        .map(|u| u.into_iter().map(|m| if m { 1 } else { 0 }).collect())
        .collect()
}
