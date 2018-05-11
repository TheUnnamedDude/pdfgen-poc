use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::io::{Write, Read};
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone)]
pub struct HtmlConverter {
    instance: Arc<Mutex<Child>>
}

impl HtmlConverter {
    pub fn new() -> HtmlConverter {
        let child = Command::new("wkhtmltopdf")
            .arg("--read-args-from-stdin")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn().unwrap();
        HtmlConverter {
            instance: Arc::new(Mutex::new(child))
        }
    }

    pub fn run_command(&self, args: Vec<String>) {
        let mut child = self.instance.lock().unwrap();
        {
            let stdin = child.stdin.as_mut().unwrap();
            println!("{}", args.join(" "));
            writeln!(stdin, "{}", args.join(" "));
        }
        let stdout = child.stderr.as_mut().unwrap();
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        while !line.contains("Done") {
            reader.read_line(&mut line).unwrap();
            println!("\"{}\"", line.trim());
            println!("{}", line.trim() != "Done");
        }
        println!("{}", line);
    }
}
