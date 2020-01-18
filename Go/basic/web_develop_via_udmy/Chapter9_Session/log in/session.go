package main

import (
	uuid "github.com/satori/go.uuid"
	"net/http"
)

func getUser(w http.ResponseWriter, r *http.Request) user {
	var u user

	c, err := r.Cookie("session")
	if err != nil {
		sID := uuid.NewV4()
		c = &http.Cookie{
			Name:       "session",
			Value:      sID.String(),
			HttpOnly:   true,
		}
	}

	http.SetCookie(w, c)

	if un, ok := dbSession[c.Value]; ok {
		u = dbUsers[un]
	}
	return u
}

func alreadyLoggedIn(r *http.Request) bool {
	c, err := r.Cookie("session")
	if err != nil {
		return false
	}

	un := dbSession[c.Value]
	_, ok := dbUsers[un]
	return ok
}
