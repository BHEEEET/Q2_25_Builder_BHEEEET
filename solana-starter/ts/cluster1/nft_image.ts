import wallet from "./wallet/turbine-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({ address:"https://devnet.irys.xyz" }));
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const png = await readFile("./cluster1/wallet/SolanaBeNeLux.png")

        //2. Convert image to generic file.
        const file = createGenericFile(png, "SolanaBeneluxDevnet",{
            contentType: "image/png"
        })

        //3. Upload 
        const [myUri] = await umi.uploader.upload([file])
        console.log("Your image URI: ", myUri);
        // change the outputeed uri to https://devnet.irys.xyz
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
