import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json"

const keypair = Keypair.fromSecretKey(Uint8Array.from(wallet));
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        const txhash = await connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
        console.log(`Success! Check out TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        const balance = await connection.getBalance(keypair.publicKey);
        console.log(balance / LAMPORTS_PER_SOL);
    } catch (e) {
        console.log(e);
    }
})();
