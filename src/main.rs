use std::fs;
use std::io::Read;

fn get_process_info(entry: &fs::DirEntry) {
    let cmdfile = entry.path().join("cmdline");
    if !cmdfile.exists() {
        println!("Could not find cmdfile - {}", cmdfile.to_str().unwrap_or("--"));
    }

    let mut cmdfile = fs::File::open(cmdfile).unwrap();
    let mut s = String::new();
    cmdfile.read_to_string(&mut s).unwrap();

    let v: Vec<&str> = s.split(|c: char| c == 0 as char).collect();
    let s = v.join(" ");

    println!("{} // {}", entry.path().display(), s);
}

fn is_process_dir(d: &str) -> bool {
    match d.parse::<u16>() {
        Err(_) => false,
        _ => true
    }
}

fn read_proc() -> Result<(), String> {
    let r = fs::read_dir("/proc");
    let r = try!(r.map_err(|e| e.to_string()));

    for entry in r {
        let entry = try!(entry.map_err(|e| e.to_string()));

        let metadata = entry.metadata();
        let metadata = try!(metadata.map_err(|e| e.to_string()));

        if metadata.is_dir() {
            let filename = entry.file_name()
                .into_string()
                .unwrap_or("unstringable".to_string());

            if is_process_dir(&filename) {
                get_process_info(&entry);
            }
        }
    }

    Ok(())
}

fn main() {
    match read_proc() {
        Err(e) => println!("Failed to read proc with error {}", e),
        _ => ()
    }
}
