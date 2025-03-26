import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "../../../../turbine-cohort-repo/0-pre-req/rust/Turbine-wallet.json"

const keypair = Keypair.fromSecretKey(Uint8Array.from(wallet));
const connection = new Connection("https://api.devnet.solana.com");

const github = Buffer.from("bheeeet", "utf-8")

const provider = new AnchorProvider(connection, new Wallet(keypair), {commitment: "confirmed"});

const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create PDA for enrollment acccount
const enrollment_seeds = [Buffer.from("pre"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);

(async () => {
    try{
        const txhash = await program.methods
        .submit(github)
        .accounts({
            signer: keypair.publicKey
        })
        .signers([
            keypair
        ]).rpc()
        console.log(`Success! Check out TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.log(e);
    }
}) ();