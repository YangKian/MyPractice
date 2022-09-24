package main

import (
	"fmt"
	"testing"
)

// TestCheckInit Check the relationship between the values obtained using
// different initialization methods and nil
func TestCheckInit(t *testing.T) {
	var s []string
	a := map[string]string{"1": "2", "3": "4"}
	t.Logf("s == nil: %v\n", s == nil)

	for k, v := range a {
		s = append(s, fmt.Sprintf("%s=%s", k, v))
	}

	s1 := []string{}
	t.Logf("s1 != nil: %v\n", s1 != nil)

	s2 := make([]string, 0)
	t.Logf("s2 != nil: %v\n", s2 != nil)

}
