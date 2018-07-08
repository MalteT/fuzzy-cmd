extern crate fuzzy_cmd;

use fuzzy_cmd::FuzzyCmd;
use std::env::args;
use std::iter::FromIterator;

fn main() {
    let mut fuzz = FuzzyCmd::new().enable_fuzzy();
    {
        let mut help = fuzz.add("help");
        help.add("all").call(|| println!("Easy help should be part of this crate.."));
        help.call(|| println!("Usage: help all"));
    }
    {
        let mut n_bake = fuzz.add("bake");
        n_bake.add("cake").call(|| bake("cake"));
        n_bake.add("oven").call(|| bake("oven"));
        n_bake.add("pizza").call(|| bake("pizza"));
        n_bake.call(|| println!("Usage: bake (cake|oven|pizza)"));
    }

    let mut args = args();
    args.next();
    let cmd = String::from_iter(args.map(|mut s| {
        s += " ";
        s
    }));
    fuzz.exec(&cmd).unwrap();
}

fn bake(recipe: &str) {
    println!(
        "Baking {}",
        match recipe {
            "cake" => "a cake. But finish your tests first!",
            "oven" => "myself, obviously... who whould do that to an oven?",
            "pizza" => "pizza..",
            _ => panic!("Bug!"),
        }
    );
}
