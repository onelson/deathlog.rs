use std::env;
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn update_file<P: AsRef<Path>>(path: P) {
    let n: u32 = match read_file(&path) {
        Ok(s) => {
            if s.trim().is_empty() {
                0
            } else {
                s.parse().expect("unable to parse file content")
            }
        }
        Err(_) => 0,
    };
    
    let mut f = File::create(&path).expect("open file for write");
    
    let _ = write!(&mut f, "{}", n + 1).expect("unable to write");
    println!("wrote: {}", n + 1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("logging to: `{}`", &args[1]);

    loop {
        let mut buffer = [0; 1];
        io::stdin().read(&mut buffer).expect("read input");
        if buffer == [10] {
	    update_file(&args[1]);
        }
    }
}
