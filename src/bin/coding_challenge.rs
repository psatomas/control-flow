fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number("purple"));
}