package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
)

var tpl *template.Template

func init() {
	tpl = template.Must(
		template.ParseFiles("./Chapter3_creating_your_own_server/" +
			"http_package/request/tpl.gohtml"))
}

type hotdog int

func (h hotdog) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	fmt.Println(r.Form)
	if err != nil {
		log.Fatalln(err)
	}
	tpl.ExecuteTemplate(w, "tpl.gohtml", r.Form)
}

func main () {
	var d hotdog
	http.ListenAndServe(":8080", d)
}
