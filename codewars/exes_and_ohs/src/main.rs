fn xo(string: &str) -> bool {
    let str = string.to_lowercase();
    if !str.contains('x') && !str.contains('o') {
        return true;
    }
    let x = str.matches('x').count();
    let o = str.matches('o').count();
    if x == o {
        return true;
    }
    return false;
}

fn main() {
    println!("{:?}", xo("xoxo"));
    println!("{:?}", xo("jojo"));
    println!("{:?}", xo("jj"));
}
