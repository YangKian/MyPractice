package rabbitmq

import (
	"fmt"
	"github.com/streadway/amqp"
	"log"
)

func NewRabbitMQPubSub(exchangeName string) *RabbitMQ {
	return NewRabbitMQ("", exchangeName, "")
}

//订阅模式下的生产者
func (mq *RabbitMQ) PublishPub(message string) {
	//1.尝试创建交换机
	if err := mq.channel.ExchangeDeclare(
		mq.Exchange, //交换机名
		"fanout",    //交换机的类型，订阅模式下为广播类型
		true,        //是否持久化
		false,       //是否自动删除
		false,       //如果设置为true，表示这个exchange不可以被client用来推送消息，仅用来进行exchange与exchange之间的绑定
		false,
		nil,
	); err != nil {
		mq.failOnError(err, "声明交换机失败")
	}

	//2.发送消息
	if err := mq.channel.Publish(
		mq.Exchange,
		"",
		false,
		false,
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        []byte(message),
		}); err != nil {
		mq.failOnError(err, "发送消息失败")
	}
}

//订阅模式下的消费者
func (mq *RabbitMQ) RecieveSub() {
	//1.尝试创建交换机
	if err := mq.channel.ExchangeDeclare(
		mq.Exchange, //交换机名
		"fanout",    //交换机的类型，订阅模式下为广播类型
		true,        //是否持久化
		false,       //是否自动删除
		false,       //如果设置为true，表示这个exchange不可以被client用来推送消息，仅用来进行exchange与exchange之间的绑定
		false,
		nil,
	); err != nil {
		mq.failOnError(err, "声明交换机失败")
	}

	//2.尝试创建队列
	q, err := mq.channel.QueueDeclare(
		"", //队列名称不写，随机生成
		false,
		false,
		true, //排他性要设置成true
		false,
		nil,
	)
	if err != nil {
		mq.failOnError(err, "队列创建失败")
	}

	//3.绑定队列到交换机
	if err := mq.channel.QueueBind(
		q.Name,      //队列名称
		"",          //订阅模式下key为空,
		mq.Exchange, //交换机
		false,
		nil,
	); err != nil {
		mq.failOnError(err, "队列绑定失败")
	}

	//4.消费信息
	msgs, err := mq.channel.Consume(
		q.Name,
		"",
		true,
		false,
		false,
		false,
		nil,
	)

	forever := make(chan bool)
	go func() {
		for d := range msgs {
			//实现逻辑函数
			log.Printf("Received a message: %s", d.Body)
		}
	}()
	fmt.Println("[*] Waiting for messages. To exit press CTRL+C")
	<-forever
}
