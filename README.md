# Modul-9-Tutorial-A-Publisher

## Understanding publisher and message broker

### How much data will the publisher send to the message broker in one run?

In one run, the publisher sends 5 event messages to the message broker. Each message is a `UserCreatedEventMessage` that contains a `user_id` and a `user_name`. The five messages are published to the same `user_created` queue, so RabbitMQ receives five user-created events every time the publisher program is executed.

### What does it mean if the URL is the same as in the subscriber program?

The URL `amqp://guest:guest@localhost:5672` being the same means both the publisher and subscriber connect to the same RabbitMQ message broker. The publisher uses that connection to send messages, while the subscriber uses it to listen for and process messages from the queue. Because both programs point to the same broker address, port, username, and password, they can communicate through RabbitMQ even though they do not call each other directly.
