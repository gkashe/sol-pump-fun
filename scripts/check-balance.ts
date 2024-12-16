import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import mekey from "../tests/keys/mekey.json";
import key1 from "../tests/keys/user1.json";
import key2 from "../tests/keys/user2.json";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

async function checkBalances() {
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    
    const mykey = Keypair.fromSecretKey(new Uint8Array(mekey));
    const payer = Keypair.fromSecretKey(new Uint8Array(key1));
    const feeAccount = Keypair.fromSecretKey(new Uint8Array(key2));

    const mykeyBalance = await connection.getBalance(mykey.publicKey);
    const payerBalance = await connection.getBalance(payer.publicKey);
    const feeAccountBalance = await connection.getBalance(feeAccount.publicKey);

    console.log("MyKey wallet (user1):");
    console.log("  Address:", mykey.publicKey.toBase58());
    console.log("  Balance:", mykeyBalance / LAMPORTS_PER_SOL, "SOL");
    
    console.log("Payer wallet (user1):");
    console.log("  Address:", payer.publicKey.toBase58());
    console.log("  Balance:", payerBalance / LAMPORTS_PER_SOL, "SOL");
    // console.log("  Payer secret key (Base58):", bs58.encode(payer.secretKey));
    
    console.log("\nFee Account wallet (user2):");
    console.log("  Address:", feeAccount.publicKey.toBase58());
    console.log("  Balance:", feeAccountBalance / LAMPORTS_PER_SOL, "SOL");
    // console.log("  feeAccount secret key (Base58):", bs58.encode(feeAccount.secretKey));
}

checkBalances().catch(console.error);
