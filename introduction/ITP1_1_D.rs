fn main() {
    use std::io::*;
    let mut stdin = BufReader::new(stdin());
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();

    let second = s.trim().parse::<i32>().unwrap();
    let (hour, second2) = divmod(second, 3600);
    let (minute, second3) = divmod(second2, 60);

    println!("{}:{}:{}", hour, minute, second3);
}

fn divmod(mut x: i32, y: i32) -> (i32, i32) {
    let mut divide:i32 = 0;
    while x >= y {
        x -= y;
        divide += 1;
    }
    return (divide, x);
}