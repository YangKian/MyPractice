package main

import (
	"log"
	"testing"
)

func TestFileSystemStore(t *testing.T) {
	t.Run("/league from a reader", func(t *testing.T) {
		database, cleanDatabase := createTempFile(t,`[
            {"Name": "Cleo", "Wins": 10},
            {"Name": "Chris", "Wins": 33}]`)
		defer cleanDatabase()

		store, err:= NewFileSystemPlayerStore(database)
		if err != nil {
			log.Fatalf("problem creating file system player store, %v ", err)
		}

		got := store.GetLeague()
		want := []Player{
			{"Cleo", 10},
			{"Chris", 33},
		}

		assertLeague(t, got, want)
	})

	t.Run("get player score", func(t *testing.T) {
		database, cleanDatabase := createTempFile(t,`[
        {"Name": "Cleo", "Wins": 10},
        {"Name": "Chris", "Wins": 33}]`)
		defer cleanDatabase()

		//store := FileSystemPlayerStore{database}
		store, err := NewFileSystemPlayerStore(database)
		if err != nil {
			log.Fatalf("problem creating file system player store, %v ", err)
		}

		got := store.GetPlayerScore("Chris")
		want := 33
		assertScoreEquals(t, got, want)
	})

	t.Run("store wins for existing players", func(t *testing.T) {
		database, cleanDatabase := createTempFile(t, `[
        {"Name": "Cleo", "Wins": 10},
        {"Name": "Chris", "Wins": 33}]`)
		defer cleanDatabase()

		//store := FileSystemPlayerStore{database}
		store, err := NewFileSystemPlayerStore(database)
		if err != nil {
			log.Fatalf("problem creating file system player store, %v ", err)
		}

		store.RecordWin("Chris")

		got := store.GetPlayerScore("Chris")
		want := 34
		assertScoreEquals(t, got, want)
	})

	t.Run("store wins for existing players", func(t *testing.T) {
		database, cleanDatabase := createTempFile(t, `[
        {"Name": "Cleo", "Wins": 10},
        {"Name": "Chris", "Wins": 33}]`)
		defer cleanDatabase()

		//store := FileSystemPlayerStore{database}
		store, err := NewFileSystemPlayerStore(database)
		if err != nil {
			log.Fatalf("problem creating file system player store, %v ", err)
		}

		store.RecordWin("Pepper")

		got := store.GetPlayerScore("Pepper")
		want := 1
		assertScoreEquals(t, got, want)
	})

}

func assertScoreEquals(t *testing.T, got, want int) {
	if got != want {
		t.Errorf("got %d want %d", got, want)
	}
}
