dist.tar.gz: dist/index.html dist/wordle_bg.wasm dist/index.min.js dist/output.css
	tar -c $^ | gzip -9 > $@

pkg/wordle_bg.wasm: src/lib.rs src/words.txt
	rustup run nightly wasm-pack build --target web

dist/:
	mkdir dist

dist/index.html: dist/ index.html
	cp index.html dist/index.html

dist/wordle_bg.wasm: dist/ pkg/wordle_bg.wasm
	cp pkg/wordle_bg.wasm dist/wordle_bg.wasm

dist/index.min.js: dist/ pkg/wordle_bg.wasm index.jsx pkg/wordle.js
	npm run build

dist/output.css: input.css index.jsx dist/
	npx tailwindcss -i ./input.css -o ./dist/output.css