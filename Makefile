
test:
	cd mtlsrust/; cargo build
	@echo --------------------
	# make the "FAIL" standout
	cd mbedtls-2.24.0/; make test | sed 's/FAIL/    FAIL/'

clean:
	cd mtlsrust/; cargo clean
	@echo --------------------
	cd mbedtls-2.24.0/; make clean
