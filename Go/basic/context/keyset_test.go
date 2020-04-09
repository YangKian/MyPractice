package main

import (
	"context"
	"testing"
)

type TraceID string

type TraceIDKey int

func Test(t *testing.T) {
	// Create a traceID for this request.
	traceID := TraceID("f47ac10b-58cc-0372-8567-0e02b2c3d479")

	//Declare a key
	const traceIDKey = 0

	//Store the traceID value inside the context with a value of zero for the key type
	ctx := context.WithValue(context.Background(), traceIDKey, traceID)

	if uuid, ok := ctx.Value(traceIDKey).(TraceIDKey); ok {
		t.Log("TraceID: ", uuid)
	}

	// Retrieve that traceID value from the Context value bag not
	// using the proper key type.
	if _, ok := ctx.Value(0).(TraceID); !ok {
		t.Log("TraceID Not Found")
	}
}
