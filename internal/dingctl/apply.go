package dingctl

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/shantz14/dinghy/api"
)

func apply(args []string) {
	config := ReadConfig()
	url := config.ServerAddress

	// TODO replace this with -f flag and yaml parsing instead of hardcoded
	payload := api.Pod {
		Kind: "Pod",
		Metadata: api.ObjectMeta{
			Name: "nginx",
			Labels: map[string]string{"app": "web"},
		},
		Spec: api.PodSpec{
			Containers: []api.Container{
				{Name: "nginx", Image: "nginx:1.27"},
			},
		},
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		log.Fatalf("Error marshalling JSON: %v", err)
	}

	url = url + "/api/pods"

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		log.Fatalf("Error creating request: %v", err)
	}

	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{
		Timeout: 10 * time.Second,
	}

	resp, err := client.Do(req)
	if err != nil {
		log.Fatalf("Error making POST request: %v", err)
	}

	defer resp.Body.Close()

	fmt.Printf("Status Code: %d\n", resp.StatusCode)

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Error reading response body: %v", err)
	}

	fmt.Printf("Response Body: \n%s\n", respBody)
}

