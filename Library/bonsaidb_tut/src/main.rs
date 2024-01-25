use bonsaidb::core::document::{CollectionDocument, Emit};
use bonsaidb::core::schema::{Collection, CollectionMapReduce, ReduceResult, SerializedCollection, View, ViewSchema, ViewMapResult, ViewMappedValue, SerializedView};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::local::Database;
use serde::{Deserialize, Serialize};
use peroxide::fuga::*;

#[derive(Debug, Serialize, Deserialize)]
enum DBShape {
    Row,
    Col
}

impl DBShape {
    pub fn from_shape(shape: Shape) -> Self {
        match shape {
            Shape::Row => Self::Row,
            Shape::Col => Self::Col
        }
    }

    pub fn to_shape(&self) -> Shape {
        match self {
            Self::Row => Shape::Row,
            Self::Col => Shape::Col
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "matrices", views = [DBMatrixByID])]
struct DBMatrix {
    pub id: usize,
    pub data: Vec<f64>,
    pub row: usize,
    pub col: usize,
    pub shape: DBShape
}

#[derive(Debug, Clone, View, ViewSchema)]
#[view(collection = DBMatrix, key = usize, value = u32, name = "by-id")]
struct DBMatrixByID;

impl DBMatrix {
    pub fn from_id_and_matrix(id: usize, matrix: Matrix) -> Self {
        Self {
            id,
            data: matrix.data,
            row: matrix.row,
            col: matrix.col,
            shape: DBShape::from_shape(matrix.shape)
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        matrix(self.data.clone(), self.row, self.col, self.shape.to_shape())
    }
}

impl CollectionMapReduce for DBMatrixByID {
    fn map<'doc>(&self, document: CollectionDocument<DBMatrix>) -> ViewMapResult<'doc, Self::View> {
        document.header.emit_key_and_value(document.contents.id, 1)
    }

    fn reduce<'view>(&self, mappings: &[ViewMappedValue<'_, Self>], _rereduce: bool) ->
    ReduceResult<Self::View> {
        Ok(mappings.iter().map(|m| m.value).sum())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open::<DBMatrix>(StorageConfiguration::new("matrix.db"))?;

    //for i in 0 .. 10 {
    //    let matrix = rand(10, 10);
    //    DBMatrix::from_id_and_matrix(i, matrix).push_into(&db)?;
    //}

    //let matrix = rand(10, 10);
    //DBMatrix::from_id_and_matrix(2, matrix).push_into(&db)?;

    let matrices = DBMatrixByID::entries(&db).with_key(&2).query()?;
    println!("Number of matrices: {}", matrices.len());

    for entry in &DBMatrixByID::entries(&db).with_key(&2).query_with_collection_docs()? {
        println!("Id: {} & {}\nMatrix: \n{}", entry.document.header.id, entry.document.contents.id, entry.document.contents.to_matrix());
    }

    Ok(())
}

#[test]
fn runs() {
    drop(std::fs::remove_dir_all("matrix.db"));
    main().unwrap();
}
