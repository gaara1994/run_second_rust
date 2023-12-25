rs:
	cargo build --release

build:
	sudo docker build -t yantao/run_seconds_rust:latest .

push:
	sudo docker push yantao/run_seconds_rust:latest

test:
	sudo docker run yantao/run_seconds_rust:latest run_second_rust failed 5	