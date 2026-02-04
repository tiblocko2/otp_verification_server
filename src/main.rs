use std::sync::Arc;
use storage::postgres::PostgresOtpRepository;
use storage::repository::OtpRepository;
use sqlx::PgPool;
use di::Container;
use ports::httpapi::server::Server;
use dotenvy::dotenv;
use app::adapters::rabbitmq_producer::RabbitMqProducer;

pub mod storage;
pub mod di;
pub mod ports;
pub mod domain;
pub mod app;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL не установлена, создайте .env с DATABASE_URL");

    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .expect("RABBITMQ_URL не установлена");

    let rabbitmq_queue = std::env::var("RABBITMQ_QUEUE")
        .expect("RABBITMQ_QUEUE не установлена");

    // Создаём пул подключений к PostgreSQL
    let pool = Arc::new(PgPool::connect(&database_url).await?);

    // Создаём Postgres репозиторий для OTP
    let repo = Arc::new(PostgresOtpRepository::new(pool.clone())) as Arc<dyn OtpRepository>;

    // RabbitMQ producer
    let producer = Arc::new(
        RabbitMqProducer::new(&rabbitmq_url, &rabbitmq_queue)
            .await
            .expect("Не удалось создать RabbitMQ producer"),
    );

    // DI container
    let container = Arc::new(Container::new(repo, producer));

    // Запуск сервера
    let server = Server::new(8080, container.clone());

    server.run().await;

    Ok(())
}
