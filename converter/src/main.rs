use std::io;

fn main() {
    let mut temp: String = String::new();
    let stdin = io::stdin();
    println!("Welcome to Fº/Cº/K converter!");
    println!("Please enter the temperature you want to convert:");

    stdin.read_line(&mut temp).expect("Cannot read line :(");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut scale: String = String::new();
    println!("What scale is this temperature in? Enter 'F' for Fahrenheit, 'C' for Celsius or 'K' for Kelvin:");

    stdin.read_line(&mut scale).expect("Cannot read line :(");
    let scale: String = scale.trim().to_uppercase();

    match scale.as_str() {
        "F" => {
            println!("Converting to Celsius and Kelvin:");
            let celsius = converter_cel(temp);
            let kelvin = converter_cel_to_kelvin(celsius);
            print_temp_message(celsius, "Celsius");
            print_temp_message(kelvin, "Kelvin");
            return;
        }
        "C" => {
            println!("Converting to Fahrenheit and Kelvin:");
            let fahr = converter_fahr(temp);
            let kelvin = converter_cel_to_kelvin(temp);
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(kelvin, "Kelvin");
            return;
        }
        "K" => {
            println!("Converting to Celsius and Fahrenheit:");
            let celsius = converter_kelvin_to_cel(temp);
            let fahr = converter_fahr(celsius);
            print_temp_message(celsius, "Celsius");
            print_temp_message(fahr, "Fahrenheit");
            return;
        }
        _ => {
            println!("Invalid scale entered.");
            return;
        }
    };
}
fn converter_cel(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}
fn converter_fahr(x: f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}
fn converter_kelvin_to_cel(x: f64) -> f64 {
    x - 273.15
}

fn converter_cel_to_kelvin(x: f64) -> f64 {
    x + 273.15
}
fn print_temp_message(temp: f64, scale: &str) {
    let message = if temp < 0.0 {
        "That's really cold!"
    } else if temp > 30.0 {
        "That's really hot!"
    } else {
        "That's a moderate temperature."
    };

    println!(
        "The converted temperature is {} degrees {}. {}",
        temp, scale, message
    );
}
