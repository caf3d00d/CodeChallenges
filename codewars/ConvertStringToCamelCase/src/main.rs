fn to_camel_case(text: &str) -> String {
    let mut ret = String::new();
    if text.is_empty() {
        return ret
    }
    let binding = text.replace("-", "_");
    let split: Vec<&str> = binding.split('_').collect();
    for (i, s) in split.iter().enumerate() {
        if i == 0 {
            ret.push_str(&s);
            continue;
        }
        let c = s.chars().next().unwrap().to_uppercase().to_string();
        ret.push_str(&c.to_string());
        ret.push_str(&s[1..s.len()]);
    }
    ret
}

fn main() {
    println!("{:?}", to_camel_case("the_stealth_warrior"));
    println!("{:?}", to_camel_case("The_Stealth-Warrior"));
    println!("{:?}", to_camel_case("A-B-C"));
    println!("{:?}", to_camel_case(""));
}
