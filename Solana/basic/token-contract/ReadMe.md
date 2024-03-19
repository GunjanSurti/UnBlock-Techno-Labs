# Quick Background Recap

1. Solana uses the concept of Accounts t okeep data on the network that's tied to your public key

2. Tokens/Nfts are accounts that is owned by an authority.

3. To be able to hold Token/Nft, we need to create Associated Token Account (ATA) that our wallet owns which allows us to hold Token / Nft.

## STEPS TO CREATE TOKEN

### Create token commands

token-id = DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM means address of token

associated account address = AnKTMeGShhWm1NFJmFJ3wmnL1MrCwojZiLYWse7zZbvd

which will be used to store tokens

- spl-token create-token

  ```sh
  - Address: DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM "this is our token address"
  - Decimals: 9

  ```

- spl-token create-account [token id]
  
this will an account that will be associated with our token address

```sh
Creating account AnKTMeGShhWm1NFJmFJ3wmnL1MrCwojZiLYWse7zZbvd

Signature: uPcU9jertareAHRCoHuJdnuvL9XDrCaz9hc2ZzvneXXu8GdfdsEkU3xGGgTEE4q7kxVgE2YPEdHNuFCWgYRYxFh
```

new account AnKTMeGShhWm1NFJmFJ3wmnL1MrCwojZiLYWse7zZbvd => this will be used to store token and associated with our public key

- spl-token mint [token id] 100

```sh
spl-token mint DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM 100
Minting 100 tokens
  Token: DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM
  Recipient: AnKTMeGShhWm1NFJmFJ3wmnL1MrCwojZiLYWse7zZbvd

Signature: 9mjxryntdXtALZjf32Sg9joZ5hu8uPs15YPfFLHhnWF4SsvXpmUjgYjdyqmGCJzMkPHZqZFaUGpBPnM7MHMTVbR
```

- spl-token supply [token id]

this will give total supply of tokens

```sh
spl-token supply DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM 
```

- spl-token balance [token id]
- spl-token accounts

```sh
Token                                         Balance
-----------------------------------------------------
B9HU5QkKaB7sdCaNtYXsNghYvobW1bHP532ajxi6bU32  0
BkErVZh2LuiBjPNKcNd3eodEFJh5XUiLxpNuPj5L5QqZ  0
DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM  200 <- our token
EeX9VNFk3uo7cvzcLpUr6Ee5Sswsdw8Jy7Gh9jzaaKz1  100 
```

- spl-token transfer --fund-recipient [token id] 10 [wallet address]

this will create new account associated with DjKrFDXXR1vUCRTevckU9PHkW9PLKMHoSAFsZeiFbHyM token address     for my [wallet address] (phantom wallet)

and will transfer ammount to it

- spl-token burn [account id] 10

this will reduce the supply from particular associated account and can only be done by particular owner of ATA account address

 ------------------------------------

- Create NFT commands:

- spl-token create-token --decimals 0
- spl-token create-account [token id]
- spl-token mint [token id] 1
- spl-token authorize [token id] mint --disable

 ------------------------------------

- Send Wrapped Sol commands:
- spl-token wrap 1
- spl-token unwrap [account id]
- spl-token transfer --fund-recipient So11111111111111111111111111111111111111112 10 [wallet address]

 ------------------------------------
