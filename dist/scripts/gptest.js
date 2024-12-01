"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const dotenv = __importStar(require("dotenv"));
// Load environment variables from .env file
dotenv.config();
const anchor = __importStar(require("@coral-xyz/anchor"));
const anchor_1 = require("@coral-xyz/anchor");
const bytes_1 = require("@coral-xyz/anchor/dist/cjs/utils/bytes");
const spl_token_1 = require("@solana/spl-token");
const web3_js_1 = require("@solana/web3.js");
const token_1 = require("@coral-xyz/anchor/dist/cjs/utils/token");
const system_1 = require("@coral-xyz/anchor/dist/cjs/native/system");
const user1_json_1 = __importDefault(require("../tests/keys/user1.json"));
const user2_json_1 = __importDefault(require("../tests/keys/user2.json"));
let connection;
let payer;
let feeAccount;
let program;
let mintAddr;
let userNativeAta;
let userAta;
function airdrop(publicKey, amount) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            // 1 - Request Airdrop
            const signature = yield connection.requestAirdrop(publicKey, amount);
            // 2 - Fetch the latest blockhash
            const { blockhash, lastValidBlockHeight } = yield connection.getLatestBlockhash();
            // 3 - Confirm transaction success
            yield connection.confirmTransaction({
                blockhash,
                lastValidBlockHeight,
                signature,
            });
        }
        catch (error) {
            console.error("Error getting airdrop:", error);
            throw error;
        }
    });
}
function requestAirdrop() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const airdropAmount = Math.pow(10, 11);
            console.log(`Requesting airdrop to admin for 1SOL : ${payer.publicKey.toBase58()}`);
            yield airdrop(payer.publicKey, airdropAmount);
            yield airdrop(feeAccount.publicKey, airdropAmount);
            const adminBalance = (yield connection.getBalance(feeAccount.publicKey)) / Math.pow(10, 9);
            console.log("admin wallet balance : ", adminBalance, "SOL");
        }
        catch (error) {
            console.error("Error requesting airdrop:", error);
            throw error;
        }
    });
}
function initialize() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const [globalConfiguration] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId);
            const [feeAccount] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("pumpfun_fee")], program.programId);
            // Check if the account already exists
            const accountInfo = yield connection.getAccountInfo(globalConfiguration);
            if (accountInfo !== null) {
                console.log("Global configuration already initialized");
                return;
            }
            const initializeArgu = {
                swapFee: 2.0,
                bondingCurveLimitation: new anchor_1.BN(8 * web3_js_1.LAMPORTS_PER_SOL),
                solAmountForDexAfterBc: new anchor_1.BN(5 * web3_js_1.LAMPORTS_PER_SOL),
                solAmountForPumpfunAfterBc: new anchor_1.BN(2 * web3_js_1.LAMPORTS_PER_SOL),
                solAmountForTokenCreatorAfterBc: new anchor_1.BN(1 * web3_js_1.LAMPORTS_PER_SOL),
                initialVirtualSol: new anchor_1.BN(3 * web3_js_1.LAMPORTS_PER_SOL),
            };
            // Add your test here.
            const tx = yield program.methods
                .initialize(initializeArgu)
                .accounts({
                globalConfiguration: globalConfiguration,
                feeAccount: feeAccount,
            })
                .signers([payer])
                .transaction();
            tx.feePayer = payer.publicKey;
            tx.recentBlockhash = (yield connection.getLatestBlockhash()).blockhash;
            console.log(yield connection.simulateTransaction(tx));
            const sig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer]);
            console.log("Initialization transaction signature:", sig);
            console.log("Global configuration:", yield program.account.initializeConfiguration.fetch(globalConfiguration));
        }
        catch (error) {
            if (error instanceof anchor.web3.SendTransactionError) {
                console.error("Transaction failed:", error.logs);
            }
            else {
                console.error("Error initializing:", error);
            }
            throw error;
        }
    });
}
function tokenMint() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            userNativeAta = yield (0, spl_token_1.getAssociatedTokenAddress)(spl_token_1.NATIVE_MINT, payer.publicKey);
            console.log(userNativeAta);
            mintAddr = web3_js_1.Keypair.generate();
            const mintsig = yield (0, spl_token_1.createMint)(connection, payer, payer.publicKey, null, 9, mintAddr, { commitment: "finalized" }, spl_token_1.TOKEN_2022_PROGRAM_ID);
            console.log(mintsig.toBase58());
            userAta = yield (0, spl_token_1.createAssociatedTokenAccount)(connection, payer, mintAddr.publicKey, payer.publicKey, { commitment: "finalized" }, spl_token_1.TOKEN_2022_PROGRAM_ID);
            console.log("userAta: ", userAta);
            const mintto_sig = yield (0, spl_token_1.mintTo)(connection, payer, mintAddr.publicKey, userAta, payer, Math.pow(10, 15), [], { commitment: "finalized" }, spl_token_1.TOKEN_2022_PROGRAM_ID);
            console.log(mintto_sig);
        }
        catch (error) {
            console.error("Error in tokenMint:", error);
            throw error;
        }
    });
}
function createPool() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const [globalConfiguration] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId);
            const [bondingCurve] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("bonding_curve"), mintAddr.publicKey.toBuffer()], program.programId);
            const [solPool] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("sol_pool"), mintAddr.publicKey.toBuffer()], program.programId);
            const tokenPool = yield (0, spl_token_1.getAssociatedTokenAddress)(mintAddr.publicKey, solPool, true, spl_token_1.TOKEN_2022_PROGRAM_ID);
            console.log(mintAddr.publicKey.toBase58());
            console.log({
                globalConfiguration: globalConfiguration,
                bondingCurve: bondingCurve,
                mintAddr: mintAddr.publicKey,
                userAta: userAta,
                solPool: solPool,
                tokenPool: tokenPool,
                feeAccount: feeAccount,
                tokenProgram: spl_token_1.TOKEN_2022_PROGRAM_ID,
            });
            // Add your test here.
            const tx = yield program.methods
                .createPool(new anchor_1.BN(Math.pow(10, 7))) //   create Pool Fee 0.01 sol
                .accounts({
                globalConfiguration: globalConfiguration,
                bondingCurve: bondingCurve,
                mintAddr: mintAddr.publicKey,
                userAta: userAta,
                solPool: solPool,
                tokenPool: tokenPool,
                feeAccount: feeAccount.publicKey,
                tokenProgram: spl_token_1.TOKEN_2022_PROGRAM_ID,
                associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
            })
                .signers([payer])
                .transaction();
            tx.feePayer = payer.publicKey;
            tx.recentBlockhash = (yield connection.getLatestBlockhash()).blockhash;
            console.log(yield connection.simulateTransaction(tx));
            const sig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer]);
            console.log(sig);
        }
        catch (error) {
            console.error("Error in createPool:", error);
            throw error;
        }
    });
}
function addLiquidity() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const [globalConfiguration] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId);
            const [bondingCurve] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("bonding_curve"), mintAddr.publicKey.toBuffer()], program.programId);
            const [solPool] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("sol_pool"), mintAddr.publicKey.toBuffer()], program.programId);
            const tokenPool = yield (0, spl_token_1.getAssociatedTokenAddress)(mintAddr.publicKey, solPool, true, spl_token_1.TOKEN_2022_PROGRAM_ID);
            // Add your test here.
            const tx = yield program.methods
                .addLiquidity(new anchor_1.BN(5 * Math.pow(10, 13)), new anchor_1.BN(Math.pow(10, 13))) //   token deposit
                .accounts({
                globalConfiguration: globalConfiguration,
                bondingCurve: bondingCurve,
                mintAddr: mintAddr.publicKey,
                userAta: userAta,
                solPool: solPool,
                tokenPool: tokenPool,
                tokenProgram: spl_token_1.TOKEN_2022_PROGRAM_ID,
            })
                .signers([payer])
                .transaction();
            tx.feePayer = payer.publicKey;
            tx.recentBlockhash = (yield connection.getLatestBlockhash()).blockhash;
            console.log(yield connection.simulateTransaction(tx));
            const sig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer]);
            console.log(sig);
            console.log("Init Configure : ", yield program.account.initializeConfiguration.fetch(globalConfiguration));
            console.log("Bonding Curve : ", yield program.account.bondingCurve.fetch(bondingCurve));
        }
        catch (error) {
            console.error("Error in addLiquidity:", error);
            throw error;
        }
    });
}
function buy() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const [globalConfiguration] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId);
            const [bondingCurve] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("bonding_curve"), mintAddr.publicKey.toBuffer()], program.programId);
            const [solPool] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("sol_pool"), mintAddr.publicKey.toBuffer()], program.programId);
            // await airdrop(solPool, 10 ** 11);
            const tokenPool = yield (0, spl_token_1.getAssociatedTokenAddress)(mintAddr.publicKey, solPool, true, spl_token_1.TOKEN_2022_PROGRAM_ID);
            const bunding = yield program.account.bondingCurve.fetch(bondingCurve);
            const price = bunding.virtualSolReserves.div(bunding.virtualTokenReserves);
            console.log(yield program.account.initializeConfiguration.fetch(globalConfiguration));
            console.log("bunding == > ", bunding.virtualSolReserves, bunding.virtualTokenReserves);
            console.log("bunding == > ", price);
            console.log(solPool);
            // Add your test here.
            const tx = yield program.methods
                .buy(new anchor_1.BN(2 * Math.pow(10, 8))) //   buy 0.1 sol
                .accounts({
                globalConfiguration: globalConfiguration,
                bondingCurve: bondingCurve,
                mintAddr: mintAddr.publicKey,
                userAta: userAta,
                solPool: solPool,
                tokenPool: tokenPool,
                feeAccount: feeAccount.publicKey,
                tokenProgram: spl_token_1.TOKEN_2022_PROGRAM_ID,
            })
                .signers([payer])
                .transaction();
            tx.feePayer = payer.publicKey;
            tx.recentBlockhash = (yield connection.getLatestBlockhash()).blockhash;
            console.log(yield connection.simulateTransaction(tx));
            const sig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer]);
            console.log(sig);
        }
        catch (error) {
            console.error("Error in buy:", error);
            throw error;
        }
    });
}
function sell() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const [globalConfiguration] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId);
            const [bondingCurve] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("bonding_curve"), mintAddr.publicKey.toBuffer()], program.programId);
            const [solPool] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("sol_pool"), mintAddr.publicKey.toBuffer()], program.programId);
            const tokenPool = yield (0, spl_token_1.getAssociatedTokenAddress)(mintAddr.publicKey, solPool, true, spl_token_1.TOKEN_2022_PROGRAM_ID);
            const bunding = yield program.account.bondingCurve.fetch(bondingCurve);
            const price = bunding.virtualSolReserves.div(bunding.virtualTokenReserves);
            console.log("bunding == > ", bunding.virtualSolReserves, bunding.virtualTokenReserves);
            console.log("bunding == > ", price);
            // Add your test here.
            const tx = yield program.methods
                .sell(new anchor_1.BN(Math.pow(10, 8))) //   sell 0.1 token
                .accounts({
                globalConfiguration: globalConfiguration,
                bondingCurve: bondingCurve,
                mintAddr: mintAddr.publicKey,
                userAta: userAta,
                solPool: solPool,
                tokenPool: tokenPool,
                feeAccount: feeAccount.publicKey,
                tokenProgram: spl_token_1.TOKEN_2022_PROGRAM_ID,
            })
                .signers([payer])
                .transaction();
            tx.feePayer = payer.publicKey;
            tx.recentBlockhash = (yield connection.getLatestBlockhash()).blockhash;
            console.log(yield connection.simulateTransaction(tx));
            const sig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer]);
            console.log(sig);
        }
        catch (error) {
            console.error("Error in sell:", error);
            throw error;
        }
    });
}
function removeLiquidity() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            userNativeAta = yield (0, spl_token_1.getAssociatedTokenAddress)(spl_token_1.NATIVE_MINT, payer.publicKey);
            const [globalConfiguration] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId);
            const [bondingCurve] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("bonding_curve"), mintAddr.publicKey.toBuffer()], program.programId);
            const [solPool] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("sol_pool"), mintAddr.publicKey.toBuffer()], program.programId);
            const tokenPool = yield (0, spl_token_1.getAssociatedTokenAddress)(mintAddr.publicKey, solPool, true, spl_token_1.TOKEN_2022_PROGRAM_ID);
            console.log("solPool : ", solPool);
            console.log("tokenPool : ", tokenPool);
            //  coin mint address
            const tx = yield program.methods
                .removeLiquidity()
                .accounts({
                globalConfiguration: globalConfiguration,
                bondingCurve: bondingCurve,
                ammCoinMint: mintAddr.publicKey,
                solPool: solPool,
                tokenPool: tokenPool,
                userTokenCoin: userAta,
                userTokenPc: userNativeAta,
                userWallet: payer.publicKey,
                tokenProgram: spl_token_1.TOKEN_2022_PROGRAM_ID,
                splTokenProgram: spl_token_1.TOKEN_PROGRAM_ID,
                associatedTokenProgram: token_1.ASSOCIATED_PROGRAM_ID,
                systemProgram: system_1.SYSTEM_PROGRAM_ID,
                sysvarRent: web3_js_1.SYSVAR_RENT_PUBKEY,
            })
                .preInstructions([
                web3_js_1.ComputeBudgetProgram.setComputeUnitLimit({
                    units: 1000000,
                }),
            ])
                .signers([payer])
                .transaction();
            tx.feePayer = payer.publicKey;
            tx.recentBlockhash = (yield connection.getLatestBlockhash()).blockhash;
            console.log(yield connection.simulateTransaction(tx));
            const sig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer], {
                skipPreflight: true,
            });
            console.log("Successfully Removed liquidity : ", sig);
        }
        catch (error) {
            console.error("Error in removeLiquidity:", error);
            throw error;
        }
    });
}
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        let anchor_provider_env = anchor.AnchorProvider.env();
        console.log("anchor_provider_env.wallet.publicKey.toBase58()", anchor_provider_env.wallet.publicKey.toBase58());
        console.log("anchor_provider_env.publicKey.toBase58()", anchor_provider_env.publicKey.toBase58());
        anchor.setProvider(anchor_provider_env);
        let anchor_provider = anchor.getProvider();
        console.log("anchor_provider: ", anchor_provider);
        connection = anchor_provider.connection;
        payer = web3_js_1.Keypair.fromSecretKey(new Uint8Array(user1_json_1.default));
        feeAccount = web3_js_1.Keypair.fromSecretKey(new Uint8Array(user2_json_1.default));
        program = anchor.workspace.Token2022Pumpfun;
        yield requestAirdrop();
        yield initialize();
        mintAddr = web3_js_1.Keypair.fromSecretKey(bytes_1.bs58.decode("JgRxbfyyorMTZxV3Gtaxn3EdGm76w7uqXENiJRxwZywHxzuV2fzLiWFd2R23PdMuTvnW59etkac15xxHggETymz"));
        userAta = new web3_js_1.PublicKey("Ck2yo7ZKtbvKkeJhdkFMTJEyjnMkVUL3ewa8DgF4zNu4");
        userNativeAta = new web3_js_1.PublicKey("2PJXikmzkd6jz5bsDeTCVN2SYuZihfhKAsGCoRRuLJ4U");
        yield tokenMint();
        yield createPool();
        yield addLiquidity();
        yield buy();
        yield sell();
        yield removeLiquidity();
        console.log('done');
    });
}
main();
//# sourceMappingURL=gptest.js.map