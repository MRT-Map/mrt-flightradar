use anyhow::Result;
use rocket::routes;

#[rocket::get("/")]
fn test() -> &'static str {
    "abc"
}

#[rocket::main]
async fn main() -> Result<()> {
    let _ = rocket::build().mount("/", routes![test]).launch().await?;
    Ok(())
}
