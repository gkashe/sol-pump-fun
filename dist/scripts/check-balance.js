"use strict";
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
const web3_js_1 = require("@solana/web3.js");
const user1_json_1 = __importDefault(require("../tests/keys/user1.json"));
const user2_json_1 = __importDefault(require("../tests/keys/user2.json"));
function checkBalances() {
    return __awaiter(this, void 0, void 0, function* () {
        const connection = new web3_js_1.Connection("https://api.devnet.solana.com", "confirmed");
        const payer = web3_js_1.Keypair.fromSecretKey(new Uint8Array(user1_json_1.default));
        const feeAccount = web3_js_1.Keypair.fromSecretKey(new Uint8Array(user2_json_1.default));
        const payerBalance = yield connection.getBalance(payer.publicKey);
        const feeAccountBalance = yield connection.getBalance(feeAccount.publicKey);
        console.log("Payer wallet (user1):");
        console.log("  Address:", payer.publicKey.toBase58());
        console.log("  Balance:", payerBalance / web3_js_1.LAMPORTS_PER_SOL, "SOL");
        console.log("\nFee Account wallet (user2):");
        console.log("  Address:", feeAccount.publicKey.toBase58());
        console.log("  Balance:", feeAccountBalance / web3_js_1.LAMPORTS_PER_SOL, "SOL");
    });
}
checkBalances().catch(console.error);
//# sourceMappingURL=check-balance.js.map