use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use jammdb::{DB, Data};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct DBID {
    id: (usize, usize),
    m: f64,
}

impl DBID {
    fn new(id: (usize, usize), m: f64) -> Self {
        Self {
            id,
            m,
        }
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "matrix.db" if exists
    let file = std::fs::File::open("matrix.db");
    if file.is_ok() {
        std::fs::remove_file("matrix.db")?;
    }

    let db = DB::open("matrix.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.create_bucket("data")?;

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(100);
    m_vec[1] = 0.1;
    for i in 0 .. 10 {
        for j in 0 .. 10 {
            let m = m_vec[i * 10 + j];
            let matrix = rand(10, 10);
            let dbid = DBID::new((i, j), m);

            let dbid_ = rmp_serde::to_vec(&dbid)?;
            let matrix_ = rmp_serde::to_vec(&matrix)?;

            bucket.put(dbid_, matrix_)?;
        }
    }
    tx.commit()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("matrix.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let mut key_to_update = vec![];

    for data in bucket.cursor() {
        if let Data::KeyValue(kv) = data {
            let key = kv.key();
            let value = kv.value();
            let dbid: DBID = rmp_serde::from_slice(key)?;
            if dbid.m == 0.1 {
                let matrix: Matrix = rmp_serde::from_slice(value)?;
                println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
                matrix.print();

                key_to_update.push(dbid);
            }
        }
    }

    for key in key_to_update {
        let matrix = zeros(10, 10);
        let matrix_ = rmp_serde::to_vec(&matrix)?;

        println!("id: {:?}, m: {:.4}", key.id, key.m);
        matrix.print();

        bucket.put(rmp_serde::to_vec(&key)?, matrix_)?;
    }

    tx.commit()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("matrix.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let mut cursor = bucket.cursor();
    let dbid = DBID::new((0, 1), 0.1);
    cursor.seek(rmp_serde::to_vec(&dbid)?);

    if let Some(Data::KeyValue(kv)) = cursor.next() {
        let key = kv.key();
        let value = kv.value();
        let dbid: DBID = rmp_serde::from_slice(key)?;
        let matrix: Matrix = rmp_serde::from_slice(value)?;

        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.print();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let read_or_update_or_write = std::env::args().nth(1).unwrap();

    match read_or_update_or_write.as_str() {
        "write" => write()?,
        "update" => update()?,
        "read" => read()?,
        _ => (),
    }

    Ok(())
}
