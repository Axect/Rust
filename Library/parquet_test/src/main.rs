use arrow2::{
    array::PrimitiveArray,
    chunk::Chunk,
    datatypes::{Field, DataType, Schema},
    error::Result,
    io::parquet::write::{
        WriteOptions,
        CompressionOptions,
        RowGroupIterator,
        Version,
        FileWriter,
        Encoding,
    },
};

fn main() -> Result<()> {
    // Create a new file
    let file = std::fs::File::create("test.parquet").unwrap();

    // Declare arrays
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![6.0, 7.0, 8.0, 9.0, 10.0];

    let x = PrimitiveArray::from(x.into_iter().map(|t| Some(t)).collect::<Vec<_>>());
    let y = PrimitiveArray::from(y.into_iter().map(|t| Some(t)).collect::<Vec<_>>());

    // Create a schema
    let schema = Schema::from(vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
    ]);

    // Create a chunk
    let data = vec![x.boxed(), y.boxed()];
    let chunk = Chunk::new(data.clone());

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Snappy,
        version: Version::V2
    };

    let encodings = (0 .. data.len()).map(|_| vec![Encoding::Plain]).collect::<Vec<_>>();

    let row_groups = RowGroupIterator::try_new(
        vec![Ok(chunk)].into_iter(),
        &schema,
        options,
        encodings,
    )?;

    // Create a writer
    let mut writer = FileWriter::try_new(file, schema, options)?;

    // Write the file
    for row_group in row_groups {
        writer.write(row_group?)?;
    }

    let _ = writer.end(None)?;

    Ok(())
}
