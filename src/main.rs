#![allow(unused)]

mod constant;
mod game;
mod error;
use game::Game;

fn main() {
    println!("Hello, world!");
    let mut g = Game::new();
    g.test();
}
