use std::io::stdin;

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

fn main() {
    let mut str = String::new();

    println!("{}", "Enter 1 || 0: ");
    stdin().read_line(&mut str).expect("No Entered Input");

    let num_str: i32 = str.trim().parse().unwrap();


    if num_str == 0 {
        let off = Type::Off();
        check_light_bulb(String::from("False"), false);
        println!("{:?}", off);
    } else if num_str == 1 {
        let on = Type::On();
        check_light_bulb(String::from("True"), true);
        println!("{:?}", on);
    }

}
