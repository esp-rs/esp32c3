all: clean generate form fmt build

clean:
	rm -rf src/

generate:
	svd2rust --target riscv -i svd/esp32c3.svd

form:
	form -i lib.rs -o src/
	rm lib.rs

fmt:
	cargo fmt

build:
	cargo clean
	cargo build --target riscv32imc-unknown-none-elf
