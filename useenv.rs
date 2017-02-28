use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::string::String;
use std::vec::Vec;

fn main() {
    let mut args = env::args().skip(1);
    let command_name = args.next().unwrap();
    let command_args: Vec<String> = args.collect();
    let mut command = Command::new(command_name);
    command.args(command_args.as_slice());

    let f = File::open(".env").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let mut parts = line.splitn(2, '=');
        let key = String::from(parts.next().unwrap());
        let value = String::from(parts.next().unwrap());

        command.env(key, value);
    }

    command.exec();
}
