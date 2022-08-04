use std::io;
fn main() {
    println!("Welcome to Convertor App, \n at first input which type of convert do you need");
    println!("\n 1 for convert from Fahrenheit to Celsius\n 2 for convert from Celsius to Fahrenheit\n 3 for exit");
    println!("");
    loop {
        println!("Enter convertor type:");
        let mut convertor_type_io = String::new();
        io::stdin()
            .read_line(&mut convertor_type_io)
            .expect("Failed to read line");
        let convertor_type:u8 = convertor_type_io.trim().parse().unwrap();

        if convertor_type == 1 {
            println!("Enter Fahrenheit: ");
            let mut fahrenheit_io = String::new();
            io::stdin()
            .read_line(&mut fahrenheit_io)
            .expect("Failed to read line");
            let fahrenheit:f32 = fahrenheit_io.trim().parse().unwrap();
            let celsius = convert_fahrenheit_to_celsius(&fahrenheit);
            println!("You entered {fahrenheit} and result is {celsius}");



        } else if convertor_type == 2 {
            println!("Enter Celsius: ");
            let mut celsius_io = String::new();
            io::stdin()
            .read_line(&mut celsius_io)
            .expect("Failed to read line");
            let celsius:f32 = celsius_io.trim().parse().unwrap();
            let fahrenheit = convert_celsius_to_fahrenheit(& celsius);
            println!("You entered {celsius} and result is {fahrenheit}");

        }else if convertor_type == 3{
            println!("You type 3 and now exit of program");
            break;

        }else {
            println!("You Enter wrong number, check it again");
            
        }

    }
    
}

fn convert_fahrenheit_to_celsius(fahrenheit: &f32) ->f32{
    (fahrenheit - 32.0) * 0.5556
}
fn convert_celsius_to_fahrenheit(celsius: &f32) ->f32{
    (celsius * 1.8) + 32.0
}