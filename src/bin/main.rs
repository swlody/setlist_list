use loco_rs::cli;
use migration::Migrator;
use setlist_list::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, Migrator>().await
}
