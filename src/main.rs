
use clap::Parser;
use std::error::Error;

mod error;

#[derive(Parser, Debug)]
#[command (author, version = "0.1", about="Ilox is a simple interpreter for the lox programming language", long_about = None)]
pub struct IloxApp {
    #[arg(short = 'f', long)]
    filename: Option<String>
}
fn main() {
    let app = IloxApp::parse();
    if let Some(ref s) = &app.filename {
        run_file(s).unwrap();
    } else  {
        run_prompt().unwrap();
    }
}


pub(crate) fn run_file(path: impl AsRef<std::path::Path>) -> Result<(), Box<dyn Error>>
{
    let code = std::fs::read_to_string(path)?;
    run(&code)
}

fn run(code: &str) -> Result<(), Box<dyn Error>> {
    println!("{code}");
    Ok(())
}


fn run_prompt() -> Result<(), Box<dyn Error>> {
    let  stdin = std::io::stdin();
    let mut buf = String::with_capacity(40);
    loop {
        stdin.read_line(&mut buf)?;
        run(&buf)?;
        buf.clear();
    }
}
