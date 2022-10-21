#![allow(unused)]

mod constant;
mod game;
use game::Game;

fn main() {
    println!("Hello, world!");
    let mut g = Game::new();
    g.test();
}
