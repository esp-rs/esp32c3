OUTPUT=esp32c3.svd
BASE=esp32c3.base.svd

all: clean patch generate form fmt build

codegen: clean generate form fmt build

clean:
	rm -rf src/

patch:
	rm -f svd/$(OUTPUT)
	svd patch svd/patches/esp32c3.yaml
	mv svd/$(BASE).patched svd/$(OUTPUT)

generate:
	svd2rust --target riscv -i svd/$(OUTPUT)

form:
	form -i lib.rs -o src/
	rm lib.rs

fmt:
	cargo fmt

build:
	cargo clean
	cargo build --target riscv32imc-unknown-none-elf
