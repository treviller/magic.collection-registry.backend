use magic_collection_registry_backend::app::Application;
use magic_collection_registry_backend::configuration::loader::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let application = Application::build(configuration).expect("Failed to build application");

    application.run().await
}
