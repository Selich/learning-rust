use std::io;
use std::cmp::Ordering;
use rand::Rng;



fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1, 10);




    loop {
        println!("Input the number");
        let stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Your guessed number is {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Lesser"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => println!("Just right!")
        }
    }

}
