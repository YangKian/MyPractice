package main

import "rabbitMQ/rabbitmq"

func main() {
	imoocOne := rabbitmq.NewRabbitMQRouting("exImooc", "imooc_two")
	imoocOne.RecieveRouting()
}
