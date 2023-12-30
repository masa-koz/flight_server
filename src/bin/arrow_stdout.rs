use arrow::array::{Int32Array, Int64Array};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::StreamWriter;
use std::io::{stdout, BufWriter};
use std::sync::Arc;

fn write_to_stdout() -> arrow::error::Result<()> {
    let array = Int32Array::from(vec![1, 2, 3, 4, 5]);

    // 配列からフィールドを作成します。
    let field = Field::new("field_name", DataType::Int32, true);
    
    // フィールドからスキーマを作成します。
    let schema = Schema::new(vec![field]);
    
    // スキーマと配列を使用してレコードバッチを作成します。
    let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(array)]).unwrap();
    
    let mut writer = StreamWriter::try_new(
        BufWriter::new(stdout().lock()),
        &batch.schema()
        )?;
    writer.write(&batch)?;
    writer.finish()?;

    Ok(())
}

fn main() -> arrow::error::Result<()> {
    write_to_stdout()?;
    Ok(())
}