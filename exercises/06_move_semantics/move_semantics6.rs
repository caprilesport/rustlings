// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    let new_data = string_uppercase(data);
    println!("new data is: {}", new_data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    let c = data.chars().last().unwrap();
    println!("value of char is {}", c);
    c
}

// Should take ownership
fn string_uppercase(mut data: String) -> String {
    data = data.to_uppercase();
    println!("{}", data);
    data
}
