# Week One Report

Name: Cyndie Kamau

Week Ending: 29/05/2026

## Courses Done

* CKB Academy Module 1 -> Theoretical knowledge on the structure of a cell, structure of a transaction
* CKB Academy Module 2 -> Manually created and sent my first transaction on testnet. [View on explorer](https://testnet.explorer.nervos.org/transaction/0x2195ea84348381ff64192ac6c0173ad763ecaeeeeea99833f5258d6898b81467)

## Key Learnings

At first I didn't understand how CKB worked since I came in with the mental model of EVM. So I had to go back and refresh on UTXO model anchoring CKB.

**UTXO model** clicked when I thought of it as dollar bills instead of balances. 

I have a $20 bill -> Go to the supermaket -> Buy goods worth $5 -> Give out my $20 bill to cashier -> Get my change back in chunks $5, $10 -> My original $20 bill is "destroyed" and my new minted balance is the two $15 chunks.

With this mental model in place, then I understood where CKB and Bitcoin diverged:

Bitcoin's UTXO -> only holds money.

CKB cells -> holds money + an arbitrary data field which can be passive (token amount, text), or active (compiled script binary)

So code stored in a cell -> Smart contract.

Essentially CKB is an enrichment layer of UTXO, and at the core of it is just a giant database full of cells. 

Spent cells -> Dead

Unspent cells -> Live

So the state of the chain -> Live cells

So in summary: Every tx consumes live cells as input (killing them) then creates new cells as outputs. No matter how complex an application is, at the core is minting and destroying cells.

Also the **cell capacity:** In CKB the capacity number is the value the cell holds + bytes of onchain storage it occupies.

So 1 CKB -> 1 byte of storage.

## Practical Progress

- [x] Setup CKB locally
- [x] Deployed a smart contract
- [x] Did my first transaction on testnet

My first tx attempt I got an error:

```
PoolRejectedTransactionByMinFeeRate: The min fee rate is 1000 shannons/KW,
requiring a transaction fee of at least 414 shannons, but the fee provided is only 0
```
**Debugging:** I was spending a single 100CKB and creating a single 100CKB output, so no room for miner's fee.

**Fix:** Lower the output capacity from `0x2540be400` to ` 0x2540be262` leaving a gap of around 414 shannons, then re-signed the witness with the new signature hash.

## Personal Footnotes

To remember the structure of a transaction:

```
{
  "version": "0x0",   --- format version
  "headerDeps": [],   --- For block headers
  "cellDeps": [       --- References to cells holding script code (Omnilock, secp256k1_blake160)
    {
      "outPoint": {
        "txHash": "0xec18bf0d857c981c3d1f4e17999b9b90c484b303378e94de1a57b0872f5d4602", --- location of Omnilock code cell 
        "index": "0x0"
      },
      "depType": "code"     --- The referenced cell holds the script binary directly
    },
    {
      "outPoint": {
        "txHash": "0xf8de3bb47d055cdf460d93a2a6e1b05f7432f9777c8c474abf4eec1d4aee5d37",  --- location of secp256k1_blake160 bundle
        "index": "0x0"
      },
      "depType": "depGroup"   --- The referenced cell is a bundle pointing to several code cells
    }
  ],
  "inputs": [   --- Cell being consumed and destroyed (100CKB)
    {
      "since": "0x0",
      "previousOutput": {
        "txHash": "0x2827129b1e1e26255542458e46dbe1d6bfb3e0392900a975d67a7513ab2a9cb8",
        "index": "0x0"
      }
    }
  ],
  "outputs": [    --- New cell created locked to my own key
    {
      "capacity": "0x2540be262",
      "lock": {
        "codeHash": "0xf329effd1c475a2978453c8600e1eaf0bc2087ee093c3ee64cc96ec6847752cb", --- tag that identifies the Omnilock code
        "hashType": "type",
        "args": "0x017edb7555a71a4ef205ec24b1637bb77c07e615b600"
      }
    }
  ],
  "outputsData": [  --- New cell's data field, empty since its just money
    "0x"
  ],
  "witnesses": [   --- Signature over the whole tx proving I'm authorized to spend the input
    "0x690000001000000069000000690000005500000055000000100000005500000055000000410000003d1df97b4d32b9f9bf6c3ca3b4433c4036b1c7d9cdeb9526eea1ab1a2f8285430863ee3f0c73723d753dddfb85e7c785ad8fb6e4ff9475dafa41c39b5d59c13901"
  ]
}
```





