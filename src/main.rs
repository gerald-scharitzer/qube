use hyper::{Client, Uri};
use qube::{Process, Processor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const VERSION: &str = "0.0.0-dev";
    println!("qube {VERSION}");

    let processor = Processor::new("processor0");
    println!("processor {}", processor.get_name());

    let process = Process::new("process0");
    println!("process {}", process.get_name());

    let client = Client::new();
    let url: Uri = "127.0.0.1".parse()?;
    Ok(())
}
