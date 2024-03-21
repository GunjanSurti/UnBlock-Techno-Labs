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
  getTokenMetadata,
  TYPE_SIZE,
  LENGTH_SIZE,
  getOrCreateAssociatedTokenAccount,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  createInitializeInstruction,
  createUpdateFieldInstruction,
  createRemoveKeyInstruction,
  pack,
  TokenMetadata,
} from "@solana/spl-token-metadata";

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
  uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json",
  // uri: "",
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

//Instruction to initialize Metadata Account
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
  { commitment: "confirmed" }
);

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

console.log("----------------------------------------------------------");
// getting metadata from blockchain (Working)

const chainMetadata = await getTokenMetadata(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Metadata : ", chainMetadata);
console.log("------------------------Meatdata-------------------------------");

//////////////////// After changing metadata ////////////////////

console.log("---------------------New Meatdata------------------------------");

const newMetaData: TokenMetadata = {
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
// Instruction to update metadata, adding custom field
const newMetadataName = createUpdateFieldInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: tokenAddress, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  field: "name", // key
  value: "newToken", // value
});

const newMetadataSymbol = createUpdateFieldInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: tokenAddress, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  field: "symbol", // key
  value: "NTK", // value
});
// to update uri we should already have some value || if null then we cannot update
const newMetadataUri = createUpdateFieldInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: tokenAddress, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  field: "uri", // key
  value: "New uri", // value
});

// this will update "additionalMetadata" and add new field and value
const newMetadataAdditionalMetadata = createUpdateFieldInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: tokenAddress, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  field: "Hello", // key
  value: "World!!", // value
});

const newTransaction = new Transaction().add(
  newMetadataName,
  newMetadataSymbol,
  newMetadataUri,
  newMetadataAdditionalMetadata
);

// Send transaction to change existing metadata
const sig2 = await sendAndConfirmTransaction(
  connection,
  newTransaction,
  [payer], // Signers
  { commitment: "confirmed" }
);
// console.log(sig2);

const newMetadata = await getTokenMetadata(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Updated Metadata : ", newMetadata);

console.log(
  "------------------------Removing Custom Field---------------------------"
);

const tx = new Transaction().add(
  createRemoveKeyInstruction({
    programId: TOKEN_2022_PROGRAM_ID,
    metadata: tokenAddress,
    updateAuthority: payer.publicKey,
    key: "Surat",
    idempotent: true, // If false the operation will fail if the field does not exist in the metadata
  })
);
const sig3 = await sendAndConfirmTransaction(connection, tx, [payer], {
  commitment: "confirmed",
});
const newMetadata23 = await getTokenMetadata(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Updated Metadata : ", newMetadata23);
console.log("---------------------------------------------------");
