package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"net/http"
	"strings"
)

func main() {
	listen, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Panic(err)
	}
	defer listen.Close()
http.ListenAndServe()
	for {
		conn, err := listen.Accept()
		if err != nil {
			log.Println(err)
		}
		go handle(conn)
	}

}

func handle(conn net.Conn) {
	defer conn.Close()
	request(conn)
}

func request(conn net.Conn) {
	scanner := bufio.NewScanner(conn)
	row := 0
	for scanner.Scan() {
		ln := scanner.Text()
		if row == 0 {
			mux(conn, ln)
		}
		if ln == "" {
			break
		}
		row++
		fmt.Println(ln)
	}
	fmt.Println("\n")
}

func mux(conn net.Conn, ln string) {
	method := strings.Fields(ln)[0]
	url := strings.Fields(ln)[1]
	fmt.Println("Method: ", method)
	fmt.Println("URL: ", url)
	switch method {
	case "GET":
		switch url {
		case "/":
			index(conn)
		case "/about":
			about(conn)
		case "/contact":
			contact(conn)
		case "/apply":
			apply(conn)
		}
	case "POST":
		switch url {
		case "/apply":
			applyProcess(conn)
		}
	}
}

func index(conn net.Conn) {
	body := `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8">
		<title></title></head><body><strong>INDEX</strong><br>
		<a href="/">index</a><br>
		<a href="/about">about</a><br>
		<a href="/contact">contact</a><br>
		<a href="/apply">apply</a><br>
		</body></html>`

	fmt.Fprint(conn, "HTTP/1.1 200 OK\r\n")
	fmt.Fprintf(conn, "Content-Length: %d\r\n", len(body))
	fmt.Fprint(conn, "Content-Type: text/html\r\n")
	fmt.Fprint(conn, "\r\n")
	fmt.Fprint(conn, body)
}

func about(conn net.Conn) {
	body := `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8">
		<title></title></head><body><strong>ABOUT</strong><br>
		<a href="/">index</a><br>
		<a href="/about">about</a><br>
		<a href="/contact">contact</a><br>
		<a href="/apply">apply</a><br>
		</body></html>`

	fmt.Fprint(conn, "HTTP/1.1 200 OK\r\n")
	fmt.Fprintf(conn, "Content-Length: %d\r\n", len(body))
	fmt.Fprint(conn, "Content-Type: text/html\r\n")
	fmt.Fprint(conn, "\r\n")
	fmt.Fprint(conn, body)
}

func contact(conn net.Conn) {
	body := `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8">
		<title></title></head><body><strong>CONTACT</strong><br>
		<a href="/">index</a><br>
		<a href="/about">about</a><br>
		<a href="/contact">contact</a><br>
		<a href="/apply">apply</a><br>
		</body></html>`

	fmt.Fprint(conn, "HTTP/1.1 200 OK\r\n")
	fmt.Fprintf(conn, "Content-Length: %d\r\n", len(body))
	fmt.Fprint(conn, "Content-Type: text/html\r\n")
	fmt.Fprint(conn, "\r\n")
	fmt.Fprint(conn, body)
}

func apply(conn net.Conn) {
	body := `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8">
		<title></title></head><body><strong>APPLY</strong><br>
		<a href="/">index</a><br>
		<a href="/about">about</a><br>
		<a href="/contact">contact</a><br>
		<a href="/apply">apply</a><br>
		</body></html>`

	fmt.Fprint(conn, "HTTP/1.1 200 OK\r\n")
	fmt.Fprintf(conn, "Content-Length: %d\r\n", len(body))
	fmt.Fprint(conn, "Content-Type: text/html\r\n")
	fmt.Fprint(conn, "\r\n")
	fmt.Fprint(conn, body)
}

func applyProcess(conn net.Conn) {
	body := `<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8">
		<title></title></head><body><strong>INDEX</strong><br>
		<a href="/">index</a><br>
		<a href="/about">about</a><br>
		<a href="/contact">contact</a><br>
		<a href="/apply">apply</a><br>
		<form method="POST" action="/apply">
		<input type="submit" value="apply">
		</form>
		</body></html>`

	fmt.Fprint(conn, "HTTP/1.1 200 OK\r\n")
	fmt.Fprintf(conn, "Content-Length: %d\r\n", len(body))
	fmt.Fprint(conn, "Content-Type: text/html\r\n")
	fmt.Fprint(conn, "\r\n")
	fmt.Fprint(conn, body)
}