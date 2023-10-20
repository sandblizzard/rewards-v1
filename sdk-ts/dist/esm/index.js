import * as anchor from "@coral-xyz/anchor";
import { IDL } from "./idl/bounty";
import { getAssociatedTokenAddress } from '@solana/spl-token';
export * as utils from './utils';
export const BOUNTY_PROGRAM_ID = new anchor.web3.PublicKey("74cnoYJmzNmGLVwj1k88eGRXMZ6srVnNt32dn7qiivpU");
/**
 * getProtocolPDA
 * @param programId
 * @returns
 */
const getProtocolPDA = () => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD")], BOUNTY_PROGRAM_ID);
};
const getSandMint = () => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("sand_mint")], BOUNTY_PROGRAM_ID);
};
const getDenominationPDA = (mint) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("DENOMINATION"), mint.toBuffer()], BOUNTY_PROGRAM_ID);
};
const getFeeCollectorPDA = (mint) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("FEE_COLLECTOR"), mint.toBuffer()], BOUNTY_PROGRAM_ID);
};
/**
 * getBountyPDA
 * @param id: typically the internal id for the given domain.
 *              e.g. for github issues it will be issueId
 * @returns
 */
const getBountyPDA = (id) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from(id)], BOUNTY_PROGRAM_ID);
};
const getDomainPDA = (platform, organization, team, domainType) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"),
        Buffer.from(platform),
        Buffer.from(organization),
        Buffer.from(team),
        Buffer.from(domainType)
    ], BOUNTY_PROGRAM_ID);
};
const getEscrowPDA = (bounty) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"),
        bounty.toBuffer(),
    ], BOUNTY_PROGRAM_ID);
};
const getRelayerPDA = (relayer) => {
    return anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("BOUNTY_SANDBLIZZARD"),
        relayer.toBuffer(),
    ], BOUNTY_PROGRAM_ID);
};
export { getProtocolPDA, getDenominationPDA, getBountyPDA, getDomainPDA, getEscrowPDA, getRelayerPDA, getFeeCollectorPDA, getSandMint };
/**
 * BountySdk provides methods to build
 * and interact with the Bounty Protocol
 */
export class BountySdk {
    signer;
    user;
    connection;
    program;
    constructor(signer, user, connection) {
        this.signer = signer;
        this.user = user;
        this.connection = connection;
        this.program = BountySdk.setUpProgram({
            keypair: anchor.web3.Keypair.generate(),
            connection: connection
        });
    }
    static setUpProgram({ keypair, connection }) {
        const provider = new anchor.AnchorProvider(connection ?? new anchor.web3.Connection("https://api.solana.com"), new anchor.Wallet(keypair), {
            preflightCommitment: "recent",
            commitment: "confirmed",
        });
        return new anchor.Program(IDL, BOUNTY_PROGRAM_ID, provider);
    }
    /**
     * createVersionedTransaction takes a list of instructions and creates a versioned transaction
     *
     * @param ixs: instructions
     * @returns
     */
    createVersionedTransaction = async (ixs) => {
        const txMessage = await new anchor.web3.TransactionMessage({
            instructions: ixs,
            recentBlockhash: (await this.program.provider.connection.getLatestBlockhash()).blockhash,
            payerKey: this.signer,
        }).compileToV0Message();
        return new anchor.web3.VersionedTransaction(txMessage);
    };
    initializeProtocol = async () => {
        const protocolPda = getProtocolPDA();
        const sandMint = getSandMint();
        const initializeProtocolIx = await this.program.methods.initialize().accounts({
            protocol: protocolPda[0],
            sandMint: sandMint[0],
        }).instruction();
        return {
            ix: initializeProtocolIx,
            protocolAccountPda: protocolPda[0],
            sandAccountMint: sandMint[0],
        };
    };
    initializeProtocolVtx = async () => this.createVersionedTransaction([(await this.initializeProtocol()).ix]);
    deactivateBountyDenomination = async (mint) => {
        const denominationPda = getDenominationPDA(mint);
        const deactivateBountyDenominationIx = await this.program.methods.deactivateBountyDenomination().accounts({
            mint,
            denomination: denominationPda[0],
        }).instruction();
        return {
            ix: deactivateBountyDenominationIx,
            protocolAccountPda: getProtocolPDA()[0],
            denominationPda: denominationPda[0],
        };
    };
    deactivateBountyDenominationVtx = async (mint) => this.createVersionedTransaction([(await this.deactivateBountyDenomination(mint)).ix]);
    createBounty = async (id, bountyAmount, bountyCreator, mint) => {
        const denominationPda = getDenominationPDA(mint);
        const bountyPda = getBountyPDA(id);
        const domainPda = getDomainPDA("github", "solana-labs", "bounty", "issue");
        const escrowPDA = getEscrowPDA(bountyPda[0]);
        const creatorAccount = await getAssociatedTokenAddress(mint, bountyCreator);
        const createBountyIx = await this.program.methods.createBounty(id, bountyAmount).accounts({
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
    };
    createBountyVtx = async ({ id, bountyAmount, bountyCreator, mint }) => this.createVersionedTransaction([(await this.createBounty(id, bountyAmount, bountyCreator, mint)).ix]);
    /**
     *
     * @param id
     * @param relayer
     * @param mint
     * @param solversWallets: this is the wallet address and thus the
     * @returns
     */
    completeBounty = async ({ id, relayer, mint, solversWallets }) => {
        // validate the solvers 
        if (solversWallets.length > 4) {
            throw new Error("Only 4 solvers can be added to a bounty");
        }
        const solverTokenAccounts = solversWallets.map(async (solver) => {
            return await getAssociatedTokenAddress(mint, solver);
        }).reduce((acc, curr) => {
            const i = Object.keys(acc).length + 1;
            return {
                ...acc,
                [`solver${i}`]: curr
            };
        }, {});
        const protocolPda = getProtocolPDA();
        const feeCollector = getFeeCollectorPDA(mint);
        const bountyDenomination = getDenominationPDA(mint);
        const relayerPda = getRelayerPDA(relayer);
        const bountyPda = getBountyPDA(id);
        const escrowPDA = getEscrowPDA(bountyPda[0]);
        const completeBountyIx = await this.program.methods.completeBounty().accounts({
            protocol: protocolPda[0],
            feeCollector: feeCollector[0],
            bountyDenomination: bountyDenomination[0],
            relayer: relayerPda[0],
            bounty: bountyPda[0],
            escrow: escrowPDA[0],
            ...solverTokenAccounts
        }).instruction();
        return {
            ix: completeBountyIx,
            protocolAccountPda: getProtocolPDA()[0],
        };
    };
    completeBountyVtx = async ({ id, relayer, mint, solversWallets }) => this.createVersionedTransaction([(await this.completeBounty({ id, relayer, mint, solversWallets })).ix]);
    addRelayer = async (relayerAddress) => {
        const protocolPda = getProtocolPDA();
        const relayerPda = getRelayerPDA(relayerAddress);
        const addRelayerIx = await this.program.methods.addRelayer(relayerAddress).accounts({
            protocol: protocolPda[0],
            relayer: relayerPda[0],
        }).instruction();
        return {
            ix: addRelayerIx,
            protocolAccountPda: getProtocolPDA()[0],
            relayerPda: relayerPda[0],
        };
    };
    addRelayerVtx = async (relayerAddress) => this.createVersionedTransaction([(await this.addRelayer(relayerAddress)).ix]);
    removeRelayer = async (relayerAddress) => {
        const protocolPda = getProtocolPDA();
        const relayerPda = getRelayerPDA(relayerAddress);
        const removeRelayerIx = await this.program.methods.removeRelayer().accounts({
            protocol: protocolPda[0],
            relayer: relayerPda[0],
        }).instruction();
        return {
            ix: removeRelayerIx,
            protocolAccountPda: getProtocolPDA()[0],
            relayerPda: relayerPda[0],
        };
    };
    removeRelayerVtx = async (relayerAddress) => this.createVersionedTransaction([(await this.removeRelayer(relayerAddress)).ix]);
    createDomain = async ({ platform, organization, team, domainType }) => {
        const domainPda = getDomainPDA(platform, organization, team, domainType);
        const protocolPda = getProtocolPDA();
        const createDomainIx = await this.program.methods.createDomain(domainType, platform, organization, team).accounts({
            protocol: protocolPda[0],
            domain: domainPda[0],
        }).instruction();
        return {
            ix: createDomainIx,
            domainPda: domainPda[0],
        };
    };
    createDomainVtx = async ({ platform, organization, team, domainType }) => this.createVersionedTransaction([(await this.createDomain({ platform, organization, team, domainType })).ix]);
    deactivateDomain = async ({ platform, organization, team, domainType }) => {
        const domainPda = getDomainPDA(platform, organization, team, domainType);
        const deactivateDomainIx = await this.program.methods.deactivateDomain().accounts({
            domain: domainPda[0],
        }).instruction();
        return {
            ix: deactivateDomainIx,
            domainPda: domainPda[0],
        };
    };
    deactivateDomainVtx = async ({ platform, organization, team, domainType }) => this.createVersionedTransaction([(await this.deactivateDomain({ platform, organization, team, domainType })).ix]);
}
//# sourceMappingURL=index.js.map