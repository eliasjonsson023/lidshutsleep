use std::fs;
use std::path::Path;
use std::{thread, time::Duration};

fn main() {
	let mut data: String;
    let mut i: i32 = 0;
	let mut b: bool = false;
	let mut quit: bool = false; 

	// declare 2 vars with the same values in one line
	let path = Path::new("/proc/acpi/button/lid/LID/").exists();
	(b, quit) = (path, path);

	for i in 0.. {
		if quit {break};
		quit = Path::new(&format!("/proc/acpi/button/lid/LID{}/", i)).exists();
	}

	loop {
		if b {
			println!("{:?}", b);
			data = fs::read_to_string("/proc/acpi/button/lid/LID/state").expect("Unable to read file").trim().to_string();
		} else {
			println!("{:?}", b);
			data = fs::read_to_string(&format!("/proc/acpi/button/lid/LID{}/state", i)).expect("Unable to read file").trim().to_string();
		}

		let my_vec: Vec<&str> = data.split_whitespace().collect::<Vec<&str>>();

		if my_vec[1] == "closed" {
			// hibernate if lid is closed
			std::process::Command::new("systemctl").arg("suspend").status().expect("Failed to suspend");
		}
		thread::sleep(Duration::from_millis(5000));
	}
}
