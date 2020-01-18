package main

import "rabbitMQ/rabbitmq"

func main() {
	mq := rabbitmq.NewRabbitMQSimple("imoocSimple")
	mq.ConsumSimple()
}
