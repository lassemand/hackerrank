
fn v_to_str(v: i32) -> &str {
    match v {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "teen",
        11 => "eleven",
        12 => "twelve",
        _ => ""
    }
}

fn time_in_words(h: i32, m: i32) {
    let hour = v_to_str(if m > 30 {
        h + 1
    } else {
        h
    });

    if m == 0 {
        println!("{} o' clock", hour)
    }
    let mut chars = m.to_string().split("");
    if m >= 10 {

    }
    let first = chars.nth(0).unwrap();
    let s = chars.nth(1).unwrap();

}