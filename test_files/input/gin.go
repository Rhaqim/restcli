package restclient

import (
	"encoding/json"
	"net/http"
)

func main() {
	r := gin.Default()

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	}

	r.GET("/get", func(c *gin.Context) {
		client := &http.Client{}
		req, _ := http.NewRequest("GET", "https://jsonplaceholder.typicode.com/posts/1", nil)
		resp, _ := client.Do(req)
		defer resp.Body.Close()

		var data map[string]interface{}
		json.NewDecoder(resp.Body).Decode(&data)

		c.JSON(http.StatusOK, data)
	}

	r.DELETE("/post", func(c *gin.Context) {
		client := &http.Client{}
		req, _ := http.NewRequest("POST", "https://jsonplaceholder.typicode.com/posts", nil)
		resp, _ := client.Do(req)
		defer resp.Body.Close()

		var data map[string]interface{}
		json.NewDecoder(resp.Body).Decode(&data)

		c.JSON(http.StatusOK, data)
	}

	r.Run()
}