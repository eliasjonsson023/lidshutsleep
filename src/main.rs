use std::fs;
use std::{thread, time};

fn main() {
	let five_seconds = time::Duration::from_millis(5000);

	let mut data: String;
	loop {
		data = fs::read_to_string("/proc/acpi/button/lid/LID0/state").expect("Unable to read file").trim().to_string();
		let my_vec = data.split_whitespace().collect::<Vec<&str>>();

		if my_vec[1] == "closed" {
			// hibernate if lid is closed
			std::process::Command::new("systemctl").arg("suspend").status().expect("Failed to suspend.");
		}
		thread::sleep(five_seconds);
	}
}
