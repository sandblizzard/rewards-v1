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
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getOrCreateAssociatedTokenAccountIx = void 0;
const spl = __importStar(require("@solana/spl-token"));
const web3_js_1 = require("@solana/web3.js");
const getOrCreateAssociatedTokenAccountIx = (connection, payer, mint, owner, allowOwnerOffCurve = false, commitment = 'finalized', programId = spl.TOKEN_PROGRAM_ID, associatedTokenProgramId = spl.ASSOCIATED_TOKEN_PROGRAM_ID) => __awaiter(void 0, void 0, void 0, function* () {
    const associatedToken = yield spl.getAssociatedTokenAddress(mint, owner, allowOwnerOffCurve, programId, associatedTokenProgramId);
    // This is the optimal logic, considering TX fee, client-side computation, RPC roundtrips and guaranteed idempotent.
    // Sadly we can't do this atomically.
    let account;
    try {
        account = yield spl.getAccount(connection, associatedToken, commitment, programId);
        return {
            instruction: null,
            address: associatedToken,
        };
    }
    catch (error) {
        // TokenAccountNotFoundError can be possible if the associated address has already received some lamports,
        // becoming a system account. Assuming program derived addressing is safe, this is the only case for the
        // TokenInvalidAccountOwnerError in this code path.
        if (error instanceof spl.TokenAccountNotFoundError ||
            error instanceof spl.TokenInvalidOwnerError) {
            // As this isn't atomic, it's possible others can create associated accounts meanwhile.
            try {
                const transaction = new web3_js_1.TransactionInstruction(spl.createAssociatedTokenAccountInstruction(payer, associatedToken, owner, mint, programId, associatedTokenProgramId));
                return {
                    instruction: transaction,
                    address: associatedToken,
                };
            }
            catch (error) {
                // Ignore all errors; for now there is no API-compatible way to selectively ignore the expected
                // instruction error if the associated account exists already.
            }
            // Now this should always succeed
            account = yield spl.getAccount(connection, associatedToken, commitment, programId);
        }
        else {
            throw error;
        }
    }
    if (!account.mint.equals(mint))
        throw new spl.TokenInvalidMintError();
    if (!account.owner.equals(owner))
        throw new spl.TokenInvalidOwnerError();
    return {
        address: associatedToken,
        instruction: null,
    };
});
exports.getOrCreateAssociatedTokenAccountIx = getOrCreateAssociatedTokenAccountIx;
//# sourceMappingURL=utils.js.map