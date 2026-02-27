fn number_to_str(v: i32) -> String {
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
        13 => "thirteen",
        14 => "fourteen",
        15 => "quarter",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "half",
        _ => {
            let s = v.to_string();
            let len = s.len();
            return s.chars()
                .enumerate()
                .map(|(i, c)| {
                    let digit = c.to_digit(10).unwrap() as i32;
                    if i == 0 {
                        number_to_str(digit * 10_i32.pow((len - 1) as u32))
                    } else {
                        number_to_str(digit)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    }.to_string()
}

fn time_in_words(h: i32, m: i32) -> String {

    let hour = number_to_str(if m > 30 {
        h + 1
    } else {
        h
    });

    if m == 0 {
        return format!("{} o' clock", hour);
    }
    let minute = number_to_str(if m > 30 {
        60 - m
    } else {
        m
    });
    let middle = if m > 30 {
        "to"
    } else {
        "past"
    };
    if vec![15, 30, 45].contains(&m) {
        format!("{} {} {}", minute, middle, hour)
    } else {
        let unit = if m == 1 {
            "minute"
        } else {
            "minutes"
        };
        format!("{} {} {} {}", minute, unit, middle, hour)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_hour() {
        assert_eq!(time_in_words(5, 0), "five o' clock");
    }

    #[test]
    fn minutes_past() {
        assert_eq!(time_in_words(5, 10), "teen minutes past five");
    }

    #[test]
    fn single_minute() {
        assert_eq!(time_in_words(5, 1), "one minute past five");
    }

    #[test]
    fn quarter_past() {
        assert_eq!(time_in_words(5, 15), "quarter past five");
    }

    #[test]
    fn half_past() {
        assert_eq!(time_in_words(5, 30), "half past five");
    }

    #[test]
    fn quarter_to() {
        assert_eq!(time_in_words(5, 45), "quarter to six");
    }

    #[test]
    fn minutes_to() {
        assert_eq!(time_in_words(5, 40), "twenty minutes to six");
    }
}