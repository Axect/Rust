use bonsaidb::core::document::{CollectionDocument, Emit};
use bonsaidb::core::schema::{Collection, CollectionMapReduce, ReduceResult, SerializedCollection, View, ViewSchema, ViewMapResult, ViewMappedValue, SerializedView};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::local::Database;
use serde::{Deserialize, Serialize};
use peroxide::fuga::*;

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "matrices", views = [DBMatrixByID])]
struct DBMatrix {
    pub id: (usize, usize),
    pub matrix: Matrix,
}

#[derive(Debug, Clone, View, ViewSchema)]
#[view(collection = DBMatrix, key = (usize, usize), value = u32, name = "by-id")]
struct DBMatrixByID;

impl DBMatrix {
    pub fn from_id_and_matrix(id: (usize, usize), matrix: Matrix) -> Self {
        Self {
            id,
            matrix,
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        self.matrix.clone()
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

    for i in 0 .. 10 {
        for j in 0 .. 10 {
            let matrix = rand(10, 10);
            DBMatrix::from_id_and_matrix((i, j), matrix).push_into(&db)?;
        }
    }

    let matrix = zeros(10, 10);
    DBMatrix::from_id_and_matrix((2,2), matrix).push_into(&db)?;

    let matrices = DBMatrixByID::entries(&db).with_key(&(2, 2)).query()?;
    println!("Number of matrices: {}", matrices.len());

    for entry in &DBMatrixByID::entries(&db).with_key(&(2,2)).query_with_collection_docs()? {
        println!("Id: {:?} & {:?}\nMatrix: \n{}", entry.document.header.id, entry.document.contents.id, entry.document.contents.to_matrix());
    }

    Ok(())
}

#[test]
fn runs() {
    drop(std::fs::remove_dir_all("matrix.db"));
    main().unwrap();
}
