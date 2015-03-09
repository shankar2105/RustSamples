// File Operation using Rust

#![feature(old_io)]
#![feature(old_path)]
use std::old_io::File;
use std::old_io::stdin;

static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

// Function to write a file
fn WriteFile(outPath: &Path) {
	let display = outPath.display();

	// Creating a file
	let mut file = match File::create(&outPath) {
			Err(why) => panic!("couldn't open {}: {}", display, why.desc),
			Ok(file) => file,
	};

	// Writing the file
	match file.write_str(LOREM_IPSUM) {
		Err(why) => panic!("Couldn't write {}: {}", display, why.desc),
		Ok(_) => println!("Write successful... {}", display),
	}
}

// Function to read a file
fn ReadFile(outPath: &Path) {
	let display = outPath.display();

	// open a file to read
	let mut file = match File::open(&outPath) {
		Err(why) => panic!("couldn't open {}: {}", display, why.desc),
		Ok(file) => file,
	};

	// read the content in it
	match file.read_to_string() {
		Err(why) => panic!("couldn't open {}: {}", display, why.desc),
		Ok(string) => println!("{} contains:\n {}", display, string),
	}
}

fn main() {
	// Output file path
	let filePath = Path::new("out/hello.txt");
	loop {
		println!("File Operation::\n");
		println!("1 Write a File\n\n2 Read a File\n\n0 Exit\n\n");
		let choice = stdin().read_line().ok().expect("Failed to read line");

		// Converting the input string to a number and excluding the <return>
		let input_num: Result<u32, _> = choice.trim().parse();
		let num = match input_num {
				Ok(num) => num,
				Err(_) => {
					println!("Please input a number!");
					continue;
				}
		};

		// perform read or write operation based on
		match num {
			1 => {WriteFile(&filePath)},
			2 => {ReadFile(&filePath)},
			_ => break,
		}
	}
}
