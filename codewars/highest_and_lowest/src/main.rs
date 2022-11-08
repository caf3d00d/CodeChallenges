fn high_and_low(numbers: &str) -> String {
    let mut big: i32;
    let mut small: i32;
    let mut str: String = String::new();
    let split: Vec<&str>= numbers.split(' ').collect();
    small = split[0].parse::<i32>().unwrap();
    big = split[0].parse::<i32>().unwrap();
    for i in split {
        if i.parse::<i32>().unwrap() > big {
            big = i.parse::<i32>().unwrap();
        }
        if i.parse::<i32>().unwrap() < small {
            small = i.parse::<i32>().unwrap()
        }
    }

    str.push_str(&big.to_string());
    str.push_str(" ");
    str.push_str(&small.to_string());
    str
}

fn main() {
    let res = high_and_low("1 2 7 -1 18");
    println!("{}", res);
}
