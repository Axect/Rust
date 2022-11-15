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
    io::parquet::read,
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
    let l = data.len();
    let chunk = vec![Ok(Chunk::new(data))];

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Snappy,
        version: Version::V2
    };

    let encodings = (0 .. l).map(|_| vec![Encoding::Plain]).collect::<Vec<_>>();

    let row_groups = RowGroupIterator::try_new(
        chunk.into_iter(),
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

    // =========================================================================
    // Read the file
    // =========================================================================
    let mut reader = std::fs::File::open("test.parquet")?;
    let metadata = read::read_metadata(&mut reader)?;
    let schema = read::infer_schema(&metadata)?;

    let fields = schema.fields.clone();

    let row_groups = metadata.row_groups;
    let chunks = read::FileReader::new(reader, row_groups, schema, None, None, None);

    for may_chunk in chunks {
        let chunk = may_chunk?;
        println!("{:#?}", chunk);

        let arrs = chunk.into_arrays();
        println!("{:#?}", arrs);

        for (field, arr) in fields.iter().zip(arrs) {
            let name = &field.name;
            let data_type = field.data_type();
            let x = arr.as_any().downcast_ref::<PrimitiveArray<f64>>().unwrap();
            let x = x.values_iter().cloned().collect::<Vec<_>>();
            println!("{}: {:?} {:?}", name, data_type, x);
        }
    }

    Ok(())
}
