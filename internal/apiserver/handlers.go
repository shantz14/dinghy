package apiserver

import (
	"fmt"
	"log"
	"net/http"
)

func apply(w http.ResponseWriter, r *http.Request) {
	var body []byte
	_, err := r.Body.Read(body)
	if err != nil {
		log.Println("Error reading apply request body: ", err)
		return
	}
	fmt.Println(string(body))

	w.Write([]byte("Response body"))
}

