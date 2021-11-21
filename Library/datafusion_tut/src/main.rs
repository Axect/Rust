use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
  // create the dataframe
  let mut ctx = ExecutionContext::new();
  let df = ctx.read_csv("data/Masan.csv", CsvReadOptions::new()).await?;

  // execute and print results
  df.show_limit(100).await?;
  Ok(())
}
