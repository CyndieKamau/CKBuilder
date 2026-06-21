# byte-equality-lock

This is a deliberately insecure learning lock script for CKB.

The lock has one rule:

```text
witness.lock == script.args
```

The bytes in the lock script `args` are the requirement baked into the cell when it is created. The bytes in the witness `lock` field are the proof supplied when the cell is spent.

If the bytes match, the script exits with `0` and the transaction can pass verification. If they do not match, or if the witness lock field is missing, the script returns `WrongSecret` with exit code `5`.

This is not real authorization. Anyone who knows the bytes can unlock the cell. Its purpose is to make the lock-script flow easy to see before replacing byte equality with real signature verification.

## Tests

The test suite builds mock transactions with `ckb-testtool` and checks three cases:

- matching witness lock bytes pass
- wrong witness lock bytes fail with exit code `5`
- missing witness lock field fails with exit code `5`

Run:

```bash
make build CLANG=/opt/homebrew/opt/llvm@18/bin/clang
cargo test
```

*This project was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/nervosnetwork/ckb-script-templates
