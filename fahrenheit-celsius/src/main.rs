use std::io;

fn main() {
    println!("Fahrenheit -> Celsius.");
    let number = get_number_from_input();

    let celsius = (number - 32.0) * 5.0 / 9.0;
    println!("You have {celsius}°C!");
    
}

fn get_number_from_input() -> f64 {
    loop {
        let mut number = String::new();
        println!("Insert a number");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return number;
    }
}