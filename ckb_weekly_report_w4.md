# Week Four Report

Name: Cyndie Kamau

Week Ending: 19/06/2026

## Courses Done

* `ckb-testtool` generated test template -> Went deeper into testing scripts as transactions instead of normal function calls.
* [Omnilock Script Documentation](https://docs.nervos.org/docs/ecosystem-scripts/omnilock#how-omnilock-works) -> Revisited Omnilock auth, args, modes, and witness structure.
* [RFC 0042 Omnilock](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0042-omnilock/0042-omnilock.md) -> Read the Omnilock witness format and examples more carefully.

## Key Learnings

This week I first finished testing my toy `byte-equality` lock, then used that as a stepping stone to understand Omnilock better.


My toy lock has one very simple rule:

`witness.lock == script.args`

So the bytes baked into the lock script args are the requirement, and the bytes supplied in the witness lock field are the proof.

That helped me see the lock script pattern more clearly:

**args ->** what the cell requires before it can be spent.

**witness ->** what the spender supplies at spend time to satisfy that requirement.

The toy lock is insecure because the proof is just the secret itself. Anyone who knows the bytes can unlock it.

But it is still useful because the shape is the same as a real lock:

The lock script loads its own args, loads the witness, checks the proof, then exits `0` or nonzero.

I then compiled using llvm since CKB VM executes RISC-V binaries:

```shell
make build CLANG=/opt/homebrew/opt/llvm@18/bin/clang
Cleaning build/release directory...
mkdir -p build/release
RUSTFLAGS="-C target-feature=+zba,+zbb,+zbc,+zbs -C passes=lower-atomic -C debug-assertions" TARGET_CC="/opt/homebrew/opt/llvm@18/bin/clang" TARGET_AR="/opt/homebrew/opt/llvm@18/bin/llvm-ar" \
		cargo build --target=riscv64imac-unknown-none-elf --release 
    Finished `release` profile [optimized + debuginfo] target(s) in 0.02s
Copying binary byte-equality-lock to build directory
```
All the 3 tests pass:

```shell
cargo test

running 3 tests
test tests::byte_equality_lock_accepts_matching_witness_lock ... ok
test tests::byte_equality_lock_rejects_missing_witness_lock ... ok
test tests::byte_equality_lock_rejects_wrong_witness_lock ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## CKB VM syscalls used in my toy lock


I used `WrongSecret=5` as the exit code my script returns after it has already loaded the data it needs and decided the check failed.

The syscalls are how the script reads transaction data from the CKB VM environment.

In my toy lock, I used two important high-level `ckb-std` helpers:

**`load_script()` ->** underneath this uses the CKB VM `load_script` syscall. It loads the currently running lock script, so I can read `script.args`.

**`load_witness_args(0, Source::GroupInput)` ->** underneath this uses the CKB VM `load_witness` syscall. It loads the witness for the first input in this script group, then parses it as `WitnessArgs`.

After that, the rest is local script logic:

**`witness_args.lock().to_opt()` ->** checks whether the witness lock field exists.

**`raw_data()` ->** gets the raw bytes from `script.args` and `witness.lock`.

**byte comparison ->** compares the two byte arrays.

So the flow is:

```
load_script syscall -> read script args
load_witness syscall -> read WitnessArgs
local comparison -> decide pass or fail
return 0 or 5 -> VM observes the script exit code
```

The script also calls `debug!`, which uses the debug syscall for logging, but that is not part of the verification rule.



## Practical Progress

- [x] Added a success test for my toy lock where the witness lock bytes match the script args.
- [x] Added a failure test where the witness lock bytes are wrong.
- [x] Added an edge-case test where the witness lock field is missing.
- [x] Asserted the exact custom error code: `WrongSecret = 5`.
- [x] Updated the toy lock README to explain the args / witness relationship and why the lock is intentionally insecure.

The three cases now look like:

```
script args:  "open sesame"
witness lock: "open sesame"
result:       pass
```

```
script args:  "open sesame"
witness lock: "wrong secret"
result:       fail with WrongSecret / exit code 5
```

```
script args:  "open sesame"
witness lock: <missing>
result:       fail with WrongSecret / exit code 5
```

## Omnilock

After testing my own toy lock, now Omnilock made much more sense.

An Omnilock script still has the same normal CKB `Script` structure:

**code_hash ->** identifies the Omnilock binary.

**hash_type ->** tells CKB how to interpret the code hash.

**args ->** the per-cell configuration.

But Omnilock packs more meaning into `args` than my toy lock.

So the shape is:

`<21 byte auth> <Omnilock args>`

The first 21 bytes are:

`<1 byte auth flag> <20 bytes auth content>`

The auth flag tells Omnilock what kind of proof to expect.

For example:

**0x00 ->** auth content is the blake160 hash of a secp256k1 public key, so the witness must provide a valid secp256k1 signature.

**0x01 ->** Ethereum style unlocking.

**0x04 ->** Bitcoin style unlocking.

**0x06 ->** CKB multisig style unlocking.

**0xFC ->** owner-lock mode, where Omnilock checks that another input cell is locked by a matching owner script.

Then after auth, Omnilock can have extra mode flags:

**administrator mode ->** allows admin-list based recovery or revocation flows.

**anyone-can-pay mode ->** allows partial spending / receiving behavior.

**time-lock mode ->** adds a since constraint.

**supply mode ->** adds supply-related constraints.

So Omnilock is not one lock rule, but more like a configurable lock framework.

## Omnilock witness

My toy lock puts raw bytes directly in `WitnessArgs.lock`.

Omnilock also uses `WitnessArgs.lock`, but the bytes inside are a Molecule structure called `OmniLockWitnessLock`:

```
signature
omni_identity
preimage
```

For a normal signature unlock, the important field is `signature`.

The script confirms which auth should be used, checks the auth flag, builds the signing message, verifies the signature, recovers or checks the public key hash, and compares it to the auth content.

So the secret is not revealed.

The witness proves ownership without exposing the private key.

## My Toy lock Script vs Omnilock Script

| Piece | byte-equality-lock | Omnilock |
| --- | --- | --- |
| Requirement in args | Raw secret bytes | Auth flag + auth content + optional mode args |
| Proof in witness | Same raw secret bytes | Signature, admin identity, owner lock proof, or preimage depending on auth mode |
| Main check | Byte equality | Auth-specific verification |
| Security | Insecure by design | Real authorization framework |
| Good for | Learning script flow | Production-style interoperable locking |

So the bridge is:

**Toy lock -> does the witness contain the same bytes as args?**

**Omnilock -> does the witness contain a valid proof for the auth method encoded in args?**

## My Week 1 Omnilock args

My Week 1 output lock args were:

`0x017edb7555a71a4ef205ec24b1637bb77c07e615b600`

Decoded as:

**auth flag ->** `0x01` which is Ethereum-style auth.

**auth content ->** `0x7edb7555a71a4ef205ec24b1637bb77c07e615b`

**Omnilock flags ->** `0x00`, so no extra modes

So my week 1 cell used Omnilock's Ethereum-style auth path.

That helped me understand why the first 21 bytes matter so much, they define the base unlocking method before any optional mode is considered.

## Personal Footnotes

The big thing that clicked this week:

 Omnilock is still a lock script, so it reads args, still needs a witness, and will still only return a pass or a fail.


The difference is that my toy lock has one tiny hardcoded rule, while Omnilock has a structured rule system for different authentication methods.

So the next step is to model the simplest real version using `secp256k1/blake160` auth path. 

`args = blake160(public key)`

`witness = signature`

`script = recover public key from signature, hash it with blake160, compare it to args`

So it will be the real version of the toy lock's byte comparison, replacing byte auth with signature auth.
