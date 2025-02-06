fn main() {
    let s = String::from("hello world");

    let s1 = first_word(&s);
    println!("s1: {s1}");

    let s3 = "hell";
    let s4 = &s3[1..2];
    println!("s4: {s4}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
