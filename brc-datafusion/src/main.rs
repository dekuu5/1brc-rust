use std::env;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::error::Result;
use datafusion::prelude::*;


fn get_input_file() -> String {
    let mut args = env::args();
    args.next();
    args.next().unwrap_or_else(|| {
        eprintln!("Usage: <program> <input_file>");
                std::process::exit(1);
    })
}


#[tokio::main]
async fn main(){
    
    let path = get_input_file();
    
    if let Err(e) = run(path).await {
            eprintln!("Error: {e}");
    }
    
}

async fn run(path: String) -> Result<()> { 
    let ctx = SessionContext::new();
    
    let schema = Schema::new(vec![
        Field::new("station", DataType::Utf8, false),
        Field::new("temperature", DataType::Float64, false),
    ]);
    
    let options = CsvReadOptions::new()
        .delimiter(b';')
        .has_header(false)
        .schema(&schema);
    ctx.register_csv("measurements", &path, options).await?;
    
    let df = ctx.sql(
        "SELECT station, MIN(temperature) as min_tmp,AVG(temperature) as avg_tmp, MAX(temperature) as max_tmp from measurements GROUP by station ORDER by station"
    ).await?;
    df.show().await?;
    Ok(())
}
