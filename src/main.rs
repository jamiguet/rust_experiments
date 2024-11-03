#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod fibonacci;
mod hr_management;
mod piggify;
mod vector_stats;
mod error_handling;
mod guessing_game;
mod gen_safe_input;
mod odd_syntax;

use fibonacci::main as fibo_main;
use hr_management::main as hr_management_main;
use piggify::main as piggify_main;
use vector_stats::main as vector_stats_main;
use error_handling::main as error_handling_main;
use guessing_game::main as guessing_game_main;
use odd_syntax::main as odd_syntax_main;

fn main() {
    println!("Now for Fibonacci calculation!");
    // fibo_main();
    // println!("Now for Vector Stats");
    // vector_stats_main();
    // println!("Now for Piggify");
    // piggify_main();
    // println!("Now for Hr management");
    // hr_management_main();
    // println!("Now for Error Handling");
    // error_handling_main();
    // println!("Now for Guessing Game");
    // let _ = guessing_game_main();
    odd_syntax_main();
}
