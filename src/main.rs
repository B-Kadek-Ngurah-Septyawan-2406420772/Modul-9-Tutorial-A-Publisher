use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(async_reactor_trait::AsyncIo);

    let connection = Connection::connect("amqp://guest:guest@127.0.0.1:5672", options).await?;
    let channel = connection.create_channel().await?;

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions {
                durable: false,
                auto_delete: false,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    let messages = [
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406420772-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406420772-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406420772-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406420772-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406420772-Emir".to_owned(),
        },
    ];

    for message in messages {
        let payload = message.try_to_vec()?;
        channel
            .basic_publish(
                "",
                "user_created",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
    }

    println!("Published 5 user_created events.");
    Ok(())
}
