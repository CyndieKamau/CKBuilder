# Week Two Report

Name: Cyndie Kamau

Week Ending: 05/06/2026

## Courses Done

* [Omnilock Script Documentation](https://docs.nervos.org/docs/ecosystem-scripts/omnilock#how-omnilock-works) -> How Omnilock works: auth structure, flag table, Omnilock arg modes, witness structure.
* [CKB Whitepaper](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0002-ckb/0002-ckb.md) -> Went through the CKB Whitepaper.
* [RFC 0042](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0042-omnilock/0042-omnilock.md) -> Omnilock RFC for args layout and authentication reference.

## Key Learnings

Going through the whitepaper helped me reinforce the mental model acquired in week 1.

So Nervos has two layers: 

**Layer 1 (CKB) ->** The verification layer that is a trust root and smart custodian. Its work it to verify and store, hence the name Common Knowledge Base.

**Layer 2 ->** The generation layer for processing most transactions and generating new states.

Transactions are only submitted to CKB when participants want them settled. 

So state generation -> offchain

State Verification -> CKB VM

This brings about determinism, since users can generate new states client-side and confirm before broadcasting.

So the outcome can either be the tx passes verification and the new state is accepted, or it fails and nothing happens.

That's why CKB's economic model -> State, not computation. So 1CKB -> 1 byte.

**The e2e workflow:**

**Offchain** -> Query indexer to find live cells -> Generator (wallet, user, dapp) constructs tx -> Sign (Serialize, hash, sign hash with private key, wrap signature with witness args)

**Onchain** -> Broadcast signed tx to any node -> Node does a checklist (double spend check etc) -> Script execution happens (load the lock script's code via CellDeps, run via CKB VM, run type script too) -> Here's where the tx will pass or fail -> Miner packages the passed tx to a block, broadcasts the block -> Every full node verifies the block and updates its live cells set.


## Key mental model shift for smart contracts

**In EVM ->** A smart contract has functions to call, storage to mutate, code that runs to produce a new state.

**In CKB ->** A script has no operations, so no storage of its own, cannot be called. What we have is:

**Lock script ->** Enforce ownership, who can consume a cell. Run only when a cell is input. Like Omnilock.

**Type script ->** Enforce meaning, what a cell is and which transitions are valid, and run both on inputs and outputs because transition rules relate old state to new.

An application's executing logic lives offchain in the generator (wallet etc), so the scripts only hold the laws it must satisfy.

## Practical Progress

- [x] Decoded my week 1 lock args by hand: flag / auth content / Omnilock flags
- [x] Decoded my week 1 witness byte-by-byte as a Molecule structure (footnotes)
- [ ] Writing my own lock script -> moved to week 3, now with the right mental model (a verifier, not a doer)



