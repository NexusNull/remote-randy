use std::fs;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;

fn main() {
    let msg = match fs::remove_file("/run/randy.sock") {
        Ok(()) => "is fine",
        Err(_NotExist) => "does not exist",
        Err(_IsDir) => "is a directory",
        Err(_) => "is not yours",
    };
    println!("The file {}!", msg);

    let listener = UnixListener::bind("/run/randy.sock").unwrap();


    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer:[u8; 512] = [0; 512];
        let read_amount = stream.read(&mut buffer).unwrap();
        let request = std::str::from_utf8(&buffer[..read_amount]).unwrap();
        println!("{}", request);
        if(request.starts_with("nexus")){
            Command::new("/bin/bash")
                .arg("-c")
                .arg("touch /home/nexus/works")
                .output()
                .expect("failed to execute process");
        }
    }
}
