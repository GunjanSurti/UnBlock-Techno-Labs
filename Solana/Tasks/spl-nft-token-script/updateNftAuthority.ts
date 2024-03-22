const fs = require("fs");
const bs58 = require("bs58");

import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import {
  AccountLayout,
  AuthorityType,
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
  decodeSetAuthorityInstruction,
} from "@solana/spl-token";
import {
  createInitializeInstruction,
  createUpdateFieldInstruction,
  createRemoveKeyInstruction,
  pack,
  TokenMetadata,
} from "@solana/spl-token-metadata";

// creating connecting

const connection = new Connection(clusterApiUrl("devnet"), {
  commitment: "confirmed",
});

// Read secret key from JSON file
const secretData = JSON.parse(fs.readFileSync("./secret.json"));
const secretKeyHex = secretData.secretKey;
const payer = Keypair.fromSecretKey(new Uint8Array(bs58.decode(secretKeyHex)));
console.log("payer account:", payer.publicKey.toBase58());

// Read secret key from JSON file => new authority
const newSecretData = JSON.parse(fs.readFileSync("./newAuthority.json"));
const newSecretKeyHex = newSecretData.secretKey;
const newPayer = Keypair.fromSecretKey(
  new Uint8Array(bs58.decode(newSecretKeyHex))
);
console.log("New payer account:", newPayer.publicKey.toBase58());

// token Address : address which will mint new tokens
const mintKeypair = Keypair.generate();
export const tokenAddress = mintKeypair.publicKey;
console.log("minting account:", tokenAddress.toBase58());

// set custom SPL token
const tokenName = "MulAuth";
const tokenSymbol = "MPA";

// Metadata Created
const metaData: TokenMetadata = {
  // The authority that can sign to update the metadata
  updateAuthority: payer.publicKey,
  mint: tokenAddress,
  name: tokenName,
  symbol: tokenSymbol,
  // The URI pointing to richer metadata
  uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json",
  //   uri: "",

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
  tokenAddress, // token mint account
  0, // decimals
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
// console.log("signature of create Account ... Update metadata", sig);
console.log("----------------------------------------------------------");

// creating or getting associated token
const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
  connection, // Connection
  payer, //Payer of the transaction and initialization fees
  tokenAddress, // int associated with the account to set or verify
  payer.publicKey, // Owner of the account to set or verify
  false, // Allow the owner account to be a PDA (Program Derived Address)
  "confirmed",
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

// creating or getting associated token for newAuthority
const newAuthorityATA = await getOrCreateAssociatedTokenAccount(
  connection, // Connection
  payer, //Payer of the transaction and initialization fees
  tokenAddress, // int associated with the account to set or verify
  newPayer.publicKey, // Owner of the account to set or verify
  false, // Allow the owner account to be a PDA (Program Derived Address)
  "confirmed",
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

console.log("Associated Account", associatedTokenAccount.address.toBase58());

console.log(
  " New Authority Associated Account",
  newAuthorityATA.address.toBase58()
);

console.log(
  "-------------------------minted by old authority--------------------------"
);

// // minted by old authority
const mintedTokenConfirmedSignature = await mintTo(
  connection,
  payer, // Signer
  tokenAddress, // token address
  associatedTokenAccount.address, // destination address
  payer, // mint Authority
  1, // 100 token because decimals for the mint are set to 9
  [payer],
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID
);

console.log("Minted Confirmed Signatuer :", mintedTokenConfirmedSignature);

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

console.log("------------------changing authority-----------------");

const newAuthority = await setAuthority(
  connection, // connection
  newPayer, // payer
  tokenAddress, //  Address of the account to set
  payer, // Current authority of the specified type
  AuthorityType.MintTokens, //Type of authority to set
  newPayer.publicKey, // new authority public key
  [payer],
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID
);
console.log("Tx : ", newAuthority);

// again minting to check if minting is possible without authority
const MintedByNewAuthority = await mintTo(
  connection,
  newPayer, // Signer
  tokenAddress, // token address
  newAuthorityATA.address, // destination address
  newPayer, // mint Authority
  1, // 100 token because decimals for the mint are set to 9
  [newPayer],
  { commitment: "confirmed" },
  TOKEN_2022_PROGRAM_ID
);

console.log("-------------------------------------------------");

///// getting mint info
const afterMintInfo = await getMint(
  connection,
  tokenAddress,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Total Supply", afterMintInfo.supply);

const newAuthorityAccountInfo = await getAccount(
  connection,
  newAuthorityATA.address,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);

console.log("Balance of asscoiated account: ", newAuthorityAccountInfo.amount);
