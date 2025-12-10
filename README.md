Merkle Tree Implementaion on Solana

Storing thousands of accounts on solana require's paying huge amount of rent solana

for 1000 accounts one need pay around 0.2 SOL in rent

'''bash
> 1000 * 32
32000
> solana rent 32000
Rent-exempt minimum: 0.22361088 SOL
'''

one better way of solving this problem is to use merkle tree. 
Instead of storing of all the accounts we just store 32 byte root hash
Then we can prove the inclusion of address buy computing the hash of siblings from ground-up
