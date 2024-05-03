/* eslint-disable no-unused-vars */
/* eslint-disable no-undef */
require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-ethers");
require("dotenv").config();

const { log } = require("console");
const { configDotenv } = require("dotenv");
const fs = require("fs");
// const infuraId = fs.readFileSync(".infuraid").toString().trim() || "";

task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});
const account = process.env.REACT_APP_PRIVATE_KEY;
const url = process.env.REACT_APP_ALCHEMY_API_URL;
// console.log(account);
// console.log(url);
module.exports = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat: {
      chainId: 1337,
    },
    mumbai: {
      url: `https://polygon-mumbai.g.alchemy.com/v2/nAhiCHKvZkhkp4A7PkkCIBON0-BXW26d`,
      //accounts: [process.env.privateKey]
    },
    matic: {
      url: "https://polygon-mainnet.g.alchemy.com/v2/nAhiCHKvZkhkp4A7PkkCIBON0-BXW26d",
      //accounts: [process.env.privateKey]
    },
    sepolia: {
      url: url,
      chainId: 11155111,
      accounts: [account],
    },
  },
  solidity: {
    version: "0.8.4",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
};
