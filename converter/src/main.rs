use std::io;

fn main() {
    let mut temp: String = String::new();
    let stdin = io::stdin();
    println!("Welcome to Fº/Cº/K/R converter!");
    println!("Please enter the temperature you want to convert:");

    stdin.read_line(&mut temp).expect("Cannot read line :(");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut scale: String = String::new();
    println!("What scale is this temperature in? Enter 'F' for Fahrenheit, 'C' for Celsius, 'K' for Kelvin and 'R' for Rankine:");

    stdin.read_line(&mut scale).expect("Cannot read line :(");
    let scale: String = scale.trim().to_uppercase();

    match scale.as_str() {
        "F" => {
            println!("Converting to Celsius, Rankine and Kelvin:");
            let celsius = converter_cel(temp);
            let rankine = converter_fahr_to_rankine(temp);
            let kelvin = converter_cel_to_kelvin(celsius);
            print_temp_message(celsius, "Celsius");
            print_temp_message(rankine, "Rankine");
            print_temp_message(kelvin, "Kelvin");
            return;
        }
        "C" => {
            println!("Converting to Fahrenheit, Kelvin and Rankine:");
            let fahr = converter_fahr(temp);
            let rankine = converter_fahr_to_rankine(fahr);
            let kelvin = converter_cel_to_kelvin(temp);
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(kelvin, "Kelvin");
            print_temp_message(rankine, "Rankine");
            return;
        }
        "R" => {
            println!("Converting to Fahrenheit, Celsius and Kelvin:");
            let fahr = converter_rankine_to_fahr(temp);
            let celsius = converter_cel(fahr);
            let kelvin = converter_cel_to_kelvin(celsius);
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(celsius, "Celsius");
            print_temp_message(kelvin, "Kelvin");
        }
        "K" => {
            println!("Converting to Celsius, Fahrenheit and Rankine:");
            let celsius = converter_kelvin_to_cel(temp);
            let fahr = converter_fahr(celsius);
            let rankine = converter_fahr_to_rankine(fahr);
            print_temp_message(celsius, "Celsius");
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(rankine, "Rankine");
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
fn converter_fahr_to_rankine(x: f64) -> f64 {
    x + 459.67
}

fn converter_rankine_to_fahr(x: f64) -> f64 {
    x - 459.67
}
fn print_temp_message(temp: f64, scale: &str) {
    let message = match scale {
        "Celsius" => {
            if temp < 0.0 {
                "That's really cold!"
            } else if temp >= 30.0 {
                "That's really hot!"
            } else {
                "That's a moderate temperature."
            }
        }
        "Fahrenheit" => {
            if temp < 32.0 {
                "That's really cold!"
            } else if temp >= 86.0 {
                "That's really hot!"
            } else {
                "That's a moderate temperature."
            }
        }
        "Kelvin" => {
            if temp < 273.15 {
                "That's really cold!"
            } else if temp >= 303.15 {
                "That's really hot!"
            } else {
                "That's a moderate temperature."
            }
        }
        "Rankine" => {
            if temp < 491.67 {
                "That's really cold!"
            } else if temp >= 545.67 {
                "That's really hot!"
            } else {
                "That's a moderate temperature."
            }
        }
        _ => "Invalid scale.",
    };

    println!(
        "The converted temperature is {:.2} degrees {}. {}",
        temp, scale, message
    );
}
