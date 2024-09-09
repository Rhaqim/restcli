package restclient

import (
	"encoding/json"
	"net/http"
)

func main() {
	http.HandleFunc("/health", health)
	// ensure add users is a POST request
	http.HandleFunc("/users", addUser)
	http.ListenAndServe(":8080", nil)
}

func health(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello, World!"))
}

type User struct {
	ID    int
	Name  string
	Email string
}

func addUser(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
		return
	}

	var user User
	err := json.NewDecoder(r.Body).Decode(&user)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// save user to database
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(user)

}
