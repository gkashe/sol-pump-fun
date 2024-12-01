import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import key1 from "../tests/keys/user1.json";
import key2 from "../tests/keys/user2.json";

async function checkBalances() {
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    
    const payer = Keypair.fromSecretKey(new Uint8Array(key1));
    const feeAccount = Keypair.fromSecretKey(new Uint8Array(key2));

    const payerBalance = await connection.getBalance(payer.publicKey);
    const feeAccountBalance = await connection.getBalance(feeAccount.publicKey);

    console.log("Payer wallet (user1):");
    console.log("  Address:", payer.publicKey.toBase58());
    console.log("  Balance:", payerBalance / LAMPORTS_PER_SOL, "SOL");
    
    console.log("\nFee Account wallet (user2):");
    console.log("  Address:", feeAccount.publicKey.toBase58());
    console.log("  Balance:", feeAccountBalance / LAMPORTS_PER_SOL, "SOL");
}

checkBalances().catch(console.error);
