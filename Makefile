
test:
	cd mtlsrust/; cargo build; cd -
	cd mbedtls-2.24.0/; make test; cd -

