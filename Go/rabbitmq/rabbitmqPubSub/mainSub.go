package main

import (
	"rabbitMQ/rabbitmq"
)

func main() {
	mq := rabbitmq.NewRabbitMQPubSub("newProduct")
	mq.RecieveSub()
}
