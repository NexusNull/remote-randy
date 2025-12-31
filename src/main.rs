use std::fs;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener};
use std::process::Command;

fn main() {
    let socket_path = "/run/randy.sock";
    let msg = match fs::remove_file(socket_path) {
        Ok(()) => "is fine",
        Err(_not_exist) => "does not exist",
    };
    println!("The file {}!", msg);

    let listener = UnixListener::bind(socket_path).unwrap();

    let mut perms = fs::metadata(socket_path).unwrap().permissions();
    perms.set_mode(0o772);
    fs::set_permissions(socket_path, perms).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer:[u8; 512] = [0; 512];
        let read_amount = stream.read(&mut buffer).unwrap();
        let request = std::str::from_utf8(&buffer[..read_amount]).unwrap();
        println!("{}", request);
        if request.starts_with("enable") {
            Command::new("/bin/bash")
                .arg("-c")
                .arg("/opt/randy/enable.sh")
                .output()
                .expect("failed to execute process");
        }
        if request.starts_with("disable") {
            Command::new("/bin/bash")
                .arg("-c")
                .arg("/opt/randy/disable.sh")
                .output()
                .expect("failed to execute process");
        }
    }
}
