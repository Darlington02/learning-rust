fn main() {
    let mut number = 0;
    let mut fib1 = 0;
    let mut fib2 = 1;

    while number < 10 {
        if number == 0{
            println!("The value is {fib1}");
        }
        if number == 1 {
            println!("The value is {fib2}");
        }
        
        let fib3 = calculate_fib(fib1, fib2);
        println!("The value is {fib3}");      

        fib1 = fib2;
        fib2 = fib3;
        number += 1;
    }
}

fn calculate_fib(x: i32, y: i32) -> i32 {
    x + y
}