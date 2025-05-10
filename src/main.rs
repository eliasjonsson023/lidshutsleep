use chrono::Local;
use std::fs;
use std::io::Write;
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

  //---------------------- FIND THE FIRST LID ------------------------- //
  for i in 0.. {
    if quit {
      break;
    };
    quit = Path::new(&format!("/proc/acpi/button/lid/LID{}/", i)).exists();
  }

  //------------- Write to /tmp/lidshutsleep everytime we go into suspended mode --//
  let log_path = "/tmp/lidshutsleep";
  let mut log_message: String;
  let mut f = std::fs::File::create(log_path)
    .unwrap_or_else(|why| panic!("Error: Could not write to {log_path}: {why}"));
  loop {
    if b {
      data = fs::read_to_string("/proc/acpi/button/lid/LID/state")
        .expect("Unable to read file")
        .trim()
        .to_string();
    } else {
      data = fs::read_to_string(&format!("/proc/acpi/button/lid/LID{}/state", i))
        .expect("Unable to read file")
        .trim()
        .to_string();
    }

    let my_vec: Vec<&str> = data.split_whitespace().collect::<Vec<&str>>();

    if my_vec[1] == "closed" {
      // ------------------- suspend, lid is closed -------------------------- //
      log_message = format!(
        "Suspended at {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
      );
      f.write_all(log_message.as_bytes()).unwrap_or_else(|why| {
        println!(
          "Error: Could not write to {log_path}: {why}\nAre there mutiple processes running?"
        )
      });
      std::process::Command::new("systemctl")
        .arg("suspend")
        .status()
        .expect("Failed to suspend");
    }
    thread::sleep(Duration::from_millis(5000));
  }
}
