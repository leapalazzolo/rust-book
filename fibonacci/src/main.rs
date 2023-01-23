use std::io;

fn main() {
    println!("Get the nth fibonacci number");
    let number = get_number_from_input();
    let mut start: u32 = 0;
    let mut result: u32 = 1;
    for _ in 1..number {
        let tmp = start;
        start = result;
        result += tmp;
    }
    println!("The result is {result}");
}

fn get_number_from_input() -> u32 {
    loop {
        let mut number = String::new();
        println!("Insert a positive number");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: u32 = match number.trim().parse() {
            Ok(num) => {num},
            Err(_) => continue,
        };

        if number == 0 {
            continue
        }
        
        return number;
    }
}
