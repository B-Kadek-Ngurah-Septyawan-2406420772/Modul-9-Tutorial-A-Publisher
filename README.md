# Modul-9-Tutorial-A-Publisher

## Understanding publisher and message broker

### How much data will the publisher send to the message broker in one run?

In one run, the publisher sends 5 event messages to the message broker. Each message is a `UserCreatedEventMessage` that contains a `user_id` and a `user_name`. The five messages are published to the same `user_created` queue, so RabbitMQ receives five user-created events every time the publisher program is executed.

### What does it mean if the URL is the same as in the subscriber program?

The URL `amqp://guest:guest@localhost:5672` being the same means both the publisher and subscriber connect to the same RabbitMQ message broker. The publisher uses that connection to send messages, while the subscriber uses it to listen for and process messages from the queue. Because both programs point to the same broker address, port, username, and password, they can communicate through RabbitMQ even though they do not call each other directly.

## Running RabbitMQ as message broker

![RabbitMQ running](assets/rabbitmq-running.png)

## Sending and processing event

![Publisher console](assets/publisher-console.png)

![Subscriber console](assets/subscriber-console.png)

When the publisher program is executed, it sends five `UserCreatedEventMessage` events to RabbitMQ through the `user_created` queue. The publisher does not send the messages directly to the subscriber. Instead, RabbitMQ acts as the message broker that receives and stores the events until they are consumed.

The subscriber program listens to the same `user_created` queue. After the publisher sends the events, the subscriber receives them one by one and prints the message contents to the console. This shows the event-driven architecture flow: the publisher only publishes events, RabbitMQ routes the messages, and the subscriber processes the events independently.

## Monitoring chart based on publisher

![RabbitMQ message chart](assets/message-chart.png)

The spike in the RabbitMQ message chart appears when the publisher program is executed. Each run of the publisher sends five messages to the `user_created` queue, so RabbitMQ records a short burst of message activity. The chart rises because messages are entering the broker, then it goes back down after the subscriber consumes and acknowledges those messages.

When the publisher is run repeatedly, the chart can show repeated spikes or a higher temporary message rate. This happens because RabbitMQ receives several groups of five events in a short time. The spike therefore represents the broker activity caused by the publisher sending events, while the decrease after the spike shows that the messages have been processed by the subscriber.
