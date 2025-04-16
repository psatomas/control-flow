fn main() {
    let some_condition_that_we_cannot_predict_in_advance = true;

    if some_condition_that_we_cannot_predict_in_advance {
        println!("This line will be output")
    }
    if false {
        println!("This line will NOT be output")
    }
}
