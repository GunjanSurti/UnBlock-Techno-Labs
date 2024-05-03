import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import SellNFT from "./components/SellNFT";
import Marketplace from "./components/Marketplace";
import Profile from "./components/Profile";
import NFTPage from "./components/NFTpage";

///////////////////////////////////////////////////////////////////
import "@rainbow-me/rainbowkit/styles.css";
import {
  getDefaultConfig,
  RainbowKitProvider,
  darkTheme,
} from "@rainbow-me/rainbowkit";
import { WagmiProvider } from "wagmi";
import { sepolia, localhost } from "wagmi/chains";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";

const config = getDefaultConfig({
  appName: "My MarketPlace",
  projectId: "ba465e23e8ba0409ef748df0b1c9d232",
  chains: [sepolia, localhost],
  ssr: false, // If your dApp uses server side rendering (SSR)
});
const queryClient = new QueryClient();

///////////////////////////////////////////////////////////////////

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(
  <WagmiProvider config={config}>
    <QueryClientProvider client={queryClient}>
      <RainbowKitProvider modalSize="compact" coolMode theme={darkTheme()}>
        <React.StrictMode>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<Marketplace />} />
              {/* <Route path="/sellNFT" element={<SellNFT />} />
              <Route path="/nftPage/:tokenId" element={<NFTPage />} />
              <Route path="/profile" element={<Profile />} /> */}
            </Routes>
          </BrowserRouter>
        </React.StrictMode>
      </RainbowKitProvider>
    </QueryClientProvider>
  </WagmiProvider>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
