use peroxide::fuga::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct DBID {
    id: (usize, usize),
    m: u64,
}

impl DBID {
    fn new(id: (usize, usize), m: f64) -> Self {
        Self { id, m: m.to_bits() }
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "matrix/" if it exists
    let file = std::fs::File::open("matrix/");
    if file.is_ok() {
        std::fs::remove_dir_all("matrix/")?;
    }

    let db_path = std::path::Path::new("matrix/");
    std::fs::create_dir(db_path)?;

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(100000);
    m_vec[9301] = 0.1;

    let mut dbid_map = HashMap::new();

    for i in 0 .. 10 {
        let run_dir = format!("matrix/run_{:04}", i);
        for j in 0 .. 1000 {
            let trial_dir = format!("{}/trial_{:04}", run_dir, j);
            std::fs::create_dir_all(&trial_dir)?;

            let m = m_vec[i * 1000 + j];
            let matrix = rand(2000, 5);
            let dbid = DBID::new((i, j), m);

            let data = matrix.data.clone();
            let row = matrix.row;
            let col = matrix.col;
            let mut df = DataFrame::new(vec![]);
            df.push("data", Series::new(data));
            df.push("row", Series::new(vec![row]));
            df.push("col", Series::new(vec![col]));

            let file_name = format!("{}/data.nc", trial_dir);
            dbid_map.insert(dbid, file_name.clone());
            df.write_nc(&file_name)?;
        }
    }

    let map_path = std::path::Path::new("matrix/map.bin");
    bincode::serialize_into(std::fs::File::create(map_path)?, &dbid_map)?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let map: HashMap<DBID, String> = bincode::deserialize_from(std::fs::File::open("matrix/map.bin")?)?;

    for (dbid, file_name) in map.iter() {
        if f64::from_bits(dbid.m) == 0.1 {
            let mut df = DataFrame::read_nc(file_name)?;
            df.as_types(vec![F64, USIZE, USIZE]);
            let data: Vec<f64> = df["data"].to_vec();
            let row: usize = df["row"].at_raw(0);
            let col: usize = df["col"].at_raw(0);

            let matrix = matrix(data, row, col, Row);

            println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
            matrix.row(0).print();

            let matrix = zeros(2000, 4);

            println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
            matrix.row(0).print();

            let data = matrix.data.clone();
            let row = matrix.row;
            let col = matrix.col;

            let mut df = DataFrame::new(vec![]);
            df.push("data", Series::new(data));
            df.push("row", Series::new(vec![row]));
            df.push("col", Series::new(vec![col]));

            df.write_nc(file_name)?;
        }
    }

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let map: HashMap<DBID, String> = bincode::deserialize_from(std::fs::File::open("matrix/map.bin")?)?;

    for (dbid, file_name) in map.iter() {
        if f64::from_bits(dbid.m) == 0.1 {
            let mut df = DataFrame::read_nc(file_name)?;
            df.as_types(vec![F64, USIZE, USIZE]);

            let data: Vec<f64> = df["data"].to_vec();
            let row: usize = df["row"].at_raw(0);
            let col: usize = df["col"].at_raw(0);

            let matrix = matrix(data, row, col, Row);

            println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
            matrix.row(0).print();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let read_or_update_or_write = std::env::args().nth(1).unwrap();

    match read_or_update_or_write.as_str() {
        "write" => write()?,
        "update" => update()?,
        "read" => read()?,
        _ => println!("unknown command"),
    }

    Ok(())
}
