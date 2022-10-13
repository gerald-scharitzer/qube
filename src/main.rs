use qube::{Process, Processor};

fn main() {
    const VERSION: &str = "0.0.0-dev";
    println!("qube {VERSION}");

    let processor = Processor::new("processor0");
    println!("processor {}", processor.get_name());

    let process = Process::new("process0");
    println!("process {}", process.get_name());
}
