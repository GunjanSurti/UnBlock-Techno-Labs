// import fullLogo from "../full_logo.png";
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link,
  useRouteMatch,
  useParams,
} from "react-router-dom";
import { useEffect, useState } from "react";
import { useLocation } from "react-router";
//////////////////////////////////
import { ConnectButton } from "@rainbow-me/rainbowkit";

//////////////////////////////////
function Navbar() {
  const [connected, toggleConnect] = useState(false);
  const location = useLocation();
  const [currAddress, updateAddress] = useState("0x");

  async function getAddress() {
    const ethers = require("ethers");
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();
    const addr = await signer.getAddress();
    updateAddress(addr);
  }

  function updateButton() {
    const ethereumButton = document.querySelector(".enableEthereumButton");
    ethereumButton.textContent = "Connected";
    ethereumButton.classList.remove("hover:bg-blue-70");
    ethereumButton.classList.remove("bg-blue-500");
    ethereumButton.classList.add("hover:bg-green-70");
    ethereumButton.classList.add("bg-green-500");
  }

  async function connectWebsite() {
    const chainId = await window.ethereum.request({ method: "eth_chainId" });
    if (chainId !== "0xaa36a7") {
      // hardhat chain id 31337

      //alert('Incorrect network! Switch your metamask network to Rinkeby');
      await window.ethereum.request({
        method: "wallet_switchEthereumChain",
        params: [{ chainId: "0xaa36a7" }],
      });
    }
    await window.ethereum
      .request({ method: "eth_requestAccounts" })
      .then(() => {
        updateButton();
        console.log("here");
        getAddress();
        window.location.replace(location.pathname);
      });
  }

  useEffect(() => {
    if (window.ethereum == undefined) return;
    let val = window.ethereum.isConnected();
    if (val) {
      console.log("here");
      getAddress();
      toggleConnect(val);
      updateButton();
    }

    window.ethereum.on("accountsChanged", function (accounts) {
      window.location.replace(location.pathname);
      console.log(` this location accounts : ${accounts}`);
    });
  }, []);

  return (
    // <div className="">
    //   <nav className="w-screen">
    //     <ul className="flex items-end justify-between py-3 bg-transparent text-white pr-5">
    //       <li className="flex items-end ml-5 pb-2">
    //         <Link to="/">
    //           {/* <img src={fullLogo} alt="" width={120} height={120} className="inline-block -mt-2"/> */}
    //           <div className="inline-block font-bold text-xl ml-2">
    //             NFT Marketplace
    //           </div>
    //         </Link>
    //       </li>
    //       <li className="w-2/6">
    //         <ul className="lg:flex justify-between font-bold mr-10 text-lg">
    // {location.pathname === "/" ? (
    //   <li className="border-b-2 hover:pb-0 p-2">
    //     <Link to="/">Marketplace</Link>
    //   </li>
    // ) : (
    //   <li className="hover:border-b-2 hover:pb-0 p-2">
    //     <Link to="/">Marketplace</Link>
    //   </li>
    // )}
    //           {/* {location.pathname === "/sellNFT" ? (
    //             <li className="border-b-2 hover:pb-0 p-2">
    //               <Link to="/sellNFT">List My NFT</Link>
    //             </li>
    //           ) : (
    //             <li className="hover:border-b-2 hover:pb-0 p-2">
    //               <Link to="/sellNFT">List My NFT</Link>
    //             </li>
    //           )}
    //           {location.pathname === "/profile" ? (
    //             <li className="border-b-2 hover:pb-0 p-2">
    //               <Link to="/profile">Profile</Link>
    //             </li>
    //           ) : (
    //             <li className="hover:border-b-2 hover:pb-0 p-2">
    //               <Link to="/profile">Profile</Link>
    //             </li>
    //           )} */}
    //           {/* <li>
    //             <button
    //               className="enableEthereumButton bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded text-sm"
    //               onClick={connectWebsite}
    //             >
    //               {connected ? "Connected" : "Connect Wallet"}
    //             </button>
    //           </li> */}
    //         </ul>
    //         <ul>
    //           <li>
    //             <ConnectButton />
    //           </li>
    //         </ul>
    //       </li>
    //     </ul>
    //   </nav>
    //   {/* <div className="text-white text-bold text-right mr-10 text-sm">
    //     {currAddress !== "0x"
    //       ? "Connected to"
    //       : "Not Connected. Please login to view NFTs"}{" "}
    //     {currAddress !== "0x" ? currAddress.substring(0, 15) + "..." : ""}
    //   </div> */}
    // </div>
    <header className="text-gray-600 body-font">
      <div className="container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center">
        <a className="flex title-font font-medium items-center text-gray-900 mb-4 md:mb-0">
          <span className="ml-3 text-xl">NFT MarketPlace</span>
        </a>
        <nav className="md:ml-auto flex flex-wrap items-center text-base justify-center">
          {location.pathname === "/" ? (
            <p className="border-b-0 text-bold p-3 title-font font-medium text-gray-900">
              <Link to="/">Marketplace</Link>
            </p>
          ) : (
            {}
          )}
        </nav>
          <ConnectButton />
      </div>
    </header>
  );
}

export default Navbar;
