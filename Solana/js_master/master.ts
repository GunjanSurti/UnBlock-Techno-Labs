//@ts-ignore
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
const fs = require("fs");
const bs58 = require("bs58");

// solana config set --url https://api.devnet.solana.com
import {
  createSignerFromKeypair,
  generateSigner,
  percentAmount,
  publicKey,
  EddsaInterface,
  keypairIdentity,
  signerIdentity,
  some,
  none,
  Amount,
  PublicKey,
  sol,
  publicKeyBytes,
  isPublicKey,
  assertPublicKey,
} from "@metaplex-foundation/umi";
import {
  createNft,
  fetchDigitalAsset,
  printSupply,
  createV1,
  createFungible,
  TokenStandard,
  findMetadataPda,
  fetchDigitalAssetWithTokenByMint,
  mplTokenMetadata,
  createMasterEditionV3,
  findMasterEditionPda,
} from "@metaplex-foundation/mpl-token-metadata";
import { utf8 } from "@metaplex-foundation/umi/serializers";

// Use the RPC endpoint of your choice.
const umi = createUmi("https://api.devnet.solana.com").use(mplTokenMetadata());

// Import your private key file and parse it.
// const wallet = "./newAuthority.json";
const wallet = "./solpg-wallet.json";
const secretKey = JSON.parse(fs.readFileSync(wallet, "utf-8"));
// console.log(secretKey);

// // Create a keypair from your private key
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey));
// // console.log(keypair.publicKey);
// // console.log(keypair.secretKey);
// // console.log(keypair);

// // Register it to the Umi client.
umi.use(keypairIdentity(keypair)); // this is important to sign the transaction from our address

// const mint = generateSigner(umi);
// console.log(mint.publicKey);

// // await createNft(umi, {
// //   mint,
// //   name: "My NFT",
// //   uri: "https://arweave.net/O27ikLNE5nSWNd-oDvpmZW46VYiwnsD96JYi_v89Uss",
// //   // uri: "https://example.com/my-nft.json",
// //   sellerFeeBasisPoints: percentAmount(5.5),
// // }).sendAndConfirm(umi);

// // await createNft()
const mint = generateSigner(umi);
// const sa = await createFungible(umi, {
//   mint,
//   name: "My Fungible",
//   uri: "https://example.com/my-fungible.json",
//   sellerFeeBasisPoints: percentAmount(5.5),
//   decimals: some(7), // for 0 decimals use some(0)
// }).sendAndConfirm(umi);
// console.log(sa);

// // await createMasterEditionV3()   
const pubke: PublicKey = publicKey(
  "CuGyenwGHjC1VCHkbjmJdCqXp5UySigoTz9bkFYzPygc"
);
// let sig = await umi.rpc.airdrop(pubke, sol(1));

let bal = await umi.rpc.getBalance(pubke);
console.log();

let c = assertPublicKey(pubke);
console.log(c);

// console.log(Number(bal.basisPoints) / 10**9);

// let x = publicKeyBytes(pubke)
// console.log(x);
const pubk: PublicKey = publicKey(
  "9XoRmzWDKt6Jyb9hu2Hsx915GXgQWpwWeteHLiqzyUgr"
);
// let master = findMasterEditionPda(umi, { pubk });
let f = umi.eddsa.isOnCurve(pubk);
console.log(f);
