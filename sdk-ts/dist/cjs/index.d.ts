import * as anchor from "@coral-xyz/anchor";
import { Bounty } from "./idl/bounty";
export * as utils from './utils';
export { Bounty };
export declare const BOUNTY_PROGRAM_ID: anchor.web3.PublicKey;
/**
 * getProtocolPDA
 * @param programId
 * @returns
 */
declare const getProtocolPDA: () => [anchor.web3.PublicKey, number];
declare const getSandMint: () => [anchor.web3.PublicKey, number];
declare const getDenominationPDA: (mint: anchor.web3.PublicKey) => [anchor.web3.PublicKey, number];
declare const getFeeCollectorPDA: (mint: anchor.web3.PublicKey) => [anchor.web3.PublicKey, number];
/**
 * getBountyPDA
 * @param id: typically the internal id for the given domain.
 *              e.g. for github issues it will be issueId
 * @returns
 */
declare const getBountyPDA: (id: string) => [anchor.web3.PublicKey, number];
declare const getDomainPDA: (platform: string, organization: string, team: string, domainType: string) => [anchor.web3.PublicKey, number];
declare const getEscrowPDA: (bounty: anchor.web3.PublicKey) => [anchor.web3.PublicKey, number];
declare const getRelayerPDA: (relayer: anchor.web3.PublicKey) => [anchor.web3.PublicKey, number];
export { getProtocolPDA, getDenominationPDA, getBountyPDA, getDomainPDA, getEscrowPDA, getRelayerPDA, getFeeCollectorPDA, getSandMint };
/**
 * BountySdk provides methods to build
 * and interact with the Bounty Protocol
 */
export declare class BountySdk {
    readonly signer: anchor.web3.PublicKey;
    readonly user: anchor.web3.PublicKey;
    readonly connection?: anchor.web3.Connection;
    program: anchor.Program<Bounty>;
    constructor(signer: anchor.web3.PublicKey, user: anchor.web3.PublicKey, connection?: anchor.web3.Connection);
    private static setUpProgram;
    /**
     * createVersionedTransaction takes a list of instructions and creates a versioned transaction
     *
     * @param ixs: instructions
     * @returns
     */
    createVersionedTransaction: (ixs: anchor.web3.TransactionInstruction[]) => Promise<anchor.web3.VersionedTransaction>;
    initializeProtocol: () => Promise<{
        ix: anchor.web3.TransactionInstruction;
        protocolAccountPda: anchor.web3.PublicKey;
        sandAccountMint: anchor.web3.PublicKey;
    }>;
    initializeProtocolVtx: () => Promise<anchor.web3.VersionedTransaction>;
    deactivateBountyDenomination: (mint: anchor.web3.PublicKey) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        protocolAccountPda: anchor.web3.PublicKey;
        denominationPda: anchor.web3.PublicKey;
    }>;
    deactivateBountyDenominationVtx: (mint: anchor.web3.PublicKey) => Promise<anchor.web3.VersionedTransaction>;
    createBounty: (id: string, bountyAmount: anchor.BN, bountyCreator: anchor.web3.PublicKey, mint: anchor.web3.PublicKey) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        bounty: anchor.web3.PublicKey;
    }>;
    createBountyVtx: ({ id, bountyAmount, bountyCreator, mint }: {
        id: string;
        bountyAmount: anchor.BN;
        bountyCreator: anchor.web3.PublicKey;
        mint: anchor.web3.PublicKey;
    }) => Promise<anchor.web3.VersionedTransaction>;
    /**
     *
     * @param id
     * @param relayer
     * @param mint
     * @param solversWallets: this is the wallet address and thus the
     * @returns
     */
    completeBounty: ({ id, relayer, mint, solversWallets }: {
        id: string;
        relayer: anchor.web3.PublicKey;
        mint: anchor.web3.PublicKey;
        solversWallets: anchor.web3.PublicKey[];
    }) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        protocolAccountPda: anchor.web3.PublicKey;
    }>;
    completeBountyVtx: ({ id, relayer, mint, solversWallets }: {
        id: string;
        relayer: anchor.web3.PublicKey;
        mint: anchor.web3.PublicKey;
        solversWallets: anchor.web3.PublicKey[];
    }) => Promise<anchor.web3.VersionedTransaction>;
    addRelayer: (relayerAddress: anchor.web3.PublicKey) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        protocolAccountPda: anchor.web3.PublicKey;
        relayerPda: anchor.web3.PublicKey;
    }>;
    addRelayerVtx: (relayerAddress: anchor.web3.PublicKey) => Promise<anchor.web3.VersionedTransaction>;
    removeRelayer: (relayerAddress: anchor.web3.PublicKey) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        protocolAccountPda: anchor.web3.PublicKey;
        relayerPda: anchor.web3.PublicKey;
    }>;
    removeRelayerVtx: (relayerAddress: anchor.web3.PublicKey) => Promise<anchor.web3.VersionedTransaction>;
    createDomain: ({ platform, organization, team, domainType }: {
        platform: string;
        organization: string;
        team: string;
        domainType: string;
    }) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        domainPda: anchor.web3.PublicKey;
    }>;
    createDomainVtx: ({ platform, organization, team, domainType }: {
        platform: string;
        organization: string;
        team: string;
        domainType: string;
    }) => Promise<anchor.web3.VersionedTransaction>;
    deactivateDomain: ({ platform, organization, team, domainType }: {
        platform: string;
        organization: string;
        team: string;
        domainType: string;
    }) => Promise<{
        ix: anchor.web3.TransactionInstruction;
        domainPda: anchor.web3.PublicKey;
    }>;
    deactivateDomainVtx: ({ platform, organization, team, domainType }: {
        platform: string;
        organization: string;
        team: string;
        domainType: string;
    }) => Promise<anchor.web3.VersionedTransaction>;
}
//# sourceMappingURL=index.d.ts.map