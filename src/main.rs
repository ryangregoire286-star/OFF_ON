use std::io::stdin;
use std::sync::atomic::{AtomicBool, Ordering};

pub static mut IS_WRITEABLE: bool = true;

fn check_light_bulb(light_level: String, is_working: bool) -> () {

    if is_working  {
        println!("{}", String::from(light_level))
    }

    else {
        println!("{}", String::from(light_level))
    }


}

#[derive(Debug)]
enum Type {

    Off(),
    On()
}

fn add(x: i32) -> i32 {
    x + 1
}

fn sub(x: i32) -> i32 {
    x - 1
}

fn main() {

    unsafe {
        while IS_WRITEABLE == true {
            let mut str = String::new();

            println!("{}", "Enter 1 || 0: ");
            stdin().read_line(&mut str).expect("No Entered Input");

            let num_str: i32 = str.trim().parse().unwrap();

            static X: AtomicBool = AtomicBool::new(false);
            static Y: AtomicBool = AtomicBool::new(true);

            if num_str == sub(1) {
                let off = Type::Off();

                check_light_bulb(String::from("False"), X.load(Ordering::Relaxed));
                println!("{:?}", off);
            } else if num_str == add(0){
                let on = Type::On();
                check_light_bulb(String::from("True"), Y.load(Ordering::Relaxed));
                println!("{:?}", on);
            }

            IS_WRITEABLE = false;
        }
    }

}