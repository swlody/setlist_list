use eyre::Context;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use serde::{Deserialize, Serialize};
use setlist_list::app::App;

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    date: chrono::NaiveDate,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ctx = playground::<App>().await.context("playground")?;

    // let active_model: articles::ActiveModel = ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    println!("welcome to playground. edit me at `examples/playground.rs`");

    let req = r#"{"date": "2022-12-20"}"#;
    let params: Params = serde_json::from_str(req)?;
    let serialized = serde_json::to_string(&params)?;
    dbg!(serialized);

    Ok(())
}
