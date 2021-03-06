//argument
const MAX_I: usize = 5;
const MAX_J: usize = 6;

pub struct Counter {
	pub is_first: bool,
	pub count: usize,
}
impl Counter {
	pub fn new(b: bool, c: usize) -> Counter {
		Counter {
			is_first: b,
			count: c,
		}
	}
}

fn flip_to1(i: usize, j: usize, map: &mut [[u8; 6]; 5], counter: &mut Counter) {
	if map[i][j] == 1 {
		map[i][j] = 0;
		if counter.is_first {
			counter.is_first = false;
			counter.count += 1;
		}
		// 再帰で隣接マスについてこれを呼び出す
		if i == 0 {
			if j == 0 {
				flip_to1(i + 1, 0, map, counter);
				flip_to1(i + 1, 1, map, counter);
				flip_to1(i, 1, map, counter);
			} else if j == MAX_J - 1 {
				flip_to1(i + 1, j, map, counter);
				flip_to1(i + 1, j - 1, map, counter);
				flip_to1(i, j - 1, map, counter);
			} else {
				flip_to1(i + 1, j - 1, map, counter);
				flip_to1(i + 1, j, map, counter);
				flip_to1(i + 1, j + 1, map, counter);
				flip_to1(i, j + 1, map, counter);
				flip_to1(i, j - 1, map, counter);
			}
		} else if i == MAX_I - 1 {
			if j == 0 {
				flip_to1(i - 1, 0, map, counter);
				flip_to1(i - 1, 1, map, counter);
				flip_to1(i, 1, map, counter);
			} else if j == MAX_J - 1 {
				flip_to1(i - 1, j, map, counter);
				flip_to1(i - 1, j - 1, map, counter);
				flip_to1(i, j - 1, map, counter);
			} else {
				flip_to1(i - 1, j - 1, map, counter);
				flip_to1(i - 1, j, map, counter);
				flip_to1(i - 1, j + 1, map, counter);
				flip_to1(i, j + 1, map, counter);
				flip_to1(i, j - 1, map, counter);
			}
		} else {
			if j == 0 {
				flip_to1(i - 1, j, map, counter);
				flip_to1(i - 1, j + 1, map, counter);
				flip_to1(i, j + 1, map, counter);
				flip_to1(i + 1, j, map, counter);
				flip_to1(i + 1, j + 1, map, counter);
			} else if j == MAX_J - 1 {
				flip_to1(i - 1, j - 1, map, counter);
				flip_to1(i - 1, j, map, counter);
				flip_to1(i, j - 1, map, counter);
				flip_to1(i + 1, j - 1, map, counter);
				flip_to1(i + 1, j, map, counter);
			} else {
				flip_to1(i - 1, j - 1, map, counter);
				flip_to1(i - 1, j, map, counter);
				flip_to1(i - 1, j + 1, map, counter);
				flip_to1(i, j + 1, map, counter);
				flip_to1(i, j - 1, map, counter);
				flip_to1(i + 1, j - 1, map, counter);
				flip_to1(i + 1, j, map, counter);
				flip_to1(i + 1, j + 1, map, counter);
			}
		}
	}
}

fn main() {
	let mut map: [[u8; MAX_J]; MAX_I] = [
		[1, 1, 1, 0, 0, 0],
		[1, 0, 0, 0, 1, 1],
		[0, 0, 1, 0, 0, 1],
		[1, 1, 0, 0, 0, 0],
		[1, 1, 0, 1, 1, 1],
	];
	println!("map = [");
	for i in 0..MAX_I {
		println!("    {:?}", map[i]);
	}
	println!("]");
	let mut counter: Counter = Counter::new(true, 0);
	for i in 0..MAX_I {
		for j in 0..MAX_J {
			flip_to1(i, j, &mut map, &mut counter);
			if !counter.is_first {
				counter.is_first = true;
			}
		}
	}
	println!("the number of cluster: {}", counter.count);
}
