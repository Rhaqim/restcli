PHONY: run-all run-p run-c run-r

run-all: 
	@echo "Running for all the generators"
	@cargo run -- -p -c -r test_files/input/gin.go
	
run-p:
	@echo "Running postman collection generator"
	@cargo run -- -p test_files/input/gin.go

run-c:
	@echo "Running curl generator"
	@cargo run -- -c test_files/input/gin.go

run-r:
	@echo "Running rest-client generator"
	@cargo run -- -r test_files/input/gin.go