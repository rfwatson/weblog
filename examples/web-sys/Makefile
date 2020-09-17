all: build

build:
	wasm-pack build --target web --out-name wasm --out-dir ./static

run:
	simple-http-server -i static -p 3000 --nocache --try-file ./static/index.html
