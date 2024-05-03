import Navbar from "./Navbar";
import NFTTile from "./NFTTile";
import MarketplaceJSON from "../Marketplace.json";
import axios from "axios";
import { useState } from "react";
import { GetIpfsUrlFromPinata } from "../utils";

export default function Marketplace() {
  const sampleData = [
    {
      name: "",
      description: "",
      website: "",
      image: "",
      price: "",
      currentlySelling: "",
      address: "",
    },
    // {
    //   name: "NFT#2",
    //   description: "Unblock's Second NFT",
    //   website: "http://axieinfinity.io",
    //   image:
    //     "https://gateway.pinata.cloud/ipfs/QmNjgYxETvvn1pEBYy2JPnQ2Rvxy7Ms6Y82nihJQtU9niV",
    //   price: "0.03ETH",
    //   currentlySelling: "True",
    //   address: "0xe81Bf5A757C4f7F82a2F23b1e59bE45c33c5b13",
    // },
    // {
    //   name: "NFT#3",
    //   description: "Unblock's Third NFT",
    //   website: "http://axieinfinity.io",
    //   image:
    //     "https://gateway.pinata.cloud/ipfs/QmNjgYxETvvn1pEBYy2JPnQ2Rvxy7Ms6Y82nihJQtU9niV",
    //   price: "0.03ETH",
    //   currentlySelling: "True",
    //   address: "0xe81Bf5A757C4f7F82a2F23b1e59bE45c33c5b13",
    // },
  ];
  const [data, updateData] = useState(sampleData);
  const [dataFetched, updateFetched] = useState(false);
  const [priced, updatePrice] = useState(0);
  const [totalminted, updateTotalMinted] = useState(0);
  async function getAllNFTs() {
    const ethers = require("ethers");
    //After adding your Hardhat network to your metamask, this code will get providers and signers
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();
    //Pull the deployed contract instance
    let contract = new ethers.Contract(
      MarketplaceJSON.address,
      MarketplaceJSON.abi,
      signer
    );
    console.log(contract);
    //create an NFT Token
    let transaction = await contract.getAllNFTs();

    //Fetch all the details of every NFT from the contract and display
    const items = await Promise.all(
      transaction.map(async (i) => {
        console.log("This is js of market place ");
        console.log(i);

        var tokenURI = await contract.tokenURI(1);

        console.log(`this is token uri ${tokenURI}`);
        console.log("getting this tokenUri", tokenURI);
        tokenURI = GetIpfsUrlFromPinata(tokenURI);
        console.log(tokenURI);
        // let meta = await axios.get(tokenURI);
        // meta = meta.data;
        // console.log(meta);
        var pric = await contract.getCurrentPrice();
        let price = ethers.utils.formatUnits(pric, "ether");
        console.log(price);
        let item = {
          price,
          tokenId: i.tokenId.toNumber(),
          seller: i.seller,
          owner: i.owner,
          image:
            "https://ipfs.io/ipfs/QmNjgYxETvvn1pEBYy2JPnQ2Rvxy7Ms6Y82nihJQtU9niV",
          name: "Enlighten your Mind",
          description: "MarketPlace Edition 0",
        };
        return item;
      })
    );

    updateFetched(true);
    updateData(items);
    getPrice();
  }

  if (!dataFetched) getAllNFTs();

  async function sellNFT() {
    const ethers = require("ethers");

    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();
    // console.log(signer.getAddress());
    //Pull the deployed contract instance
    let contract = new ethers.Contract(
      MarketplaceJSON.address,
      MarketplaceJSON.abi,
      signer
    );
    let price = await contract.getCurrentPrice();
    console.log(price.toNumber());
    let newPrice = price.toNumber() / 10 ** 18;
    let tokenId = await contract.getCurrentTokenId();
    let signerAddress = await signer.getAddress();
    let buyNFT = await contract.buyFromSupply(signerAddress, { value: price });
    await buyNFT.wait();
    getPrice();

    alert(
      `NFT Bought Successfully TokenID : ${tokenId} for price ${newPrice} ETH`
    );
  }
  async function getPrice() {
    const ethers = require("ethers");

    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();
    // console.log(signer.getAddress());
    //Pull the deployed contract instance
    let contract = new ethers.Contract(
      MarketplaceJSON.address,
      MarketplaceJSON.abi,
      signer
    );
    let price = await contract.getCurrentPrice();
    console.log(price.toNumber());
    let newPrice = price.toNumber() / 10 ** 18;
    updatePrice(newPrice);
    let totalTokens = await contract.getCurrentTokenId();
    updateTotalMinted(totalTokens.toNumber());
  }
  // getPrice();

  return (
    <div>
      <Navbar></Navbar>
      <div className="flex flex-col place-items-center mt-12">
        {/* <div className="md:text-xl font-bold text-white">Top NFTs</div> */}
        <div className="flex m-5 mb-2 justify-between flex-wrap max-w-screen-xl text-center ">
          {data.map((value, index) => {
            return <NFTTile data={value} key={0}></NFTTile>;
          })}
        </div>
        <div className="flex flex-col">
          <div className="mt-4 flex flex-row">
            <p className="flex justify-center bg-indigo-500   text-white font-bold mx-1 py-2 px-2 w-48  border-b-4 border-indigo-700  rounded">
              Price: {priced} ETH{" "}
            </p>
            <p className="flex justify-center bg-indigo-500   text-white font-extralight  py-2 px-2 w-48  border-b-4 border-indigo-700  rounded">
              {totalminted}/100 sold
            </p>
          </div>
          <button
            className="bg-purple-500  hover:bg-purple-400 text-white font-bold w-96 mt-4 mx-2 mb-2 py-2 px-2   border-b-4 border-purple-700 hover:border-purple-500 rounded"
            onClick={sellNFT}
          >
            Buy NFT
          </button>
        </div>
      </div>
    </div>
  );
}

// {data.map((value, index) => {
//   for (let i = 0; (i = 1); i++) {
//     return <NFTTile data={value} key={0}></NFTTile>;
//   }
// })}
