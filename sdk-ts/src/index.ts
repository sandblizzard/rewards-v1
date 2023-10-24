import * as anchor from "@coral-xyz/anchor"
import { Bounty, IDL } from "./idl/bounty"
import { getAssociatedTokenAddress, getMint } from '@solana/spl-token';

export * as utils from './utils';
export { Bounty }

export const BOUNTY_PROGRAM_ID = new anchor.web3.PublicKey("BoUNtye7MsbG3rWSXxgXTyWt2Q7veUrKwWeDJo7BED3e");

/**
 * getProtocolPDA 
 * @param programId 
 * @returns 
 */
const getProtocolPDA = () => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD")],
        BOUNTY_PROGRAM_ID
    )
}

const getSandMint = () => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("sand_mint")],
        BOUNTY_PROGRAM_ID
    )
}


const getDenominationPDA = (mint: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("DENOMINATION"), mint.toBuffer()],
        BOUNTY_PROGRAM_ID
    )
}

const getFeeCollectorPDA = (mint: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("FEE_COLLECTOR"), mint.toBuffer()],
        BOUNTY_PROGRAM_ID
    )
}

/**
 * getBountyPDA 
 * @param id: typically the internal id for the given domain. 
 *              e.g. for github issues it will be issueId
 * @returns 
 */
const getBountyPDA = (id: string) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from(id)],
        BOUNTY_PROGRAM_ID
    )
}

const getDomainPDA = ({
    platform,
    organization,
    team,
    domainType
}: { platform: string, organization: string, team: string, domainType: string }) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"),
        Buffer.from(platform),
        Buffer.from(organization),
        Buffer.from(team),
        Buffer.from(domainType)
        ],
        BOUNTY_PROGRAM_ID
    )
}

const getEscrowPDA = (bounty: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"),
        bounty.toBuffer(),
        ],
        BOUNTY_PROGRAM_ID
    )
}

const getRelayerPDA = (relayer: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"),
        relayer.toBuffer(),
        ],
        BOUNTY_PROGRAM_ID
    )
}

export {
    getProtocolPDA,
    getDenominationPDA,
    getBountyPDA,
    getDomainPDA,
    getEscrowPDA,
    getRelayerPDA,
    getFeeCollectorPDA,
    getSandMint
}
/**
 * BountySdk provides methods to build
 * and interact with the Bounty Protocol
 */
export class BountySdk {
    public program: anchor.Program<Bounty>;
    constructor(
        readonly signer: anchor.web3.PublicKey,
        readonly connection?: anchor.web3.Connection,
    ) {
        this.program = BountySdk.setUpProgram({
            keypair: anchor.web3.Keypair.generate(),
            connection: connection
        });
    }

    private static setUpProgram({
        keypair,
        connection
    }:
        {
            keypair: anchor.web3.Keypair,
            connection?: anchor.web3.Connection
        }) {
        const provider = new anchor.AnchorProvider(connection ?? new anchor.web3.Connection("https://api.solana.com"), new anchor.Wallet(keypair), {
            preflightCommitment: "recent",
            commitment: "confirmed",
        })
        return new anchor.Program<Bounty>(IDL, BOUNTY_PROGRAM_ID, provider);
    }

    /**
     * createVersionedTransaction takes a list of instructions and creates a versioned transaction
     *
     * @param ixs: instructions
     * @returns
     */
    createVersionedTransaction = async (
        ixs: anchor.web3.TransactionInstruction[], payer: anchor.web3.PublicKey = this.signer
    ) => {
        const txMessage = await new anchor.web3.TransactionMessage({
            instructions: ixs,
            recentBlockhash: (
                await this.program.provider.connection.getLatestBlockhash()
            ).blockhash,
            payerKey: payer,
        }).compileToV0Message();

        return new anchor.web3.VersionedTransaction(txMessage);
    };

    initializeProtocol = async () => {
        const protocolPda = getProtocolPDA();
        const sandMint = getSandMint();
        const initializeProtocolIx = await this.program.methods.initialize().accounts({
            protocolOwner: this.signer,
            protocol: protocolPda[0],
            sandMint: sandMint[0],
        }).instruction()

        return {
            vtx: this.createVersionedTransaction([initializeProtocolIx], this.signer),
            ix: initializeProtocolIx,
            protocolAccountPda: protocolPda[0],
            sandAccountMint: sandMint[0],
        }
    }


    deactivateBountyDenomination = async (mint: anchor.web3.PublicKey) => {
        const denominationPda = getDenominationPDA(mint);
        const deactivateBountyDenominationIx = await this.program.methods.deactivateBountyDenomination().accounts({
            mint,
            denomination: denominationPda[0],
        }).instruction();

        return {
            vtx: this.createVersionedTransaction([deactivateBountyDenominationIx]),
            ix: deactivateBountyDenominationIx,
            protocolAccountPda: getProtocolPDA()[0],
            denominationPda: denominationPda[0],
        }
    }

    private accountExists(account: anchor.web3.PublicKey) {
        return this.program.provider.connection.getAccountInfo(account);
    }

    createBounty = async ({
        id,
        bountyAmount,
        bountyCreator,
        mint,
        platform,
        organization,
        team,
        domainType
    }: { id: string, bountyAmount: anchor.BN, bountyCreator: anchor.web3.PublicKey, mint: anchor.web3.PublicKey, platform: string, organization: string, team: string, domainType: string }) => {
        const denominationPda = getDenominationPDA(mint);
        const transactionInstructions: anchor.web3.TransactionInstruction[] = [];
        if (!(await this.accountExists(denominationPda[0]))) {
            // create denomination 
            const createDenominationIx = (await this.addBountyDenomination({ mint })).ix;
            transactionInstructions.push(createDenominationIx)
        }
        const domainPda = getDomainPDA({ platform, organization, team, domainType });
        if (!(await this.accountExists(domainPda[0]))) {
            // create domain
            const createDomainIx = (await this.createDomain({
                platform,
                organization,
                team,
                domainType
            })).ix;
            transactionInstructions.push(createDomainIx)
        }

        const bountyPda = getBountyPDA(id);
        if (await this.accountExists(bountyPda[0])) {
            throw new Error(`Bounty account ${bountyPda[0]} already exists`)
        }
        const escrowPDA = getEscrowPDA(bountyPda[0]);
        if (await this.accountExists(escrowPDA[0])) {
            throw new Error(`Escrow account ${escrowPDA[0]} already exists`)
        }
        const creatorAccount = await getAssociatedTokenAddress(mint, bountyCreator)
        const createBountyIx = await this.program.methods.createBounty(id, bountyAmount).accounts({
            creator: bountyCreator,
            mint,
            bounty: bountyPda[0],
            domain: domainPda[0],
            creatorAccount,
            bountyDenomination: denominationPda[0],
            escrow: escrowPDA[0],
        }).instruction();
        transactionInstructions.push(createBountyIx)

        return {
            vtx: this.createVersionedTransaction(transactionInstructions, bountyCreator),
            ix: transactionInstructions,
            bounty: bountyPda[0],
        }
    }


    /**
     * 
     * @param id 
     * @param relayer 
     * @param mint 
     * @param solversWallets: this is the wallet address and thus the 
     * @returns 
     */
    completeBounty = async (
        {
            id,
            relayer,
            mint,
            completer,
            solversWallets
        }:
            { id: string, relayer?: anchor.web3.PublicKey, mint: anchor.web3.PublicKey, completer: anchor.web3.PublicKey, solversWallets: anchor.web3.PublicKey[] }) => {

        // validate the solvers 
        if (solversWallets.length > 4) {
            throw new Error("Only 4 solvers can be added to a bounty")
        }

        if (solversWallets.length === 0) {
            throw new Error("At least one solver must be added to a bounty")
        }
        const fullSolverWallets = (await Promise.all([null, null, null, null].map(async (_solver, idx) => {
            const address = solversWallets[idx] ? await getAssociatedTokenAddress(mint, solversWallets[idx]) : null
            return address
        })))

        const solverTokenAccounts = fullSolverWallets.reduce((acc, curr) => {
            const i = Object.keys(acc).length + 1;
            return {
                ...acc,
                [`solver${i}`]: curr
            }
        }, {})


        const protocolPda = getProtocolPDA();
        const feeCollector = getFeeCollectorPDA(mint);
        const bountyDenomination = getDenominationPDA(mint);
        const bountyPda = getBountyPDA(id);
        const escrowPDA = getEscrowPDA(bountyPda[0]);

        let completeBountyIx: anchor.web3.TransactionInstruction;
        if (relayer && (await this.accountExists(relayer))) {
            completeBountyIx = await this.program.methods.completeBountyAsRelayer().accounts({
                payer: completer,
                protocol: protocolPda[0],
                feeCollector: feeCollector[0],
                bountyDenomination: bountyDenomination[0],
                bounty: bountyPda[0],
                escrow: escrowPDA[0],
                relayer: relayer,
                ...solverTokenAccounts
            }).instruction();
        } else {
            completeBountyIx = await this.program.methods.completeBounty().accounts({
                payer: completer,
                protocol: protocolPda[0],
                feeCollector: feeCollector[0],
                bountyDenomination: bountyDenomination[0],
                bounty: bountyPda[0],
                escrow: escrowPDA[0],
                ...solverTokenAccounts
            }).instruction();
        }


        return {
            vtx: this.createVersionedTransaction([completeBountyIx], completer),
            ix: completeBountyIx,
            protocolAccountPda: getProtocolPDA()[0],
        }
    }



    addRelayer = async (relayerAddress: anchor.web3.PublicKey) => {
        const protocolPda = getProtocolPDA();
        const relayerPda = getRelayerPDA(relayerAddress);
        const addRelayerIx = await this.program.methods.addRelayer(relayerAddress).accounts({
            signer: this.signer,
            protocol: protocolPda[0],
            relayer: relayerPda[0],
        }).instruction();

        return {
            vtx: this.createVersionedTransaction([addRelayerIx], this.signer),
            ix: addRelayerIx,
            protocolAccountPda: getProtocolPDA()[0],
            relayerPda: relayerPda[0],
        }
    }


    removeRelayer = async (relayerAddress: anchor.web3.PublicKey) => {
        const protocolPda = getProtocolPDA();
        const relayerPda = getRelayerPDA(relayerAddress);
        const removeRelayerIx = await this.program.methods.removeRelayer().accounts({
            signer: this.signer,
            protocol: protocolPda[0],
            relayer: relayerPda[0],
        }).instruction();

        return {
            vtx: this.createVersionedTransaction([removeRelayerIx], this.signer),
            ix: removeRelayerIx,
            protocolAccountPda: getProtocolPDA()[0],
            relayerPda: relayerPda[0],
        }
    }


    createDomain = async ({
        platform,
        organization,
        team,
        domainType
    }: { platform: string, organization: string, team: string, domainType: string }) => {
        const domainPda = getDomainPDA({ platform, organization, team, domainType });
        const protocolPda = getProtocolPDA();
        const createDomainIx = await this.program.methods.createDomain(domainType, platform, organization, team).accounts({
            creator: this.signer,
            protocol: protocolPda[0],
            domain: domainPda[0],
        }).instruction();

        return {
            vtx: this.createVersionedTransaction([createDomainIx], this.signer),
            ix: createDomainIx,
            domainPda: domainPda[0],
        }
    }



    deactivateDomain = async (
        {
            platform,
            organization,
            team,
            domainType
        }: { platform: string, organization: string, team: string, domainType: string }) => {
        const domainPda = getDomainPDA({ platform, organization, team, domainType });
        const deactivateDomainIx = await this.program.methods.deactivateDomain().accounts({
            signer: this.signer,
            domain: domainPda[0],
        }).instruction();

        return {
            vtx: this.createVersionedTransaction([deactivateDomainIx], this.signer),
            ix: deactivateDomainIx,
            domainPda: domainPda[0],
        }
    }


    addBountyDenomination = async ({ mint }: { mint: anchor.web3.PublicKey }) => {
        const denominationPda = getDenominationPDA(mint);
        const protocolPda = getProtocolPDA();
        if (!(await this.accountExists(protocolPda[0]))) {
            throw new Error(`Protocol account ${protocolPda[0]} does not exist`)
        }

        if (await this.accountExists(denominationPda[0])) {
            throw new Error(`Denomination account ${denominationPda[0]} already exists`)
        }
        if (!(await getMint(this.connection, mint))) {
            throw new Error(`Mint ${mint} does not exist`)
        }
        const addBountyDenominationIx = await this.program.methods.addBountyDenomination().accounts({
            creator: this.signer,
            protocol: protocolPda[0],
            mint,
            denomination: denominationPda[0],
            feeCollector: getFeeCollectorPDA(mint)[0],
        }).instruction();

        return {
            vtx: this.createVersionedTransaction([addBountyDenominationIx]),
            ix: addBountyDenominationIx,
            protocolAccountPda: getProtocolPDA()[0],
            denominationPda: denominationPda[0],
        }
    }


}