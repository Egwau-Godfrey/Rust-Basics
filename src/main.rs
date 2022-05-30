use std::io::{self, Write};

fn main() {
    let mut sum: f64 = 0.0;
    let mut count = 0.0;
    let mut max: f64 = 0.0;
    let mut min: f64 = 0.0;
    loop {
        let mut number = String::new();
        print!("Enter a number : ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut number).unwrap();

        number.pop();
        if number == "done" {
            break;
        } else {

            let number: f64 = number.parse().unwrap();
            sum += number;
            count += 1.0;
            if number > max {
                max = number;
            }
            if number < max {
                min = number;
            }
        }

    };
    println!("\nThe sum is {}", sum);
    println!("The count is {}", count);
    println!("The average is {}", sum/count);
    println!("The maximum is {}", max);
    println!("The minimum is {}", min);

}