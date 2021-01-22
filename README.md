# mbedtls-incr

This is the repo for incremental conversion to rust.

Workflow:
create a branch for your team, and commit changes to that branch.

mtlsrust/  folder contains the rust code.
mbedtls-2.24.0/  folder contains the base mbedtls c code.

Incremental porting:

See mtlsrust/src/pkcs12.rs  mbedtls-2.24.0/library/pkcs12.c  for an example of porting a single function from C to rust.

Example function is pkcs12_fill_buffer(). 


you can run the test suites with

```bash
    make test
```

clean up the workspace with
```
    make clean
```

