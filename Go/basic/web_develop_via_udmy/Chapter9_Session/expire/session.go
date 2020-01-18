package main

import (
	"fmt"
	uuid "github.com/satori/go.uuid"
	"net/http"
	"time"
)

func getUser(w http.ResponseWriter, r *http.Request) user {
	//检查请求中是否包含session
	c, err := r.Cookie("seesion")
	if err != nil {
		sID := uuid.NewV4()
		c = &http.Cookie{
			Name:       "session",
			Value:      sID.String(),
			HttpOnly:   true,
		}
	}
	c.MaxAge = sessionLength
	http.SetCookie(w, c)

	//检查数据库中是否存储过该session的信息
	var u user
	if s, ok := dbSessions[c.Value]; ok {
		s.lastActivity = time.Now()
		dbSessions[c.Value] = s
		u = dbUsers[s.un]
	}
	return u
}

func alreadyLoggedIn(w http.ResponseWriter, r *http.Request) bool {
	c, err := r.Cookie("session")
	if err != nil {
		return false
	}

	s, ok := dbSessions[c.Value]
	if ok {
		s.lastActivity = time.Now()
		dbSessions[c.Value] = s
	}
	_, ok = dbUsers[s.un]
	c.MaxAge = sessionLength
	http.SetCookie(w, c)
	return ok
}

func cleanSession() {
	fmt.Println("BEFORE CLEAN")
	showSession()
	for k, v := range dbSessions {
		if time.Now().Sub(v.lastActivity) > time.Second * 30 {
			delete(dbSessions, k)
		}
	}
	dbSessionsCleaned = time.Now()
	fmt.Println("AFTER CLEAN")
	showSession()
}

func showSession() {
	fmt.Println("********")
	for k, v := range dbSessions {
		fmt.Println(k, v.un)
	}
	fmt.Println("")
}