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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut builder = DatabaseBuilder::new();
    //builder.define::<DBMatrix>()?;

    //let db = builder.create("matrix.db")?;
    //let rw = db.rw_transaction().unwrap();
    //let u = Uniform(0.0, 1.0);
    //for i in 0 .. 10 {
    //    for j in 0 .. 10 {
    //        let m = u.sample(1)[0];
    //        let matrix = rand(10, 10);
    //        rw.insert(
    //            DBMatrix::from_param_and_matrix((i, j), m, matrix)
    //        )?;
    //    }
    //}
    //
    //let m = 0.1;
    //let key = (1, 2);
    //rw.insert(
    //    DBMatrix::from_param_and_matrix(key, m, rand(10, 10))
    //)?;
    //rw.commit()?;

    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;
    let db = builder.open("matrix.db")?;
    let r = db.r_transaction()?;

    for item in r.scan().secondary::<DBMatrix>(DBMatrixKey::m)?.range(0.0..0.02) {
        println!("id: {:?}, m: {:.4}", item.id, item.m);
        item.matrix.print();
    }

    Ok(())
}
