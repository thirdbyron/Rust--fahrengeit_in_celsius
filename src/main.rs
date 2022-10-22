use std::io;

fn main() {
    println!("Type the number of fahrengeits:");

    let mut fahrengeit = String::new();

    io::stdin().read_line(&mut fahrengeit).expect("You failed!");

    let fahrengeit: f32 = fahrengeit.trim().parse().expect("Wrong fahrengate value");

    let celsius = fahrengeit_in_celsius(fahrengeit);

    println!("{} fahrengeits is equal to {} Celsius", fahrengeit, celsius);
}

fn fahrengeit_in_celsius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}
