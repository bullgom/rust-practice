fn get_str_length(s:String) -> usize {
    s.len()
}

fn main() {
    let a = String::from("Hello");
    let str_len = get_str_length(a);

    println!("String Length {}", str_len);
}
