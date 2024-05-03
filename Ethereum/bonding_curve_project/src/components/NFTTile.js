
import { BrowserRouter as Router, Link } from "react-router-dom";
import { GetIpfsUrlFromPinata } from "../utils";

function NFTTile(data) {
  const newTo = {
    pathname: "/nftPage/" + data.data.tokenId,
  };

  const IPFSUrl = GetIpfsUrlFromPinata(data.data.image);

  return (
    // <Link to={newTo}>
    <div className="border-2 flex flex-col items-center rounded-lg w-48 md:w-96 md-h:96 shadow-2xl">
      <img
        src={IPFSUrl}
        alt=""
        className="w-96 h-96 rounded-lg object-cover"
        crossOrigin="anonymous"
      />
      <div className="text-white w-full p-2 bg-gradient-to-t from-[#454545] to-transparent rounded-lg pt-5 -mt-20">
        <strong className="text-xl">{data.data.name}</strong>
        <p className="display-inline">{data.data.description}</p>
      </div>
    </div>
    // </Link>
  );
}

export default NFTTile;
