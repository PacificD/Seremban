use std::io;

fn convert_temperature(input_type: u8, temperature: f32) -> f32 {
    if input_type == 1 {
        // fahrenheit to celsius
        (temperature - 32.0) * 5.0 / 9.0
    } else {
        // celsius tp fahrenheit
        temperature * 9.0 / 5.0 + 32.0
    }
}

pub fn convert() {
    loop {
        let (mut input_type, mut temperature) = (String::new(), String::new());

        println!("Please enter 1 for fahrenheit to celsius or 2 for celsius to fahrenheit");

        io::stdin()
            .read_line(&mut input_type)
            .expect("Failed to read line");

        let input_type: u8 = match input_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                break;
            }
        };

        println!("Please enter the temperature");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                break;
            }
        };
        let converted_temperature = convert_temperature(input_type, temperature);

        let (raw_sign, res_sign) = if input_type == 1 {
            ("°F", "°C")
        } else {
            ("°C", "°F")
        };

        break println!("{temperature}{raw_sign} is {converted_temperature}{res_sign}");
    }
}
