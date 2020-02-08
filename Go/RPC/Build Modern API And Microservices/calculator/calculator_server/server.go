package main

import (
	"context"
	"fmt"
	"gRPC_Via_Udemy/calculator/calculatorpb"
	"google.golang.org/grpc"
	"log"
	"net"
)

type server struct {}

func (*server) Calculate(ctx context.Context,
	req *calculatorpb.CalculateRequest) (*calculatorpb.CalculateResponse, error) {
	fmt.Printf("Calculate function was invoked with %v", req)
	sum := req.FirstNum + req.SecondNum
	res := &calculatorpb.CalculateResponse{
		Result:               sum,
	}
	return res, nil
}

func (*server) ManyCalculate(req *calculatorpb.ManyCalculateRequests,
	stream calculatorpb.CalculateService_ManyCalculateServer) error {
	fmt.Printf("ManyCalculate function was invoked with %v", req)
	num := req.FirstNum
	if num < 2 {
		stream.Send(&calculatorpb.ManyCalculateResponses{Result:0})
	}
	var k int32 = 2
	for num > 1 {
		if num % k == 0 {
			stream.Send(&calculatorpb.ManyCalculateResponses{Result:k})
			num /= k
		} else {
			k += 1
		}
	}
	return nil
}

func main() {
	lis, err := net.Listen("tcp", "0.0.0.0:50051")
	if err != nil {
		log.Fatalf("listen err: %v", err)
	}

	s := grpc.NewServer()
	calculatorpb.RegisterCalculateServiceServer(s, &server{})

	if err := s.Serve(lis); err != nil {
		log.Fatalf("Server err: %v", err)
	}
}
