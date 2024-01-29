use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use native_db::*;
use native_model::{native_model, Model};

#[derive(Debug, Serialize, Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct DBMatrix {
    #[primary_key]
    pub id: (u32, u32),
    #[secondary_key]
    pub m: f64,
    pub matrix: Matrix,
}

impl DBMatrix {
    pub fn from_param_and_matrix(id: (u32, u32), m: f64, matrix: Matrix) -> Self {
        Self {
            id,
            m,
            matrix,
        }
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "matrix.db" if exists
    std::fs::remove_file("matrix.db")?;
    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;

    let db = builder.create("matrix.db")?;
    let rw = db.rw_transaction().unwrap();
    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(100);
    m_vec[1] = 0.1;
    for i in 0 .. 10 {
        for j in 0 .. 10 {
            let m = m_vec[i * 10 + j];
            let matrix = rand(10, 10);
            rw.insert(
                DBMatrix::from_param_and_matrix((i as u32, j as u32), m, matrix)
            )?;
        }
    }
    rw.commit()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;

    let db = builder.open("matrix.db")?;
    let rw = db.rw_transaction().unwrap();
    let m = 0.1;
    for item in rw.scan().secondary::<DBMatrix>(DBMatrixKey::m)?.start_with(m) {
        println!("id: {:?}, m: {:.4}", item.id, item.m);
        item.matrix.print();

        let new_item = DBMatrix::from_param_and_matrix(item.id, item.m, zeros(10, 10));
        println!("new id: {:?}, m: {:.4}", new_item.id, new_item.m);
        new_item.matrix.print();
        rw.update(item, new_item)?;
    }
    rw.commit()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;
    let db = builder.open("matrix.db")?;
    let r = db.r_transaction()?;

    let item: DBMatrix = r.get().primary((0u32, 1u32))?.unwrap();
    println!("id: {:?}, m: {:.4}", item.id, item.m);
    item.matrix.print();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let read_or_write = std::env::args().nth(1).unwrap();

    match read_or_write.as_str().trim() {
        "read" => read()?,
        "update" => update()?,
        "write" => write()?,
        _ => panic!("Invalid argument"),
    }

    Ok(())
}
