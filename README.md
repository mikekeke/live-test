# something

Works fine:

```
make test-odra-casper
```

Throws error:

```
make run-livenet
```

Error log:

```
💁  INFO : Deploying "some_contract.wasm".
💁  INFO : Found wasm under "wasm/some_contract.wasm".
🙄  WAIT : Waiting 15s for "4c1bf2e2cba8d1d9835b1e3a56bff7a7eb5e706b84fd4b7717e53201dadbc8f7".
💁  INFO : Deploy "4c1bf2e2cba8d1d9835b1e3a56bff7a7eb5e706b84fd4b7717e53201dadbc8f7" successfully executed.
💁  INFO : Contract "hash-b832b6df25112283ae0d8c0f3eafedbdf2bdbb9225bea7fb3f3bf848122afbcb" deployed.
💁  INFO : Calling "hash-b832b6df25112283ae0d8c0f3eafedbdf2bdbb9225bea7fb3f3bf848122afbcb" with entrypoint "add_something".
🙄  WAIT : Waiting 15s for "8bb3473499edb42f9e32b8331ebc8c393be098f0517024f242790448c2dc50d3".
💁  INFO : Deploy "8bb3473499edb42f9e32b8331ebc8c393be098f0517024f242790448c2dc50d3" successfully executed.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("missing field `lock_status`", line: 0, column: 0)', /home/mike/.cargo/registry/src/github.com-1ecc6299db9ec823/odra-casper-livenet-0.4.0/src/casper_client.rs:368:66
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
make: *** [Makefile:2: run-livenet] Error 101
```