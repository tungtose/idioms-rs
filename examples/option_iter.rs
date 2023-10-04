pub fn main() {
    let e_char = Some("e");

    let mut char_list = vec!["a", "b", "c", "d"];

    char_list.extend(e_char);

    println!("Char list: {:?}", char_list);

    for c in char_list.iter().chain(e_char.iter()) {
        println!("{} is a char", c);
    }
}
