use std::fs;
use std::io::Read;

struct Process {
    process_id: u16,
    process_string: String
}

impl Process {
    fn new() -> Process {
        Process { process_id: 0, process_string: String::new() }
    }
}

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
        Err(e) => println!("Failed to read file {} with error: {}", name, e),
        _ => ()
    }

    let v: Vec<&str> = s.split(|c: char| c == 0 as char).collect();
    let s = v.join(" ");

    if s.is_empty() {
        return None;
    }

    let process = Process {
        process_string: s,
        process_id: entry.file_name().into_string().unwrap().parse::<u16>().unwrap()
    };

    Some(process)
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
                let p = get_process_info(&entry);
                match p {
                    Some(p) => println!("[{}] :: {}", p.process_id, p.process_string),
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
