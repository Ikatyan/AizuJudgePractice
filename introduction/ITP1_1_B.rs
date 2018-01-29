fn main() {
    use std::io::*;
    let mut stdin = BufReader::new(stdin());
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();

    let x = s.trim().parse::<u32>().unwrap();
    println!("{}", x * x * x);
}
