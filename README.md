# Modul-9-Tutorial-A-Publisher

## Understanding publisher and message broker

### How much data will the publisher send to the message broker in one run?

In one run, the publisher sends 5 event messages to the message broker. Each message is a `UserCreatedEventMessage` that contains a `user_id` and a `user_name`. The five messages represent five different user-created events: Amir, Budi, Cica, Dira, and Emir. All of them are published to the same `user_created` queue, so RabbitMQ receives five user-created events every time the publisher program is executed. The publisher does not wait for the subscriber to finish processing each message before sending the next one. This shows that the publisher's responsibility is only to publish events to the broker, while the message broker handles the delivery process. Because of that, the publisher can finish quickly even if the subscriber is slower.

### What does it mean if the URL is the same as in the subscriber program?

The URL `amqp://guest:guest@localhost:5672` being the same means both the publisher and subscriber connect to the same RabbitMQ message broker. The first `guest` is the RabbitMQ username, and the second `guest` is the password for that user. The `localhost:5672` part means the broker is running on the same machine using port `5672`, which is the default AMQP port. The publisher uses that connection to send messages, while the subscriber uses it to listen for and process messages from the queue. Because both programs point to the same broker address, port, username, and password, they can communicate through RabbitMQ even though they do not call each other directly. This also means that if the URL is changed to another broker address, both programs must use the same new broker URL so they still communicate through the same message broker.

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

## Bonus: Sending and processing event on cloud

![Cloud RabbitMQ running](assets/bonus-cloud-rabbitmq-running.png)

![Cloud publisher console](assets/bonus-cloud-publisher-console.png)

![Cloud subscriber console](assets/bonus-cloud-subscriber-console.png)

For the bonus experiment, RabbitMQ was moved from the local machine to a Google Cloud VM. The publisher and subscriber programs still run the same event-driven flow, but they connect to the cloud broker through the VM public IP instead of using the local RabbitMQ container. The publisher sends five `UserCreatedEventMessage` events to the `user_created` queue on the cloud RabbitMQ broker.

The subscriber receives those messages from the cloud queue and prints the event data in the console. This proves that the publisher and subscriber do not need to be on the same machine as the message broker. As long as the AMQP URL, user credentials, and firewall ports are configured correctly, RabbitMQ can route events through the cloud just like it did locally.

## Bonus: Monitoring chart based on publisher on cloud

![Cloud RabbitMQ message chart](assets/bonus-cloud-message-chart.png)

The RabbitMQ chart on the cloud broker shows the same behavior as the local experiment. When the publisher is executed, it sends five `UserCreatedEventMessage` events to the `user_created` queue, so the message rate briefly increases. This creates a spike in the chart because RabbitMQ receives a burst of messages from the publisher.

After the subscriber consumes and acknowledges the messages, the message rate goes back down. This means the cloud RabbitMQ broker successfully acted as the middle layer between the publisher and subscriber. The spike is caused by publishing activity, and the return to idle shows that the subscriber has processed the queued events.
