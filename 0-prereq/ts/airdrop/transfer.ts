import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import wallet from "./dev-wallet.json"
import towallet from "../../../../turbine-cohort-repo/0-pre-req/rust/Turbine-wallet.json"

console.log(Keypair.fromSecretKey(Uint8Array.from(towallet)).publicKey.toBase58());

const from = Keypair.fromSecretKey(Uint8Array.from(wallet));
const to = new PublicKey("GaKuQyYqJKNy8nN9Xf6VmYJQXzQDvvUHHc8kTeGQLL3f");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        // Get balance
        const balance = await connection.getBalance(from.publicKey);

        // Create transaction
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance,
            })
        );
        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;

        // Calculate fee
        const fee = (await connection.getFeeForMessage(transaction.compileMessage(), 'confirmed')).value || 0;

        // Remove transfer intstruction
        transaction.instructions.pop();

        // Add transfer instruction with correct amount of lamports
        transaction.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee,
            })
        );

        // Sign transaction
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]);
        console.log("Success! Check out your TX: https://explorer.solana.com/tx/" + signature + "?cluster=devnet");
    } catch (e) {
        console.log(e);
    }
})();