package main

import (
	"context"
	"fmt"
	"gRPC_Via_Udemy/calculator/calculatorpb"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
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
	//doUnary(c)
	//doServerStream(c)
	//doClientStream(c)
	//doBiDiStream(c)
	doErrorUnary(c)
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

func doClientStream(c calculatorpb.CalculateServiceClient) {
	requests := []*calculatorpb.AvgCalculateRequests {{Num: 1}, {Num: 2}, {Num: 3}, {Num: 4}}

	stream, err := c.AverageCalculate(context.Background())
	if err != nil {
		log.Fatalf("error while calling AverageCalculate RPC: %v", err)
	}

	for _, req := range requests {
		fmt.Println("Sending req: ", req)
		stream.Send(req)
	}

	res, err := stream.CloseAndRecv()
	if err != nil {
		log.Fatalf("error while reading from Response: %v", err)
	}
	log.Println("res from response: ", res)
}

func doBiDiStream(c calculatorpb.CalculateServiceClient) {
	stream, err := c.FindMax(context.Background())
	if err != nil {
		log.Fatalf("Error while creating stream: %v", err)
		return
	}
	requests := []*calculatorpb.FindMaxRequests {{Num: 11}, {Num: 2}, {Num: 12}, {Num: 4}, {Num: 6}}
	waitc := make(chan struct{})

	go func() {
		for _, req := range requests {
			fmt.Printf("send req: %v\n", req)
			stream.Send(req)
		}
		stream.CloseSend()
	}()

	go func() {
		for {
			res, err := stream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				close(waitc)
				log.Fatalf("Error while receiving from response: %v", err)
				return
			}
			log.Printf("response message : %v\n", res.GetResult())
		}
		close(waitc)
	}()
	<- waitc
}

func doErrorUnary(c calculatorpb.CalculateServiceClient) {
	fmt.Println("Starting to do a SquareRoot Unary RPC...")
	doErrorCall(c, 10)
	doErrorCall(c, -2)
}

func doErrorCall(c calculatorpb.CalculateServiceClient, n int32) {
	res, err := c.SquareRoot(context.Background(), &calculatorpb.SquareRootRequest{
		Number:               n,
	})
	if err != nil {
		respErr, ok := status.FromError(err)
		if ok {
			//actual error from gRPC
			fmt.Println("Error message received from server: ", respErr.Message())
			fmt.Println("Error code: ", respErr.Code())
			if respErr.Code() == codes.InvalidArgument {
				fmt.Println("we probably sent a negative number!")
			}
		} else {
			log.Fatalf("Big Error calling SquareRoot: %v\n", err)
		}
	}
	fmt.Printf("Result of square root of %v: %v\n", n, res.GetNumberRoot())
}
