package main

import (
	"io"
	"net/http"
)

func d(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, "dog dog dog")
}

func c(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, "dog dog dog")
}

func main() {
	http.HandleFunc("/dog", d)
	http.HandleFunc("/cat", c)

	http.Handle("/dog", http.HandlerFunc(d))

	http.ListenAndServe(":8080", nil)
}
