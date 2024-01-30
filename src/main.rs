extern crate serde_derive;


#[macro_use]
mod blockchain;

use std::io;
use std::process;
use std::io::Write;


fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Please Input a miner's address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Please proveide an integer");
    println!("Generating a genesis block!!");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);

    loop {
        println!("Index");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");

        print!("Enter your choice number : ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!(" ");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!!");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Sending amount : ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let result = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match result {
                    true => println!("Transaction added!"),
                    false => println!("Transaction failed!"),
                }
            },
            2 => {
                println!("Generating new block");
                let result = chain.generate_new_block();
                match result {
                    true => println!("Block generated successfully!"),
                    false => println!("Block generation failed!"),
                }

            },
            3 => {
                let mut new_diff = String::new();
                println!("enter new difficulty");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let result = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match result {
                    true => println!("Updated Difficulty!"),
                    false => println!("Failed update difficulty!"),
                }

            },
            4 => {
                let mut new_reward = String::new();
                println!("Enter new reward!!");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let result = chain.update_reward(new_reward.trim().parse().unwrap());
                match result {
                    true => println!("Updated reward!"),
                    false => println!("Failed update reward!"),
                }
            },
            _ => println!("\tInvalid input please retry again\t"),
        }
    }

}
