package main

import (
	uuid "github.com/satori/go.uuid"
	"golang.org/x/crypto/bcrypt"
	"html/template"
	"net/http"
)

type user struct {
	UserName string
	Password []byte
	First string
	Last string
}

var tpl *template.Template
var dbUsers = map[string]user{}      // user ID, user
var dbSession = map[string]string{} // session ID, user ID

func init() {
	tpl = template.Must(template.ParseGlob("./Chapter9_Session/signup/*.gohtml"))
	bs, _ := bcrypt.GenerateFromPassword([]byte("password"), bcrypt.MinCost)
	dbUsers["test@test.com"] = user{"test@test.com", bs, "James", "Bond"}
}

func main() {
	http.HandleFunc("/", index)
	http.HandleFunc("/bar", bar)
	http.HandleFunc("/signup", signup)
	http.HandleFunc("/login", login)
	http.HandleFunc("/logout", logout)
	http.Handle("/favicon.ico", http.NotFoundHandler())
	http.ListenAndServe(":8080", nil)
}

func bar(w http.ResponseWriter, r *http.Request){
	u := getUser(w, r)
	if !alreadyLoggedIn(r) {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}
	tpl.ExecuteTemplate(w, "bar.gohtml", u)
}

func index(w http.ResponseWriter, r *http.Request){
	u := getUser(w, r)
	tpl.ExecuteTemplate(w, "index.gohtml", u)
}
func signup(w http.ResponseWriter, r *http.Request){
	if alreadyLoggedIn(r) {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	var u user
	if r.Method == http.MethodPost {
		un := r.FormValue("username")
		p := r.FormValue("password")
		f := r.FormValue("firstname")
		l := r.FormValue("lastname")

		if _, ok := dbUsers[un]; ok {
			http.Error(w, "Username already taken", http.StatusForbidden)
			return
		}

		sID := uuid.NewV4()
		c := &http.Cookie{
			Name:       "session",
			Value:      sID.String(),
			HttpOnly:   true,
		}

		http.SetCookie(w, c)

		dbSession[c.Value] = un
		bs, err := bcrypt.GenerateFromPassword([]byte(p), bcrypt.MinCost)
		if err != nil {
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}

		u = user{un, bs, f, l}
		dbUsers[un] = u
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}
	tpl.ExecuteTemplate(w, "signup.gohtml", u)
}
func login(w http.ResponseWriter, r *http.Request){
	if alreadyLoggedIn(r) {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	if r.Method == http.MethodPost {
		un := r.FormValue("username")
		p := r.FormValue("password")

		u, ok := dbUsers[un]
		if !ok {
			http.Error(w, "Username and/or password do not match", http.StatusForbidden)
			return
		}

		err := bcrypt.CompareHashAndPassword(u.Password, []byte(p))
		if err != nil {
			http.Error(w, "Username and/or password do not match", http.StatusForbidden)
			return
		}
		sID := uuid.NewV4()
		c := &http.Cookie{
			Name:  "session",
			Value: sID.String(),
		}
		http.SetCookie(w, c)
		dbSession[c.Value] = un
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}
	tpl.ExecuteTemplate(w, "login.gohtml", nil)
}

func logout(w http.ResponseWriter, r *http.Request){
	if !alreadyLoggedIn(r) {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	c, _ := r.Cookie("session")
	delete(dbSession, c.Value)

	c = &http.Cookie{
		Name:       "session",
		Value:      "",
		MaxAge:     -1,
		HttpOnly:   true,
	}
	http.SetCookie(w, c)
	http.Redirect(w, r, "/login", http.StatusSeeOther)
}