use std::io;
fn main() {
    println!("--------------simple rust unit converter---------------");
    loop {
        println!("\nSelect Catagory...");
        println!("1. Length (Meters <--> Kilometers <--> Miles)");
        println!("2. Temparature (Celcius <--> Farenheit");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => {length_converter()}
            2 => {temparature_converter()}
            3 => {break}
            _ => println!("Invalid choice, please try again")
        }
    }
}


fn length_converter() {
    println!("Enter a value to convert...");
    let value : f64 = read_f64();
    println!("Select input unit: 1. Meters 2. Kilometers 3. Miles");
    let input_unit = read_u32();

    let meters = match input_unit {
        1 => value,
        2 => value * 1000.0,
        3 => value * 1609.34,
        _ => {println!("Invalid input..."); return;}
    };

    println!("Converted values:");
    println!("{:.2} m", meters);
    println!("{:.2} km", meters / 1000.0);
    println!("{:.2} mi", meters / 1609.34);
}

fn temparature_converter() {
    println!("Enter temparature value...");
    let value : f64 = read_f64();
    println!("Select input scale: 1. Celcius 2. Farenheit ");
    let scale = read_u32();

    match scale {
        1 => {
            let f = (value * 9.0 / 5.0) + 32.0;
            println!("{:.2} C is {:.2} F", value, f);
        },
        2 => {
            let c = (value - 32.0) * 5.0 / 9.0;
            println!("{:.2} C is {:.2} F", c, value);
        },
        _ => {println!("Invalid scale...");}
    };
}


//Helper functions for inputs
fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    input.trim().parse().unwrap_or(0.0)
}

fn read_u32() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    input.trim().parse().unwrap_or(0)
}