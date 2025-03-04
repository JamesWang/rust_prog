fn print_stirng(s: String) {
    println!("print_String:{}", s);
}

fn print_str(s: &str) {
    println!("print_str:{}", s);
}

pub fn run_string_str() {
    print_stirng(String::from("String"));
    print_str(&String::from("String"));
    print_str("str");
    //print_stirng("str");
}

