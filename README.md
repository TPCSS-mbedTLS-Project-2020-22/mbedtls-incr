# mbedtls-incr

This is the repo for incremental conversion to rust.

Workflow:
create a branch for your team, and commit changes to that branch.

Incremental porting:

See library/pkcs12.c and src/pkcs12.rs for an example.

you can run the test suites with

```bash
    cd mtlsrust/
    make test
```

clean up the workspace with
```
    cd mtlsrust/
    make clean
```

