mod process;

use std::fs;
use std::io::Read;

use process::Process;

fn get_process_info(entry: &fs::DirEntry) -> Option<Process> {

    let cmdfile = entry.path().join("cmdline");
    let name = String::from(cmdfile.to_str().unwrap_or("no-name"));

    if !cmdfile.exists() {
        println!("Could not find cmdfile - {}", name);
        return None;
    }

    let mut cmdfile = match fs::File::open(cmdfile) {
        Err(e) => {
            println!("Failed to open file {} with error {}", name, e);
            return None;
        }
        Ok(f) => f,
    };

    let mut s = String::new();

    match cmdfile.read_to_string(&mut s) {
        Err(e) => {
            println!("Failed to read file {}: error: {}", name, e);
        },
        _ => ()
    }

    let v: Vec<&str> = s.split(|c: char| c == 0 as char).collect();
    let s = v.join(" ");

    if s.is_empty() {
        return None;
    }

    let pid = entry.file_name().into_string().unwrap()
        .parse::<u16>().unwrap();
    let process = Process::new(pid, &s);

    Some(process)
}

fn is_process_dir(d: &str) -> bool {
    d.parse::<u16>().is_ok()
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
                let p = get_process_info(&entry);
                match p {
                    Some(p) => println!("{}", p),
                    _ => ()
                }
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
