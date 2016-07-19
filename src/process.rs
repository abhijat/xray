use std::fmt;

pub struct Process {
    pub pid: u16,
    pub cmd: String
}

impl Process {
    pub fn new(id: u16, s: &str) -> Process {
        Process { pid: id, cmd: s.to_string(), }
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] // {}", self.pid, self.cmd)
    }
}
