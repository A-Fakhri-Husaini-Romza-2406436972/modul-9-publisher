use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::*,
    BasicProperties,
    Connection,
    ConnectionProperties,
};
use lapin::types::FieldTable;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() {
    let conn = Connection::connect(
        "amqp://guest:guest@127.0.0.1:5672/%2f",
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect");

    let channel = conn.create_channel().await.expect("Create channel failed");

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue declare failed");

    let users = vec![
        ("1", "2406436972-Amir"),
        ("2", "2406436972-Budi"),
        ("3", "2406436972-Cica"),
        ("4", "2406436972-Dira"),
        ("5", "2406436972-Emir"),
    ];

    for (id, name) in users {
        let message = UserCreatedEventMessage {
            user_id: id.to_string(),
            user_name: name.to_string(),
        };

        let payload = message.try_to_vec().expect("Serialize failed");

        channel
            .basic_publish(
                "",
                "user_created",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .expect("Publish failed")
            .await
            .expect("Confirm failed");

        println!("Message sent: {:?}", message);
    }
}