package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "<h1>Hello from Go (Backstage Template)</h1><p>Managed by Traefik at superlab.app</p>")
	})

	fmt.Println("Server starting at :80")
	http.ListenAndServe(":80", nil)
}
