use std::io;

fn main() {
    let mut temp: String = String::new();
    let stdin = io::stdin();
    println!("Welcome to Fº/Cº converter!");
    println!("Please enter the temperature you want to convert:");

    stdin.read_line(&mut temp).expect("Cannot read line :(");

    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut scale: String = String::new();
    println!("What scale is this temperature in? Enter 'F' for Fahrenheit or 'C' for Celsius:");

    stdin.read_line(&mut scale).expect("Cannot read line :(");
    let scale: String = scale.trim().to_uppercase();

    let result: i32 = match scale.as_str() {
        "F" => converter_cel(temp),
        "C" => converter_fahr(temp),
        _ => {
            println!("Invalid scale entered.");
            return;
        }
    };

    println!("The converted temperature is {} degrees.", result);
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
