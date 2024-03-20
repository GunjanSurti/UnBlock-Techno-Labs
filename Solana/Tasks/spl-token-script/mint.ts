const fs = require("fs");
const bs58 = require("bs58");
const { mintTo, TOKEN_2022_PROGRAM_ID } = require("@solana/spl-token");
import { tokenAddress } from "./demoToken.ts";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
// Read secret key from JSON file
const secretData = JSON.parse(fs.readFileSync("./secret.json"));
const secretKeyHex = secretData.secretKey;
const payer = Keypair.fromSecretKey(new Uint8Array(bs58.decode(secretKeyHex)));

// const tokenMint = "8KS5gBy5zymhAEhY5Jd9LXA3Gqpv16wgbNayBj2nUCeC";
const connection = new Connection(clusterApiUrl("devnet"));
console.log("connected-----------------------------------");

const mintedConfirmedSignature = await mintTo(
  connection,
  payer, // Signer
  tokenAddress, // token address
  payer.publicKey, // destination address
  payer, // mint Authority
  100000000000, // 100 token because decimals for the mint are set to 9
  [payer],
  {},
  TOKEN_2022_PROGRAM_ID
);


