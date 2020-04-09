# Implementing OAuth 2.0 with Go(Golang) 

- Learn by the blog: https://www.sohamkamani.com/blog/golang/2018-06-24-oauth-with-golang/
- Run:
  - Register your new application on Github : https://github.com/settings/applications/new. In the "callback URL" field, enter "http://localhost:8080/oauth/redirect". Once you register, you will get a client ID and client secret.
  - Replace the values of the `clientID` and `clientSecret` variables in the [main.go](https://github.com/sohamkamani/go-oauth-example/blob/master/main.go) file and also the [index.html](https://github.com/sohamkamani/go-oauth-example/blob/master/public/index.html#L14) file
  - Start the server by executing `go run main.go`
  - Navigate to [http://localhost:8080](http://localhost:8080/) on your browser.