package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"problems/adventure_story/story"
)

func main() {
	port := flag.Int("port", 8080, "the port to start the web application on")
	filename := flag.String("file", "gopher.json", "the JSON file with the story")
	flag.Parse()
	fmt.Printf("Using the story in %s.\n", *filename)

	f, err := os.Open(*filename)
	if err != nil {
		log.Fatalf("open file err: %v\n", err)
	}

	storys := story.JsonStory(f)

	h := story.NewHandler(storys, nil)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", *port), h))
}
