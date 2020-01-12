package main

import "net/http"

func getUser(r *http.Request) user {
	var u user

	c, err := r.Cookie("session")
	if err != nil {
		return u
	}

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
