use backend::app_run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app_run().await
}
