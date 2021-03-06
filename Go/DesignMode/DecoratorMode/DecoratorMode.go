package main

import (
	"fmt"
	"log"
	"net/http"
)

type HttpHandlerDecorator func(handlerFunc http.HandlerFunc) http.HandlerFunc

func Handler(h http.HandlerFunc, decors ...HttpHandlerDecorator) http.HandlerFunc {
	for i := range decors {
		d := decors[i]
		h = d(h)
	}
	return h
}

func WithServerHeader(h http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Println("WithServerHeader()")
		w.Header().Set("Server", "HelloServer v0.0.1")
		h(w, r)
	}
}

func WithAuthCookie(h http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Println("WithAuthCookie()")
		cookie, err := r.Cookie("Auth")
		if err != nil || cookie.Value != "Pass" {
			w.WriteHeader(http.StatusForbidden)
			return
		}
		h(w, r)
	}
}

func hello(w http.ResponseWriter, r *http.Request) {
	log.Printf("Recieved Request %s from %s\n", r.URL.Path, r.RemoteAddr)
	fmt.Fprintf(w, "Hello, world " + r.URL.Path)
}

func main() {
	http.HandleFunc("/v4/hello", Handler(hello, WithServerHeader, WithAuthCookie))
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal("ListenAndServer err: ", err)
	}
}
