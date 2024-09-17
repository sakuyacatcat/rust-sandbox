.PHONY: build run clean

build:
	rustc ./src/$(FILE).rs -o ./bin/$(FILE)

run: build
	./bin/$(FILE)

clean:
	rm -rf ./bin/*
