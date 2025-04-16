fn main() {
    let evaluation = true;

    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("the value is false");
        }
    }

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("{value}");
}