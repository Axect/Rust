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
    let mut m_vec = u.sample(100000);
    m_vec[9301] = 0.1;
    for i in 0 .. 1000 {
        for j in 0 .. 100 {
            let m = m_vec[i * 100 + j];
            let matrix = rand(20, 5);
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

    let data = bucket.cursor().find(|data| {
        if let Data::KeyValue(kv) = data {
            let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
            dbid.m == 0.1
        } else {
            false
        }
    });

    if let Some(Data::KeyValue(kv)) = data {
        let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
        let matrix: Matrix = rmp_serde::from_slice(kv.value()).unwrap();
        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.print();
        key_to_update.push(dbid);
    }

    for key in key_to_update {
        let matrix = zeros(20, 5);
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

    let data = bucket.cursor().find(|data| {
        if let Data::KeyValue(kv) = data {
            let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
            dbid.m == 0.1
        } else {
            false
        }
    });

    if let Some(Data::KeyValue(kv)) = data {
        let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
        let matrix: Matrix = rmp_serde::from_slice(kv.value()).unwrap();
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
