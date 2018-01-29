fn main() {
    use std::io::*;
    let mut reader = BufReader::new(stdin());
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();

    let split:Vec<u32> = s.trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let x = split[0];
    let y = split[1];
    println!("{} {}",x * y, x * 2 + y * 2);
}