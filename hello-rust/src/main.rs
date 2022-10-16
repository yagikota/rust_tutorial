fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else {
        "Buzz".to_string()
    }
}

fn main() {
    let res = (0..100)
        .map(fizz_buzz)
        .fold(String::from(""), |acc, line| format!("{}\n{}", acc, line));
    println!("{}", res);
}
