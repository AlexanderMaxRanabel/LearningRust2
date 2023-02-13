use std::io;
use std::io::Read;
use std::str::Chars;

fn square_area() {
    let mut square_side_input = String::new();
    println!("Please enter the length of one of the sides of square(Centimeter): ");
    io::stdin().read_line(&mut square_side_input)
        .expect("Failed to read input");

    let square_side_input = square_side_input.trim();

    let square_side_length = match square_side_input.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
    };

    println!("{}", square_side_length * square_side_length);
}

fn square_perimeter() {
    let mut square_side_input = String::new();
    println!("Please enter the length of one of the sides of square(Centimeter): ");
    io::stdin().read_line(&mut square_side_input)
        .expect("Failed to read input");

    let square_side_input = square_side_input.trim();

    let square_side_length = match square_side_input.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
    };

    println!("{}", square_side_length * square_side_length);
}

fn main() {


    let mut input = String::new();
    println!("Enter an integer: ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    let number1 = match input.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
    };

    let mut input2= String::new();
    println!("Enter a second integer: ");
    io::stdin().read_line(&mut input2)
        .expect("Failed to read input");

    let input2 = input2.trim();

    let number2 = match input2.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
    };

    let mut operation= String::new();
    println!("+, -, *, / for first 4 operations : ");
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    match operation {
        "+" => {
            println!("{}", number1 + number2);
        },
        "-" => {
          println!("{}", number1 - number2);
        },
        "*" => {
          println!("{}", number1 * number2);
        },
        "/" => {
          if number1 == 0 || number2 == 0 {
              println!("Error: Any Of the numbers cannot be zero");
          } else {
              println!("{}", number1 / number2);
          }
        },
        "AD" => {
            println!("Write 'Square' to enter square mode. Write 'Rectangle' to enter Rectangle mode. Write 'Triangle' to enter triangle mode. Write 'Circle to enter circle mode");
            let mut geometry_operation = String::new();
            io::stdin().read_line(&mut geometry_operation).expect("Failed to read input");
            let geometry_operation = geometry_operation.trim();
            match geometry_operation {
                "Square" => {
                    println!("Write 'Area' to calculate the area of a square. 'Perimeter' to calculate the perimeter of a square: ");
                    let mut square_mode = String::new();
                    io::stdin().read_line(&mut square_mode).expect("Failed to read input");
                    let square_mode = square_mode.trim();
                    match square_mode {
                        "Area" => {
                            square_area();
                        },
                        "Perimeter" => {
                            square_perimeter();
                        },
                        _ => {
                            println!("Unknown Mode");
                        },
                    }
                },
                _ => {
                    println!("Unknown Mode");
                },
            }
        },
        _ => {
          println!("Unknown ");
        },
    }

    let mut control= String::new();
    io::stdin().read_line(&mut control).expect("Failed to read input");
}
