package main

import (
	"context"
	"gRPC_Via_Udemy/calculator/calculatorpb"
	"google.golang.org/grpc"
	"io"
	"log"
)

func main() {
	conn, err := grpc.Dial("localhost:50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("client err: %v", err)
	}
	defer conn.Close()

	c := calculatorpb.NewCalculateServiceClient(conn)
	doUnary(c)
	doServerStream(c)
}

func doUnary(c calculatorpb.CalculateServiceClient) {
	req := & calculatorpb.CalculateRequest{
		FirstNum:             10,
		SecondNum:            3,
	}
	res, err := c.Calculate(context.Background(), req)
	if err != nil {
		log.Fatalf("client server err: ", err)
	}
	log.Printf("the result is: %v", res.Result)
}

func doServerStream(c calculatorpb.CalculateServiceClient) {
	req := & calculatorpb.ManyCalculateRequests{
		FirstNum:             120,
	}
	res, err := c.ManyCalculate(context.Background(), req)
	if err != nil {
		log.Fatalf("error while calling CalculateManyTimes RPC: %v", err)
	}
	for {
		num, err := res.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalf("Response stream err: %v", err)
		}
		log.Println("Response from stream: ", num)
	}
}
