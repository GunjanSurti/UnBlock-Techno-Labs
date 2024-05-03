// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.0;

import "hardhat/console.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract MyToken is ERC721URIStorage {
    address payable public admin; // this address will
    using Counters for Counters.Counter;

    Counters.Counter private _tokenIds;

    constructor() ERC721("MyToken", "MTK") {
        admin = payable(msg.sender);
        safeMintForAdmin(
            "https://gateway.pinata.cloud/ipfs/QmNjgYxETvvn1pEBYy2JPnQ2Rvxy7Ms6Y82nihJQtU9niV"
        );
        // safeMintForAdmin("https://cloudflare-ipfs.com/ipfs/QmNZEeAN1zk6hLoHHREVkZ7PoPYaoH7n6LR6w9QAcEc29h");
        // safeMintForAdmin("QmNZEeAN1zk6hLoHHREVkZ7PoPYaoH7n6LR6w9QAcEc29h");
        console.log("This is constructor");
    }
    //////////////////////////////////////////////////////////////

    struct ListedToken {
        uint256 tokenId;
    }
    mapping(uint256 => ListedToken) private idToListedToken;

    function createListedToken(uint256 tokenId) private {
        //Update the mapping of tokenId's to Token details, useful for retrieval functions
        idToListedToken[tokenId] = ListedToken(tokenId);
    }
    //////////////////////////////////////////////////////////////

    // this function mints first nft to its owner

    function safeMintForAdmin(string memory tokenURI) private {
        _tokenIds.increment();

        uint256 newTokenId = _tokenIds.current();

        console.log("Id after deployment %s", newTokenId);

        _safeMint(admin, newTokenId);
        _setTokenURI(newTokenId, tokenURI);
    }

    function buyFromSupply(address to) public payable {
        _tokenIds.increment();
        uint256 tokenId = _tokenIds.current();

        console.log("Id while deployment %s", tokenId);

        uint256 feez = getPrice(tokenId, 1);

        require(msg.value == feez, "Not enough ETH sent");

        _safeMint(to, tokenId);

        // transferring ether to deployer of contract
        (bool success, ) = admin.call{value: msg.value}("");
        require(success == true, "The value is not sent ");
    }

    function getCurrentPrice() public view returns (uint256) {
        uint256 id = _tokenIds.current();
        id++;
        return getPrice(id, 1);
    }

    /*
     *  BondingCurve
     *
     * @param s supply, total shares supply
     * @param a amount, number of shares to buy/sell
     */
    function getPrice(
        uint256 s,
        uint256 a
    ) public pure virtual returns (uint256) {
        // this is the original friend tech formula with the underflow fix
        // the fix allows both supply and amount be zero, as well as
        // it allows supply be zero when the amount is bigger than one
        uint256 sum1 = s == 0 ? 0 : ((s - 1) * s * (2 * (s - 1) + 1)) / 6;
        uint256 sum2 = s == 0 && a <= 1
            ? 0
            : ((s + a - 1) * (s + a) * (2 * (s + a - 1) + 1)) / 6;
        uint256 summation = sum2 - sum1;

        // this returns value equivalent in usd
        // thats why we are doing this calculation and dividing by 16000
        return (summation * 1 ether) / 16000;
    }

    function getCurrentTokenId() public view returns (uint256) {
        return _tokenIds.current();
    }

    function getAllNFTs() public view returns (ListedToken[] memory) {
        // uint nftCount = _tokenIds.current();
        ListedToken[] memory tokens = new ListedToken[](1);
        uint currentIndex = 0;
        uint currentId;
        //at the moment currentlyListed is true for all, if it becomes false in the future we will
        //filter out currentlyListed == false over here
        for (uint i = 0; i < 1; i++) {
            currentId = i + 1;
            ListedToken storage currentItem = idToListedToken[currentId];
            tokens[currentIndex] = currentItem;
            currentIndex += 1;
        }
        //the array 'tokens' has the list of all NFTs in the marketplace
        return tokens;
    }
}
