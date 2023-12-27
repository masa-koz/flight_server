use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;
use std::env;
use std::io::BufReader;
use std::process::{Command, Stdio};

fn read_from_process(program: &String) -> arrow::error::Result<Vec<RecordBatch>> {
    let mut child = Command::new(program)
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let reader = StreamReader::try_new(BufReader::new(stdout), None)?;
    reader.collect()
}

fn main() -> arrow::error::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <program>", args[0]);
        return Ok(());
    }
    let batches = read_from_process(&args[1])?;
    println!("Read batches: {:?}", batches);
    Ok(())
}
