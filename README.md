# Artist_NFT_Ink

## Introduction:

The PSP34 NFT contract is a smart contract written in Rust using the ink! framework. The contract is designed to allow the creation, ownership, transfer, and sale of non-fungible tokens (NFTs) in a decentralized manner.


## Features:


The PSP34 NFT contract has the following features:
1. Minting: The contract allows the creation of new NFTs by minting them. A new NFT is created by providing a unique token ID, a price, and an artist ID.

2. Ownership: Each NFT is owned by an account, which can be transferred to another account.

3. For sale: An NFT can be put up for sale by setting a price. Once an NFT is for sale, anyone can buy it by paying the set price.

4. Artists: Each NFT is associated with an artist, who is identified by a unique artist ID. Artists can set their name and account ID, which can be used to verify their ownership of the NFTs associated with them.

5. Decentralization: The PSP34 NFT contract is a fully decentralized smart contract that runs on a blockchain. This means that it cannot be controlled or censored by any central authority.



## Usage:

The PSP34 NFT contract can be used in a number of ways, including:


1. Creating new NFTs: Anyone can create a new NFT by calling the mint function and providing a unique token ID, a price, and an artist ID.

2. Transferring ownership: The owner of an NFT can transfer ownership to another account by calling the transfer function and providing the ID of the NFT and the account to transfer ownership to.

3. Putting NFTs up for sale: The owner of an NFT can put it up for sale by calling the set_price function and providing the ID of the NFT and the price they want to sell it for.

4. Buying NFTs: Anyone can buy an NFT that is for sale by calling the buy function and providing the ID of the NFT they want to buy. The price of the NFT will be deducted from the buyer's account, and the ownership of the NFT will be transferred to them.

5. Associating NFTs with artists: The owner of an NFT can associate it with an artist by calling the set_token_artist function and providing the ID of the NFT and the ID of the artist they want to associate it with.

6. Setting artist details: An artist can set their name and account ID by calling the set_artist function and providing their artist ID, name, and account ID.

7. Retrieving NFT details: Anyone can retrieve the details of an NFT by calling the get_token function and providing the ID of the NFT.


## Conclusion:
The PSP34 NFT contract is a powerful tool for creating, owning, and trading non-fungible tokens. It allows anyone to create new NFTs, transfer ownership, put them up for sale, and buy them. The contract is fully decentralized, meaning that it cannot be controlled or censored by any central authority.

