use dialoguer::{Input, Select};

fn main() {
    println!("--------------simple rust unit converter---------------");
    loop {
        //Converter catagories
        let catagories = [
            "Length (Meters <--> Kilometers <--> Miles)",
            "Temparature (Celcius <--> Farenheit",
            "Exit"
        ];
        //Prompt user for converter choice
        let catagory = Select::new()
            .with_prompt("Select a conversion or exit")
            .items(&catagories)
            .default(0)
            .interact()
            .unwrap();

        match catagory {
            0 => {length_converter()}
            1 => {temparature_converter()}
            2 => {break}
            _ => println!("Invalid choice, please try again")
        }
    }
}


fn length_converter() {
    //prompt user to input a value to convert
    let value : f64 = Input::new()
        .with_prompt("Enter a length to convert")
        .interact_text()
        .unwrap();
    
    let units = [
        "Meters", 
        "Kilometers", 
        "Miles",
    ];
    let unit = Select::new()
        .with_prompt("What unit is the value?")
        .items(&units)
        .default(0)
        .interact()
        .unwrap();

    let meters = match unit {
        0 => value,
        1 => value * 1000.0,
        2 => value * 1609.34,
        _ => {println!("Invalid input..."); return;}
    };

    println!("Converted values:");
    println!("{:.2} m", meters);
    println!("{:.2} km", meters / 1000.0);
    println!("{:.2} mi", meters / 1609.34);
}

fn temparature_converter() {
    let value : f64 = Input::new()
        .with_prompt("What is the temparature?")
        .interact_text()
        .unwrap();

    let scales = [
        "Celcius",
        "Farenheit",
    ];
    let scale = Select::new()
        .with_prompt("What scale is the temparature?")
        .items(&scales)
        .default(0)
        .interact()
        .unwrap();

    match scale {
        0 => {
            let f = (value * 9.0 / 5.0) + 32.0;
            println!("{:.2} C is {:.2} F", value, f);
        },
        _ => {
            let c = (value - 32.0) * 5.0 / 9.0;
            println!("{:.2} C is {:.2} F", c, value);
        },
    };
}
