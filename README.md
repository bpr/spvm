## SPVM-1
SPVM-1 is a verison 1 implementation of the Spire PoC Virtual Machine in Solidity.
SPVM is a programming assignment, translating some of the essentials from Solidity to Rust.
## The Task:
The task is to
1. Check out solidity implementation of spvm [here](https://github.com/spire-labs/spvm-1?tab=readme-ov-file).
2. Write equivalent rust code
3. Can omit the following structures and methods
    a. `struct Block`
    b. `function proposeBlock` 
    c. `function setElectionContract`
    d. `function setSlashingContract`
4. The primary functionality we are interested is for the enforcer, proposer to be able to make local state changes to accounts and verify new incoming transactions against it The task consists of three sub-tasks:
5. While we would need some sort of persistent storage for production, for the purposes of this exercise feel free to store the state in memory or use a simple ORM
6. Write examples for usage of the following  functions
    * `get_balance`
    * `set_balance`
    * `execute_raw_transaction`
    * `validate_signature`
    * `execute_transaction`

## The Design
    1. [Design requirements](https://www.notion.so/spirelabs/PoC-Custom-VM-Design-Requirements-3a6ae1a72e2e4fd98460985bae46d404) 
    2. [Design specification](https://www.notion.so/spirelabs/SPVM-1-Design-Spec-a0d758b19f744279a7d83ee0aa6964e3)
## My Approach
* Map Solidity `contract` to Rust record, by hoistng type definitions to the top level, and letting function defs be methods of an impl.
* Map Solidity types to Rust types. This isn't perfect, as an example, an `Address` is mapped to a Rust array, but there are tricks that are used in `validate_signature` to see if an `Address` corresponds to a smart contract.

