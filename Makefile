
test:
	cd mtlsrust/; cargo build
	@echo --------------------
	cd mbedtls-2.24.0/; make test

clean:
	cd mtlsrust/; cargo clean
	@echo --------------------
	cd mbedtls-2.24.0/; make clean
