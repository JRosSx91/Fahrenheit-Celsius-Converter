use std::io;

fn main() {
    let mut temp: String = String::new();
    let stdin = io::stdin();
    println!("Welcome to Fº/Cº/K converter!");
    println!("Please enter the temperature you want to convert:");

    stdin.read_line(&mut temp).expect("Cannot read line :(");

    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut scale: String = String::new();
    println!("What scale is this temperature in? Enter 'F' for Fahrenheit, 'C' for Celsius or 'K' for Kelvin:");

    stdin.read_line(&mut scale).expect("Cannot read line :(");
    let scale: String = scale.trim().to_uppercase();

    let (result, result_scale) = match scale.as_str() {
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

    print_temp_message(result, result_scale);
}
fn converter_cel(x: i32) -> i32 {
    (x - 32) * 5 / 9
}
fn converter_fahr(x: i32) -> i32 {
    x * 9 / 5 + 32
}
fn converter_kelvin_to_cel(x: i32) -> i32 {
    x - 273
}

fn converter_cel_to_kelvin(x: i32) -> i32 {
    x + 273
}
fn print_temp_message(temp: i32, scale: &str) {
    let message = if temp < 0 {
        "That's really cold!"
    } else if temp > 30 {
        "That's really hot!"
    } else {
        "That's a moderate temperature."
    };

    println!(
        "The converted temperature is {} degrees {}. {}",
        temp, scale, message
    );
}
