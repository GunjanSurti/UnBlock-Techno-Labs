const fs = require("fs");
const bs58 = require("bs58");

import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  createInitializeMintInstruction,
  getMintLen,
  createInitializeMetadataPointerInstruction,
  getMint,
  getMetadataPointerState,
  getTokenMetadata,
  TYPE_SIZE,
  LENGTH_SIZE,
  createAccount,
  getOrCreateAssociatedTokenAccount,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAccount,
  mintTo,
  createSetAuthorityInstruction,
  closeAccount,
  setAuthority,
} from "@solana/spl-token";
import {
  createInitializeInstruction,
  createUpdateFieldInstruction,
  createRemoveKeyInstruction,
  pack,
  TokenMetadata,
} from "@solana/spl-token-metadata";

import { AccountLayout } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { AuthorityType } from "@solana/spl-token";
// creating connecting

const connection = new Connection(clusterApiUrl("devnet"));

// Read secret key from JSON file
const secretData = JSON.parse(fs.readFileSync("./secret.json"));
const secretKeyHex = secretData.secretKey;
const payer = Keypair.fromSecretKey(new Uint8Array(bs58.decode(secretKeyHex)));
console.log("payer account:", payer.publicKey.toBase58());

// token Address : address which will mint new tokens
const mintKeypair = Keypair.generate();
export const tokenAddress = mintKeypair.publicKey;
console.log("minting account:", tokenAddress.toBase58());

// set custom SPL token
const tokenName = "Token Unblock";
const tokenSymbol = "UBT";

// Metadata Created
const metaData: TokenMetadata = {
  // The authority that can sign to update the metadata
  updateAuthority: payer.publicKey,
  mint: tokenAddress,
  name: tokenName,
  symbol: tokenSymbol,
  // The URI pointing to richer metadata
  //   uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json",
  uri: "",

  additionalMetadata: [["Surat", "Uttran"]],
};
// we need calculate space to store on solana blockchain
const mintSpace = getMintLen([ExtensionType.MetadataPointer]);

// we need to calculate how much space our metadata needs on solana
// Size of MetadataExtension 2 bytes for type, 2 bytes for length both are constant values  provided by spl-token package
const metadataSpace = TYPE_SIZE + LENGTH_SIZE + pack(metaData).length;

// asking blockchain how much lamports we need for rent exempt
const lamports = await connection.getMinimumBalanceForRentExemption(
  mintSpace + metadataSpace
);

// creating account
const createAccountIx = SystemProgram.createAccount({
  fromPubkey: payer.publicKey, // payer of transaction
  newAccountPubkey: tokenAddress, // account where token will receive
  space: mintSpace, // we need exact amount of space for mint itself when you are creating initial account
  lamports: lamports,
  programId: TOKEN_2022_PROGRAM_ID, // predefined program by solana foundation like "SystemProgram"
});

// initializing metadata
const initializeMetadataPointerIx = createInitializeMetadataPointerInstruction(
  tokenAddress, // account to mint tokens
  payer.publicKey, // update authority
  tokenAddress, // actual address which will hold metadata itself || "mint account" itself can be metadata program || can be null
  TOKEN_2022_PROGRAM_ID
);

// creating Initialize Mint Instruction
const initializeMintIx = createInitializeMintInstruction(
  tokenAddress, // mint to this account
  9, // decimals
  payer.publicKey, // payer acc
  payer.publicKey, // freezeAuthority
  TOKEN_2022_PROGRAM_ID
);

//Instruction to initialize Metadata Account data
const initializeMetadataIx = createInitializeInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: tokenAddress, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  mint: tokenAddress, // Mint Account address
  mintAuthority: payer.publicKey, // Designated Mint Authority
  name: metaData.name,
  symbol: metaData.symbol,
  uri: metaData.uri,
});

// Instruction to update metadata, adding custom field
const updateMetadataField = createUpdateFieldInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: tokenAddress, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  field: metaData.additionalMetadata[0][0], // key
  value: metaData.additionalMetadata[0][1], // value
});

// Add instructions to new transaction
// order is important here as some token need metadata to be inilized before mint
const transaction = new Transaction().add(
  createAccountIx,
  initializeMetadataPointerIx,
  // note: the above instructions are required before initializing the mint
  initializeMintIx,
  initializeMetadataIx,
  updateMetadataField
);

// Send transaction
const sig = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, mintKeypair], // Signers
  undefined
);
// console.log("sig:", sig);
console.log("----------------------------------------------------------");

// creating or getting associated token
const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  payer,
  tokenAddress,
  payer.publicKey,
  false,
  "confirmed",
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

console.log("Associated Account", associatedTokenAccount.address.toBase58());

// getting metadata from blockchain (Working)
console.log("---------------------------------------------------");

const chainMetadata = await getTokenMetadata(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Metadata : ", chainMetadata);
console.log("---------------------------------------------------");

// console.log("Associated Account without 58---", associatedTokenAccount.address);

// here we will mint 100 tokens to "associatedTokenAccount"

const mintedTokenConfirmedSignature = await mintTo(
  connection,
  payer, // Signer
  tokenAddress, // token address
  associatedTokenAccount.address, // destination address
  payer, // mint Authority
  100000000000, // 100 token because decimals for the mint are set to 9
  [payer],
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID
);

// console.log("Minted Confirmed Signatuer :", mintedTokenConfirmedSignature);

// checking total supply of token
const tokenMintInfo = await getMint(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Total Supply", tokenMintInfo.supply);

// checking balance of associated account
const tokenAccountInfo = await getAccount(
  connection,
  associatedTokenAccount.address,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Balance of Associated account ", tokenAccountInfo.amount);

/*******************Way to show all tokens present in wallet ****************/

// const tokenAccounts = await connection.getTokenAccountsByOwner(
//   // my public key
//   new PublicKey("BZeBnYJ1t42cYudj55xB6TMYUuMiwF8xm3138W3wHpUX"),
//   {
//     programId: TOKEN_2022_PROGRAM_ID,
//   },
//   { commitment: "confirmed" }
// );

// console.log("Token                                         Balance");
// console.log("------------------------------------------------------------");
// tokenAccounts.value.forEach((tokenAccount) => {
//   const accountData = AccountLayout.decode(tokenAccount.account.data);
//   console.log(`${new PublicKey(accountData.mint)}   ${accountData.amount}`);
// });

console.log(
  "-------------------------------------------FreezeAccount----------------------"
);

// Disable minting function

console.log("------------------changing authority-----------------");

const stopMinting = await setAuthority(
  connection, // connection
  payer, // payer
  tokenAddress, //  Address of the account to disable
  payer,
  AuthorityType.MintTokens, //Type of authority to set
  null, // new authority
  [payer],
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID
);

console.log("Tx : ", stopMinting);

// again minting to check if minting is possible without authority
const FreezMintedConfirmedSignature = await mintTo(
  connection,
  payer, // Signer
  tokenAddress, // token address
  associatedTokenAccount.address, // destination address
  payer, // mint Authority
  100000000000, // 100 token because decimals for the mint are set to 9
  [payer],
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID
);

console.log("-------------------------------------------------");

// console.log(
//   "after freezing Minted Confirmed Signatuer :",
//   FreezMintedConfirmedSignature
// );

///// getting mint info
const afterMintInfo = await getMint(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Total Supply", afterMintInfo.supply);

const AfterTokenAccountInfo = await getAccount(
  connection,
  associatedTokenAccount.address,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Balance of asscoiated account: ", AfterTokenAccountInfo.amount);

/*****************Additional **********************/
///// close token minting account not asscoiated account
// will only close if balance is "zero"

// const txSig = await closeAccount(
//   connection, // connection
//   payer, // payer
//   associatedTokenAccount.address, // Account to colse
//   payer.publicKey, // Account to receive the remaining balance of the closed account
//   payer, //Authority which is allowed to close the account
//   [payer],
//   { commitment: "confirmed" },
//   TOKEN_2022_PROGRAM_ID
// );

/**************To create new public key ***************************/
// const pub = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v')
