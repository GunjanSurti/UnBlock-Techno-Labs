const fs = require("fs");
const bs58 = require("bs58");
// const splToken = require("@solana/spl-token");
// const {
//   createMint,
//   getMint,
//   getOrCreateAssociatedTokenAccount,
//   getAccount,
//   mintTo,
//   freezeAccount /* imported */,
// } = require("@solana/spl-token");
// const { Keypair, Connection, LAMPORTS_PER_SOL } = require("@solana/web3.js");

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

// mint account
const mintKeypair = Keypair.generate();
const mint = mintKeypair.publicKey;
console.log("minting account:", mint.toBase58());

// set SPL token
const tokenName = "Token Unblock";
const tokenSymbol = "UBT";

const metaData: TokenMetadata = {
  // The authority that can sign to update the metadata
  updateAuthority: payer.publicKey,
  mint: mint,
  name: tokenName,
  symbol: tokenSymbol,
  // The URI pointing to richer metadata
  uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json",

  additionalMetadata: [["description", "Only Possible On Solana"]],
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

const createAccountIx = SystemProgram.createAccount({
  fromPubkey: payer.publicKey, // payer of transaction
  newAccountPubkey: mint, // account where token will receive
  space: mintSpace, // we need exact amount of space for mint itself when you are creating initial account
  lamports: lamports,
  programId: TOKEN_2022_PROGRAM_ID, // predefined program by solana foundation like "SystemProgram"
});

// initializing metadata
const initializeMetadataPointerIx = createInitializeMetadataPointerInstruction(
  mint, // account to mint tokens
  payer.publicKey, // update authority
  mint, // actual address which will hold metadata itself || "mint account" itself can be metadata program || can be null
  TOKEN_2022_PROGRAM_ID
);

const initializeMintIx = createInitializeMintInstruction(
  mint, // mint to this account
  9, // decimals
  payer.publicKey, // payer acc
  null,
  TOKEN_2022_PROGRAM_ID
);

//Instruction to initialize Metadata Account data
const initializeMetadataIx = createInitializeInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: mint, // Account address that holds the metadata
  updateAuthority: payer.publicKey, // Authority that can update the metadata
  mint: mint, // Mint Account address
  mintAuthority: payer.publicKey, // Designated Mint Authority
  name: metaData.name,
  symbol: metaData.symbol,
  uri: metaData.uri,
});

// Instruction to update metadata, adding custom field
const updateMetadataField = createUpdateFieldInstruction({
  programId: TOKEN_2022_PROGRAM_ID, // Token Extension Program as Metadata Program
  metadata: mint, // Account address that holds the metadata
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
  [payer, mintKeypair] // Signers
);
console.log("sig:", sig);

// getting metadata from blockchain
const chainMetadata = await getTokenMetadata(connection, mint);

console.log(chainMetadata);
