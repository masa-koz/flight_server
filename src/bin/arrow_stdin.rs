use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;
use std::io::{stdin, BufReader};

fn read_from_stdin() -> arrow::error::Result<Vec<RecordBatch>> {
    let reader = StreamReader::try_new(
        BufReader::new(stdin().lock()),
        None
        )?;
    reader.collect()
}

fn main() -> arrow::error::Result<()> {
    let batches = read_from_stdin()?;
    println!("Read batches: {:?}", batches);
    Ok(())
}