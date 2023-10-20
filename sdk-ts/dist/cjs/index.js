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
exports.BountySdk = exports.getSandMint = exports.getFeeCollectorPDA = exports.getRelayerPDA = exports.getEscrowPDA = exports.getDomainPDA = exports.getBountyPDA = exports.getDenominationPDA = exports.getProtocolPDA = exports.BOUNTY_PROGRAM_ID = exports.utils = void 0;
const anchor = __importStar(require("@coral-xyz/anchor"));
const bounty_1 = require("./idl/bounty");
const spl_token_1 = require("@solana/spl-token");
exports.utils = __importStar(require("./utils"));
exports.BOUNTY_PROGRAM_ID = new anchor.web3.PublicKey("74cnoYJmzNmGLVwj1k88eGRXMZ6srVnNt32dn7qiivpU");
/**
 * getProtocolPDA
 * @param programId
 * @returns
 */
const getProtocolPDA = () => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD")], exports.BOUNTY_PROGRAM_ID);
};
exports.getProtocolPDA = getProtocolPDA;
const getSandMint = () => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("sand_mint")], exports.BOUNTY_PROGRAM_ID);
};
exports.getSandMint = getSandMint;
const getDenominationPDA = (mint) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("DENOMINATION"), mint.toBuffer()], exports.BOUNTY_PROGRAM_ID);
};
exports.getDenominationPDA = getDenominationPDA;
const getFeeCollectorPDA = (mint) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("FEE_COLLECTOR"), mint.toBuffer()], exports.BOUNTY_PROGRAM_ID);
};
exports.getFeeCollectorPDA = getFeeCollectorPDA;
/**
 * getBountyPDA
 * @param id: typically the internal id for the given domain.
 *              e.g. for github issues it will be issueId
 * @returns
 */
const getBountyPDA = (id) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from(id)], exports.BOUNTY_PROGRAM_ID);
};
exports.getBountyPDA = getBountyPDA;
const getDomainPDA = (platform, organization, team, domainType) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"),
        Buffer.from(platform),
        Buffer.from(organization),
        Buffer.from(team),
        Buffer.from(domainType)
    ], exports.BOUNTY_PROGRAM_ID);
};
exports.getDomainPDA = getDomainPDA;
const getEscrowPDA = (bounty) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"),
        bounty.toBuffer(),
    ], exports.BOUNTY_PROGRAM_ID);
};
exports.getEscrowPDA = getEscrowPDA;
const getRelayerPDA = (relayer) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"),
        relayer.toBuffer(),
    ], exports.BOUNTY_PROGRAM_ID);
};
exports.getRelayerPDA = getRelayerPDA;
/**
 * BountySdk provides methods to build
 * and interact with the Bounty Protocol
 */
class BountySdk {
    constructor(signer, user, connection) {
        this.signer = signer;
        this.user = user;
        this.connection = connection;
        /**
         * createVersionedTransaction takes a list of instructions and creates a versioned transaction
         *
         * @param ixs: instructions
         * @returns
         */
        this.createVersionedTransaction = (ixs) => __awaiter(this, void 0, void 0, function* () {
            const txMessage = yield new anchor.web3.TransactionMessage({
                instructions: ixs,
                recentBlockhash: (yield this.program.provider.connection.getLatestBlockhash()).blockhash,
                payerKey: this.signer,
            }).compileToV0Message();
            return new anchor.web3.VersionedTransaction(txMessage);
        });
        this.initializeProtocol = () => __awaiter(this, void 0, void 0, function* () {
            const protocolPda = getProtocolPDA();
            const sandMint = getSandMint();
            const initializeProtocolIx = yield this.program.methods.initialize().accounts({
                protocol: protocolPda[0],
                sandMint: sandMint[0],
            }).instruction();
            return {
                ix: initializeProtocolIx,
                protocolAccountPda: protocolPda[0],
                sandAccountMint: sandMint[0],
            };
        });
        this.initializeProtocolVtx = () => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.initializeProtocol()).ix]); });
        this.deactivateBountyDenomination = (mint) => __awaiter(this, void 0, void 0, function* () {
            const denominationPda = getDenominationPDA(mint);
            const deactivateBountyDenominationIx = yield this.program.methods.deactivateBountyDenomination().accounts({
                mint,
                denomination: denominationPda[0],
            }).instruction();
            return {
                ix: deactivateBountyDenominationIx,
                protocolAccountPda: getProtocolPDA()[0],
                denominationPda: denominationPda[0],
            };
        });
        this.deactivateBountyDenominationVtx = (mint) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.deactivateBountyDenomination(mint)).ix]); });
        this.createBounty = (id, bountyAmount, bountyCreator, mint) => __awaiter(this, void 0, void 0, function* () {
            const denominationPda = getDenominationPDA(mint);
            const bountyPda = getBountyPDA(id);
            const domainPda = getDomainPDA("github", "solana-labs", "bounty", "issue");
            const escrowPDA = getEscrowPDA(bountyPda[0]);
            const creatorAccount = yield (0, spl_token_1.getAssociatedTokenAddress)(mint, bountyCreator);
            const createBountyIx = yield this.program.methods.createBounty(id, bountyAmount).accounts({
                mint,
                bounty: bountyPda[0],
                domain: domainPda[0],
                creatorAccount,
                bountyDenomination: denominationPda[0],
                escrow: escrowPDA[0],
            }).instruction();
            return {
                ix: createBountyIx,
                bounty: bountyPda[0],
            };
        });
        this.createBountyVtx = ({ id, bountyAmount, bountyCreator, mint }) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.createBounty(id, bountyAmount, bountyCreator, mint)).ix]); });
        /**
         *
         * @param id
         * @param relayer
         * @param mint
         * @param solversWallets: this is the wallet address and thus the
         * @returns
         */
        this.completeBounty = ({ id, relayer, mint, solversWallets }) => __awaiter(this, void 0, void 0, function* () {
            // validate the solvers 
            if (solversWallets.length > 4) {
                throw new Error("Only 4 solvers can be added to a bounty");
            }
            const solverTokenAccounts = solversWallets.map((solver) => __awaiter(this, void 0, void 0, function* () {
                return yield (0, spl_token_1.getAssociatedTokenAddress)(mint, solver);
            })).reduce((acc, curr) => {
                const i = Object.keys(acc).length + 1;
                return Object.assign(Object.assign({}, acc), { [`solver${i}`]: curr });
            }, {});
            const protocolPda = getProtocolPDA();
            const feeCollector = getFeeCollectorPDA(mint);
            const bountyDenomination = getDenominationPDA(mint);
            const relayerPda = getRelayerPDA(relayer);
            const bountyPda = getBountyPDA(id);
            const escrowPDA = getEscrowPDA(bountyPda[0]);
            const completeBountyIx = yield this.program.methods.completeBounty().accounts(Object.assign({ protocol: protocolPda[0], feeCollector: feeCollector[0], bountyDenomination: bountyDenomination[0], relayer: relayerPda[0], bounty: bountyPda[0], escrow: escrowPDA[0] }, solverTokenAccounts)).instruction();
            return {
                ix: completeBountyIx,
                protocolAccountPda: getProtocolPDA()[0],
            };
        });
        this.completeBountyVtx = ({ id, relayer, mint, solversWallets }) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.completeBounty({ id, relayer, mint, solversWallets })).ix]); });
        this.addRelayer = (relayerAddress) => __awaiter(this, void 0, void 0, function* () {
            const protocolPda = getProtocolPDA();
            const relayerPda = getRelayerPDA(relayerAddress);
            const addRelayerIx = yield this.program.methods.addRelayer(relayerAddress).accounts({
                protocol: protocolPda[0],
                relayer: relayerPda[0],
            }).instruction();
            return {
                ix: addRelayerIx,
                protocolAccountPda: getProtocolPDA()[0],
                relayerPda: relayerPda[0],
            };
        });
        this.addRelayerVtx = (relayerAddress) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.addRelayer(relayerAddress)).ix]); });
        this.removeRelayer = (relayerAddress) => __awaiter(this, void 0, void 0, function* () {
            const protocolPda = getProtocolPDA();
            const relayerPda = getRelayerPDA(relayerAddress);
            const removeRelayerIx = yield this.program.methods.removeRelayer().accounts({
                protocol: protocolPda[0],
                relayer: relayerPda[0],
            }).instruction();
            return {
                ix: removeRelayerIx,
                protocolAccountPda: getProtocolPDA()[0],
                relayerPda: relayerPda[0],
            };
        });
        this.removeRelayerVtx = (relayerAddress) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.removeRelayer(relayerAddress)).ix]); });
        this.createDomain = ({ platform, organization, team, domainType }) => __awaiter(this, void 0, void 0, function* () {
            const domainPda = getDomainPDA(platform, organization, team, domainType);
            const protocolPda = getProtocolPDA();
            const createDomainIx = yield this.program.methods.createDomain(domainType, platform, organization, team).accounts({
                protocol: protocolPda[0],
                domain: domainPda[0],
            }).instruction();
            return {
                ix: createDomainIx,
                domainPda: domainPda[0],
            };
        });
        this.createDomainVtx = ({ platform, organization, team, domainType }) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.createDomain({ platform, organization, team, domainType })).ix]); });
        this.deactivateDomain = ({ platform, organization, team, domainType }) => __awaiter(this, void 0, void 0, function* () {
            const domainPda = getDomainPDA(platform, organization, team, domainType);
            const deactivateDomainIx = yield this.program.methods.deactivateDomain().accounts({
                domain: domainPda[0],
            }).instruction();
            return {
                ix: deactivateDomainIx,
                domainPda: domainPda[0],
            };
        });
        this.deactivateDomainVtx = ({ platform, organization, team, domainType }) => __awaiter(this, void 0, void 0, function* () { return this.createVersionedTransaction([(yield this.deactivateDomain({ platform, organization, team, domainType })).ix]); });
        this.program = BountySdk.setUpProgram({
            keypair: anchor.web3.Keypair.generate(),
            connection: connection
        });
    }
    static setUpProgram({ keypair, connection }) {
        const provider = new anchor.AnchorProvider(connection !== null && connection !== void 0 ? connection : new anchor.web3.Connection("https://api.solana.com"), new anchor.Wallet(keypair), {
            preflightCommitment: "recent",
            commitment: "confirmed",
        });
        return new anchor.Program(bounty_1.IDL, exports.BOUNTY_PROGRAM_ID, provider);
    }
}
exports.BountySdk = BountySdk;
//# sourceMappingURL=index.js.map