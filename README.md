We are intermittently experiencing this error:

> Program log: Burning cNFT
> Program log: Burning: 8CSxs2dBxuH3L1H8UyezqJeAKNqnMA2AXdBu4VqM3YVt
> Invoking Bubblegum
  > Program log: Instruction: Burn
  > Invoking Unknown (cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK) Program
    > Program log: Instruction: ReplaceLeaf
    > Program log: Attempting to fill in proof
    > Program consumption: 104282 units remaining
    > Program log: Active Index: 42
    > Program log: Rightmost Index: 5908689
    > Program log: Buffer Size: 64
    > Program log: Leaf Index: 5889207
    > Program log: Failed to find root in change log -> replaying full buffer
    > Program log: Fast-forwarding proof, starting index 43
    > Program consumption: 97977 units remaining
    > Program consumption: 93013 units remaining
    > Program log: Error using concurrent merkle tree: Invalid root recomputed from proof
    > Program log: AnchorError thrown in programs/account-compression/src/lib.rs:273. Error Code: ConcurrentMerkleTreeError. Error Number: 6001. Error Message: Concurrent merkle tree error.
    > Program cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK  consumed 27901 of 112187 compute units
    > Program returned error: custom program error: 0x1771
  > Program Bubblegum  consumed 70055 of 154341 compute units
> Program GwR3T5wAAWRCCNyjCs2g9aUM7qAtwNBsn2Z515oGTi7i  consumed 115564 of 199850 compute units

We believe it has to due with the low buffer size of 64 set in the tree.  We're open to suggestions on combating this problem.  We were able to burn this cnft in a successive call to the program.
