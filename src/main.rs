use std::io;
fn main() {
	let mut buf = String::new();
	io::stdin().read_line(&mut buf).unwrap();
	// get N
	let n = buf.trim().parse::<usize>().unwrap();
	// read N rows
	let rows:Vec<String> = read_multi_line(n);
	// create N rows table with vector
	let tbl:Vec<Vec<&str>> = rows.iter().map(|row|
		row.split_whitespace().collect()
	).collect();
	
	//get min value index
	let mut min = u32::MAX;
	let mut min_idx = usize::MIN;
	for (idx, row) in tbl.iter().enumerate(){
		let age = row[1].parse::<u32>().unwrap();
		if age < min {
			min = age;
			min_idx = idx;
		}
	}

	for idx in (min_idx..n).chain(0..min_idx) {println!("{}", tbl[idx][0])}
}

fn read_multi_line(count:usize)->Vec<String>	
{
	(0..count).map(|_i|{
		let mut buf = String::new();
		io::stdin().read_line(&mut buf).unwrap();
		buf
	}).collect::<Vec<String>>()
}