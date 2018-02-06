fn main() {
	let stdin = std::io::stdin();
	let mut s = String::new();
	stdin.read_line(&mut s).unwrap();
	
	let input = s.trim().split(" ").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	let a = input[0];
	let b = input[1];
	
	if a < b {
		println!("a < b");
	}else if a > b {
		println!("a > b");
	}else{
		println!("a == b");
	}
}
