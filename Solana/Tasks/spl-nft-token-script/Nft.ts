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
