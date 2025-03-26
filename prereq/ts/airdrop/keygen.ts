import { Keypair } from "@solana/web3.js";

// Generate a new keypair
let kp = Keypair.generate();
console.log(kp.publicKey.toBase58());
console.log(`[${kp.secretKey}]`);