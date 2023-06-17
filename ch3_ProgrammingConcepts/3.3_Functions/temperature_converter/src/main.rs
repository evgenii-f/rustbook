use std::io;

fn read_convert_type()->String {
    let mut convert_type = String::new();
    loop {
        println!("Choose the units: C for F->C, F for C->F:");
        match io::stdin().read_line(&mut convert_type) {
            Ok(num) =>  {
                println!("{num}");
                // if user_input.eq_ignore_ascii_case("f") || user_input.eq_ignore_ascii_case("c") {
                //     return user_input.make_ascii_lowercase();
                // }
                continue;
            }
            Err(_) => continue,
        }
    }
}

fn main() {
    let convert_type = read_convert_type();
}
