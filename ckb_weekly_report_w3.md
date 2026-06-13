# Week Three Report
Name: Cyndie Kamau
Week Ending: 12/06/2026
## Courses Done
* [Intro to Script](https://docs.nervos.org/docs/script/intro-to-script) ->Went through the Script data structure: code_hash, hash_type, args. How CKB-VM locates and runs script code.
* [Example: A Minimal Script](https://docs.nervos.org/docs/script/rust/rust-example-minimal-script) -> Tried out the sample carrot type script. 
* [RFC 0009 VM Syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md) -> Learnt how a running script reaches transaction data across the VM boundary.

## Key Learnings
A lock script is a RISC-V binary that runs on CKB-VM. 

CKB instead of inventing a custom instruction set like the EVM did it adopted RISC-V, a real hardware instruction set. 

Payoff -> any language with a RISC-V backend can target it, and entire other VMs (a JS engine, an EVM interpreter) can be compiled to RISC-V and hosted as ordinary scripts.

The script outputs only an exit code:

0 -> pass, 

nonzero -> fail and whole tx is rejected. 

So same tx, same exit code, always. That's what lets a user run the script client-side and know the on-chain result in advance.

**The Script data structure ->** A cell's lock is not raw code, but a structure with three fields:

**code_hash ->** fingerprint of the binary to run, not its location.


**hash_type ->** tells the VM how to interpret code_hash (data / data1 / data2 / type).

**args ->** the per-instance data baked in at creation, e.g. an owner's public key hash.

**code_hash vs cell_deps ->** `code_hash` is the requirement (the what). 

`cell_deps` -> the tx names the deployed cell holding the binary. The VM hashes that binary per `hash_type` and checks it equals `code_hash` before running. 

**The three hashes I had fused ->**
`code_hash` -> inside the Script structure, points to the binary.

public key hash -> inside args, names the authorized owner.

lock script hash -> hash of the whole Script structure, the identifier indexers use to find cells. The running script never touches this one.

**Syscalls ->** The VM is sealed so tx data lives in the node, so it does not directly access it.

 A syscall never hands over the whole tx at once. 
 
 So each call fetches one specified piece:
 
 * a whole field (args, a witness)
 * one attribute of one indexed cell (input 3's lock, output 1's capacity)

So full tx visibility is built by repeating indexed fetches, not granted in one call.

## Mental model: lock vs type
-> Input cells' lock scripts execute. 
-> Output cells' lock scripts do not.
-> A lock guards consumption, so it only needs to run when something is spent. 
-> Type scripts run on both inputs and outputs, because transition rules relate old state to new. 


**Group sources ->** A script polices only its own cells via GroupInput / GroupOutput (the subset of cells running this same script). 

This is how a token type script ignores unrelated cells in the same transaction.

## The default lock, walked end to end
`secp256k1_blake160_sighash_all`, the lock guarding my week 1 cell. So there's six steps:
1. Load own args (one syscall) -> the 20-byte blake160 owner key fingerprint (the requirement).
2. Load the witness (one syscall) -> the 65-byte signature in the lock field (the proof)
3. Assemble the signed message (many syscalls): blank the signature slot to zeros, then hash the whole tx. 
4. Recover the public key (computation) -> from signature + message via `secp256k1`.
5. Reduce and compare (computation) -> `blake160 `the recovered key, compare to the `args` fingerprint.
6. Exit on the comparison -> match returns 0, mismatch returns nonzero.


## Practical Progress
- [x] Walked through the Script data structure by hand
- [x] Followed the carrot type script's tutorial
- [x] Wrote my own byte-equality toy lock from scratch in Rust: `load_script` for args, `load_witness_args` for the witness lock field, `to_opt()` to handle the absent case, `raw_data()` byte comparison, custom `WrongSecret` error code
- [ ] Write the two unit tests (success + failure with assert on error code 5) -> in progress
- [ ] Add signature verification to turn the toy lock into a real lock -> week 4