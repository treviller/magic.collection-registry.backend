use magic_collection_registry_backend::app::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application = Application::build()?;

    application.run().await
}
