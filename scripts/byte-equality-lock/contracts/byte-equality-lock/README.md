# byte-equality-lock

This contract is a toy CKB lock script used for learning.

It loads its own script args with `load_script`, then loads the first group input witness with `load_witness_args`. The script reads the witness `lock` field and compares those bytes with the script args.

```text
script.args == witness.lock -> pass
script.args != witness.lock -> WrongSecret / exit code 5
missing witness.lock -> WrongSecret / exit code 5
```

The important learning point is that the script is not called with normal function arguments. It reads the transaction through CKB syscalls, so tests must build a transaction and place the proof in the same witness field the script would inspect on-chain.

This lock is insecure by design. It is only a bridge toward a real lock where `args` identify an owner and the witness contains a signature proving control of that owner key.

*This contract was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/cryptape/ckb-script-templates
