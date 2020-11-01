extern crate arrow;

use std::sync::Arc;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, BufReader};
use arrow::{
    array::{Int64Array, StringArray},
    datatypes::{Schema, Field, DataType},
    record_batch::RecordBatch,
    ipc::writer::FileWriter,
    ipc::reader::FileReader,
};

fn main() -> Result<(), Box<dyn Error>> {
    let id1 = Int64Array::from(vec![1,2,3,4,5]);
    let id2 = StringArray::from(vec!["hi", "hello", "a", "b", "c"]);
    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            Field::new("id1", DataType::Int64, false),
            Field::new("id2", DataType::Utf8, false),
        ])),
        vec![Arc::new(id1), Arc::new(id2)]
    )?;

    println!("{}", batch.num_columns());
    println!("{}", batch.num_rows());

    // Prepare for writing
    let file = File::create("data/test.dat")?;
    let buf_writer = BufWriter::new(file);

    // IPC writing
    let mut fw = FileWriter::try_new(buf_writer, &batch.schema())?;
    fw.write(&batch)?;
    fw.finish()?;

    println!("Success writing");

    // Prepare for reading
    let read_file = File::open("data/test.dat")?;
    let buf_reader = BufReader::new(read_file);

    // IPC reading
    let sr = FileReader::try_new(buf_reader)?;
    println!("Success");
    for rrb in sr.into_iter() {
        let rb = rrb.unwrap();
        println!("{}", rb.num_columns());
        println!("{:?}", rb.column(0));
        println!("{:?}", rb.column(1));
    }

    Ok(())
}
