all: test

test:
	@#cd mtlsrust/; cargo build
	cd mtlsrust/; cargo test   # run the rust tests, if any
	@# copy the generated libraries into mbedtls/library folder
	@#cp mtlsrust/target/debug/libmtlsrust.a   mbedtls-2.24.0/library/
	cp mtlsrust/target/debug/libmtlsrust.so  mbedtls-2.24.0/library/
	@echo --------------------
	@# make the "FAIL" standout
	cd mbedtls-2.24.0/; make test | sed 's/FAIL/    FAIL/'

clean:
	cd mtlsrust/; cargo clean
	-rm -f mbedtls-2.24.0/library/libmtlsrust.*
	@echo --------------------
	cd mbedtls-2.24.0/; make clean
