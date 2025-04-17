fn main() {
    let mut seconds = 21;
    
    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;

        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }
        
        println!("{seconds} seconds to blastoff..");
        seconds -= 1;
    }
}