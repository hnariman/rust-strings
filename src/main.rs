fn create_strings() {
    let slice: &str = "This is some fixed one";
    let stored_in_heap: String = String::from("This is some dynamic string we have");
    let owned = "This is owned string".to_owned();
    let string_made = "this is converted to string".to_string();
    println!("{}", slice);
    println!("{}", stored_in_heap);
    println!("{}", owned);
    println!("{}", string_made);
}

fn manipulate_string() -> String {
    let mut message = "This is some message".to_string();
    println!("{}", message);
    message.push_str(" more text added");
    println!("{}", message);
    return message;
}

fn concat_strings() {
    let s1: String = String::from("First part ");
    let s2: String = String::from("Second part ");
    // we can't borrow things twice, so we have to implicitly clone values
    // such is not very welcomed for the sake of memory management
    let formatted: String = format!("format with clone: {} _ {} ", s1.clone(), s2.clone());
    println!("{}", formatted);
    // or we can copy things by their references
    let formatted_two: String = format!("formatted with copies: {} _ {} ", &s1, &s2);
    println!("{}", formatted_two);

    let concatination = ["first ", "n second"].concat();
    println!("{}", concatination);

    let concat_one = s1 + "AND " + &s2;
    println!("{}", concat_one);

    let slice_type_concatinated: &str = &("Some text ".to_owned() + "HOLLOW");
    println!("{}", slice_type_concatinated);

    let slice_type_better_practice_concat: &str = concat!("first slice ", "second slice");
    println!("{}", slice_type_better_practice_concat);
}

fn main() {
    create_strings();
    let msg = manipulate_string();
    println!("{:#?}", msg);
    concat_strings();
}
