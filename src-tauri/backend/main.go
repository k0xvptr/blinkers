package main

import (
	"net/http"
	"github.com/k0xvptr/blinkers/containers"
)

func main() {
	http.HandleFunc("/api/createTask", containers.CreateTask)
	http.ListenAndServe(":8080", nil)
}
