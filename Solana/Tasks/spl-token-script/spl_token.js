const fs = require("fs");
const bs58 = require("bs58");
const splToken = require("@solana/spl-token");
const {
  createMint,
  getMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
  freezeAccount /* imported */,
} = require("@solana/spl-token");
const { Keypair, Connection, LAMPORTS_PER_SOL } = require("@solana/web3.js");

///////////////////////////////////
const { TokenMetadata } = require("@solana/spl-token-metadata");

async function deployToken() {
  const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
  );

  // Read secret key from JSON file
  const secretData = JSON.parse(fs.readFileSync("./secret.json"));
  const secretKeyHex = secretData.secretKey;
  const payer = Keypair.fromSecretKey(
    new Uint8Array(bs58.decode(secretKeyHex))
  );

  // print wallet balance
  let balance = await connection.getBalance(payer.publicKey);
  console.log(
    `Wallet ${payer.publicKey} Balance is ${balance / LAMPORTS_PER_SOL} SOL`
  );

  // set SPL token
  const tokenName = "Token Unblock";
  const tokenSymbol = "UBT";
  // const initialSupply = 1000000; // Initial supply of tokens

  // Generate a new token mint
  const tokenMint = await createMint(
    connection,
    payer,
    payer.publicKey, // Mint authority
    payer.publicKey, // freeze authority
    9 // Decimals, default to 0
  );
  console.log("-----------------");
  console.log("token Address: ", tokenMint.toBase58());
  //////////////////////////////////////////////
  const metadata = {
    // The authority that can sign to update the metadata
    updateAuthority: payer,
    // The associated mint, used to counter spoofing to be sure that metadata belongs to a particular mint
    mint: payer,
    // The longer name of the token
    name: tokenName,
    // The shortened symbol for the token
    symbol: tokenSymbol,
    // The URI pointing to richer metadata
    uri: "",
    // Any additional metadata about the token as key-value pairs
    additionalMetadata: [["key", "value"]],
  };

  // we need calculate space to store on solana blockchain
  const mintSpace = splToken.getMintLen([
    splToken.ExtensionType.MetadataPointer,
  ]);

  // we need to calculate how much space our metadata needs on solana
  // const metadataSpace = splToken.TYPE_SIZE + splToken.LENGTH_SIZE; // THIS MAKES it deserialze
  const metadataSpace =
    splToken.TYPE_SIZE + splToken.LENGTH_SIZE + pack(metadata).length;

    // asking blockchain how much lamports we need for rent exempt 
  const lamports = await connection.getMinimumBalanceForRentExemption(
    mintSpace + metadataSpace
  );
  ///////////////////////////////////////

  // await tokenMetadataInitialize(

  // )
  const mintInfo = await getMint(connection, tokenMint);

  console.log(mintInfo.supply); // 0

  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    tokenMint,
    payer.publicKey
  );

  console.log("My Account: ", tokenAccount.address.toBase58());

  const tokenAccountInfo = await getAccount(connection, tokenAccount.address);

  console.log(tokenAccountInfo.amount);

  /**
   * Mint tokens to an account
   *
   * @param connection     Connection to use
   * @param payer          Payer of the transaction fees
   * @param mint           Mint for the account
   * @param destination    Address of the account to mint to
   * @param authority      Minting authority
   * @param amount         Amount to mint
   * @param multiSigners   Signing accounts if `authority` is a multisig
   * @param confirmOptions Options for confirming the transaction
   * @param programId      SPL Token program account
   *
   * @return Signature of the confirmed transaction
   */
  await mintTo(
    connection,
    payer,
    tokenMint,
    tokenAccount.address,
    payer, // mint Authority
    100000000000 // because decimals for the mint are set to 9
  );
  const mintInfo1 = await getMint(connection, tokenMint);

  console.log("Post Mint totalSupply: ", mintInfo1.supply);

  const tokenAccountInfo1 = await getAccount(connection, tokenAccount.address);

  console.log("Post Mint my Token Amount: ", tokenAccountInfo1.amount);

  // Add logic to freeze mint

  // await freezeAccount(
  //   connection,
  //   payer,
  //   tokenAccount.address,
  //   tokenMint,
  //   payer
  // );

  // again mint this should throw error
  await mintTo(
    connection,
    payer,
    tokenMint,
    tokenAccount.address,
    payer, // mint Authority
    100000000000 // because decimals for the mint are set to 9
  );
  console.log("Post Mint my Token Amount: ", tokenAccountInfo1.amount);

  console.log("Token deployed successfully!");
  console.log("Token Mint Address:", tokenAccount.address.toBase58());
}

deployToken().catch(console.error);
