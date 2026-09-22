package apiserver

import (
	"log"
	"net/http"
)

type Server struct {
	Port string
}

func (s *Server) Run() {
	http.HandleFunc("/api/pods", apply)

    log.Println("Dinghy API server running on port", s.Port)
    http.ListenAndServe(":" + s.Port, nil)
}

func NewServer(port string) *Server {
	return &Server{
		port,
	}
}

