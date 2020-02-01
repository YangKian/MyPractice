package main

import (
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestRecordingWinsAndRetrievingThem(t *testing.T) {
	store := NewInMemoryPlayerStore()
	server := PlayerServer{store}
	player := "Pepper"

	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))
	go server.ServeHTTP(httptest.NewRecorder(), newPostWinRequest(player))

	time.Sleep(1*time.Second)
	response := httptest.NewRecorder()
	server.ServeHTTP(response, newGetScoreRequest(player))
	assertStatus(t, response.Code, http.StatusOK)
	assertResponseBody(t, response.Body.String(), "8")
}
