//@ts-ignore
import { verifyCollectionV1 } from "@metaplex-foundation/mpl-token-metadata";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

// Use the RPC endpoint of your choice.
const umi = createUmi("http://127.0.0.1:8899").use(mplTokenMetadata());

let metadata = new PublicKey("6NvhHqTTHVZoHEjoEc3oPRw7HrJ98Yn5Ku4kDq7XqiYj");
await verifyCollectionV1(umi, {
  metadata,
  collectionMint,
  authority: collectionAuthority,
}).sendAndConfirm(umi);
