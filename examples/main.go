package restclient

import "fmt"

func main() {
	// Create a new client
	client := NewClient("http://localhost:8080")

	// Create a new request
	req := client.NewRequest()

	// Set the request method
	req.Method = "GET"

	// Set the request path
	req.Path = "/api/v1/users"

	// Send the request
	resp, err := client.Send(req)

	if err != nil {
		// Handle error
	}

	// Handle response
	fmt.Println(resp.Body)
}

type Client struct {
	URL string
}

func NewClient(url string) *Client {
	return &Client{
		URL: url,
	}
}

type Request struct {
	Method string
	Path   string
}

func (c *Client) NewRequest() *Request {
	return &Request{}
}

type Response struct {
	Body string
}

func (c *Client) Send(req *Request) (*Response, error) {
	return &Response{}, nil
}
