use rand::prelude::*;
use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};

mod dice{
    use rand::Rng;

    pub fn roll_for_dice(dice_type: &str) -> Option<u32> {
        let sides: u32 = match dice_type {
            "d4" => 4,
            "d6" => 6,
            "d8" => 8,
            "d10" => 10,
            "d12" => 12,
            "d20" => 20,
            "d100" => 100,
            _ => return None
        };
        let roll : u32 = roll_for_dice(sides);
        Some(roll)
    }
    fn roll_dice(sides: u32){
        let mut rng = rand::thread_rng();
        rng.gen_range(1..sides);
    }
}

fn logo() {
    println!(
        "
  ____
 /' .\\    _____
/: \\___\\  / .  /\\
\\' / . / /____/..\\
 \\/___/  '  ' /  /
          ' __'\\/
        "
    );
}

fn start() {
    logo();
    println!("Welcome to Roll FOr Chaos!");
    println!("I understand Die Notation!");
    println!("Tell me what kind of Die you want to roll. ex. 2d20");
    println!("Then I will ask how many times you would like to roll.");
    println!("Give it a shot!");
}

fn main() {
    println!("Hello, world!");
}
