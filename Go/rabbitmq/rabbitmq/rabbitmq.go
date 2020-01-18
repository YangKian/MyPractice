package rabbitmq

import (
	"github.com/streadway/amqp"
	"log"
)

const MQURL = "amqp://guest:guest@192.168.0.105:5672/imooc"

type RabbitMQ struct {
	conn      *amqp.Connection
	channel   *amqp.Channel
	QueueName string
	Exchange  string
	Key       string
	Mqurl     string
}

func NewRabbitMQ(queuename, exchange, key string) *RabbitMQ {
	rabbitMQ := &RabbitMQ{
		QueueName: queuename,
		Exchange:  exchange,
		Key:       key,
		Mqurl:     MQURL,
	}

	var err error
	if rabbitMQ.conn, err = amqp.Dial(MQURL); err != nil {
		rabbitMQ.failOnError(err, "创建RabbitMQ失败")
	}
	if rabbitMQ.channel, err = rabbitMQ.conn.Channel(); err != nil {
		rabbitMQ.failOnError(err, "创建channel失败")
	}
	return rabbitMQ
}

func (mq *RabbitMQ) Destory() {
	mq.channel.Close()
	mq.conn.Close()
}

func (mq *RabbitMQ) failOnError(err error, message string) {
	if err != nil {
		log.Fatalf("%s:%s", message, err)
	}
}
