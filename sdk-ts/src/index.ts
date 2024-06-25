import { BN, web3, Program, AnchorProvider } from "@coral-xyz/anchor";
import { Bounty, IDL } from "./idl/bounty"
import { getAssociatedTokenAddress, getMint } from '@solana/spl-token';
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet.js";
import { getOrCreateAssociatedTokenAccountIx } from "./utils";

export * as utils from './utils';
export { Bounty }

const METADATA_SEED = "metadata";

const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
export const BOUNTY_PROGRAM_ID = new web3.PublicKey("5Hwbrh6QMrHvBNZfYXmsktWtfohcSSCMaC5Er9ErwNoQ");

/**
 * getProtocolPDA 
 * @param programId 
 * @returns 
 */
const getProtocolPDA = () => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD")],
        BOUNTY_PROGRAM_ID
    )
}

const getSandMint = () => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("sand_mint")],
        BOUNTY_PROGRAM_ID
    )
}


const getDenominationPDA = (mint: web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("DENOMINATION"), mint.toBuffer()],
        BOUNTY_PROGRAM_ID
    )
}

const getFeeCollectorPDA = (mint: web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), Buffer.from("FEE_COLLECTOR"), mint.toBuffer()],
        BOUNTY_PROGRAM_ID
    )
}

const getSolverPDA = (solver: web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), solver.toBuffer()],
        BOUNTY_PROGRAM_ID
    )
}

const getSolverTokenAccount = (solver: web3.PublicKey, mint: web3.PublicKey) => {
    return getAssociatedTokenAddress(mint, solver)
}


const getMetadataAddress = (mint: web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(METADATA_SEED),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
    );
}




/**
 * getBountyPDA 
 * @param id: typically the internal id for the given domain. 
 *              e.g. for github issues it will be issueId
 * @returns 
 */
const getBountyPDA = (id: number) => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"), new BN(id).toBuffer('le', 8)],
        BOUNTY_PROGRAM_ID
    )
}

const getDomainPDA = ({
    platform,
    organization,
    team,
    domainType
}: { platform: string, organization: string, team: string, domainType: string }) => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"),
        Buffer.from(platform),
        Buffer.from(organization),
        Buffer.from(team),
        Buffer.from(domainType)
        ],
        BOUNTY_PROGRAM_ID
    )
}

const getEscrowPDA = (bounty: web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BOUNTY_SANDBLIZZARD"),
        bounty.toBuffer(),
        ],
        BOUNTY_PROGRAM_ID
    )
}

const getRelayerPDA = (relayer: web3.PublicKey) => {
    return web3.PublicKey.findProgramAddressSync(
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
    getSandMint,
    getSolverPDA
}
/**
 * BountySdk provides methods to build
 * and interact with the Bounty Protocol
 */
export class BountySdk {
    public program: Program<Bounty>;
    readonly connection: web3.Connection
    constructor(
        readonly signer: web3.PublicKey,
        protoConnection?: web3.Connection,
    ) {
        const connection = protoConnection ?? new web3.Connection("https://api.solana.com");
        this.program = BountySdk.setUpProgram({
            keypair: web3.Keypair.generate(),
            connection: connection
        });
        this.connection = connection;
    }

    private static setUpProgram({
        keypair,
        connection
    }:
        {
            keypair: web3.Keypair,
            connection?: web3.Connection
        }) {
        const provider = new AnchorProvider(connection ?? new web3.Connection("https://api.solana.com"), new NodeWallet(keypair), {
            preflightCommitment: "recent",
            commitment: "confirmed",
        })
        return new Program<Bounty>(IDL, BOUNTY_PROGRAM_ID, provider);
    }

    /**
     * createVersionedTransaction takes a list of instructions and creates a versioned transaction
     *
     * @param ixs: instructions
     * @returns
     */
    createVersionedTransaction = async (
        ixs: web3.TransactionInstruction[], payer: web3.PublicKey = this.signer
    ) => {
        const txMessage = await new web3.TransactionMessage({
            instructions: ixs,
            recentBlockhash: (
                await this.program.provider.connection.getLatestBlockhash()
            ).blockhash,
            payerKey: payer,
        }).compileToV0Message();

        return new web3.VersionedTransaction(txMessage);
    };

    initializeProtocol = async () => {
        const protocolPda = getProtocolPDA();
        const sandMint = getSandMint();
        const metadataAddress = getMetadataAddress(sandMint[0]);

        const initializeProtocolIx = await this.program.methods.initialize().accounts({
            protocolOwner: this.signer,
            metadata: metadataAddress[0],
            protocol: protocolPda[0],
            sandMint: sandMint[0],
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            rentSysvar: web3.SYSVAR_RENT_PUBKEY,
        }).instruction()

        return {
            vtx: this.createVersionedTransaction([initializeProtocolIx], this.signer),
            ix: initializeProtocolIx,
            protocolAccountPda: protocolPda[0],
            sandAccountMint: sandMint[0],
        }
    }

    registerSolver = async (solver: web3.PublicKey) => {
        const protocolPda = getProtocolPDA();
        const sandMint = getSandMint();
        const solverPda = getSolverPDA(solver);
        const registerSolverIx = await this.program.methods.registerSolver().accounts({
            signer: solver,
            protocol: protocolPda[0],
            solverAccount: solverPda[0],
            sandMint: sandMint[0],
            solverTokenAccount: await getSolverTokenAccount(solver, sandMint[0]),
        }).instruction();

        return {
            vtx: await this.createVersionedTransaction([registerSolverIx], solver),
            ix: registerSolverIx,
            protocolAccountPda: protocolPda[0],
            solverPda: solverPda[0],
        }
    }

    getSolverAccount = async (solver: web3.PublicKey) => {
        const solverPda = getSolverPDA(solver);
        return this.program.account.solver.fetch(solverPda[0]);
    }

    claimReward = async (solver: web3.PublicKey) => {
        const protocolPda = getProtocolPDA();
        const sandMint = getSandMint();
        const solverPda = getSolverPDA(solver);
        const claimRewardsIx = await this.program.methods.claimRewards().accounts({
            signer: solver,
            protocol: protocolPda[0],
            solver: solverPda[0],
            solverTokenAccount: await getSolverTokenAccount(solver, sandMint[0]),
            mint: sandMint[0],
        }).instruction();

        return {
            vtx: await this.createVersionedTransaction([claimRewardsIx], solver),
            ix: claimRewardsIx,
            protocolAccountPda: protocolPda[0],
            solverPda: solverPda[0],
        }
    }


    deactivateBountyDenomination = async (mint: web3.PublicKey) => {
        const denominationPda = getDenominationPDA(mint);
        const deactivateBountyDenominationIx = await this.program.methods.deactivateBountyDenomination().accounts({
            mint,
            denomination: denominationPda[0],
        }).instruction();

        return {
            vtx: await this.createVersionedTransaction([deactivateBountyDenominationIx]),
            ix: deactivateBountyDenominationIx,
            protocolAccountPda: getProtocolPDA()[0],
            denominationPda: denominationPda[0],
        }
    }

    private accountExists(account: web3.PublicKey) {
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
    }: { id: number, bountyAmount: BN, bountyCreator: web3.PublicKey, mint: web3.PublicKey, platform: string, organization: string, team: string, domainType: string }) => {
        const denominationPda = getDenominationPDA(mint);
        const transactionInstructions: web3.TransactionInstruction[] = [];
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
        // create ata if not exist 
        if (!(await this.accountExists(creatorAccount))) {
            const createCreatorAccountIx = await getOrCreateAssociatedTokenAccountIx(this.connection, bountyCreator, mint, bountyCreator);
            if (createCreatorAccountIx.instruction) {
                transactionInstructions.push(createCreatorAccountIx.instruction)
            }
        }
        const createBountyIx = await this.program.methods.createBounty(new BN(id), bountyAmount).accounts({
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
            vtx: await this.createVersionedTransaction(transactionInstructions, bountyCreator),
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
            { id: number, relayer?: web3.PublicKey, mint: web3.PublicKey, completer: web3.PublicKey, solversWallets: web3.PublicKey[] }) => {

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

        const sandMint = getSandMint();
        const solverAccounts = await Promise.all([null, null, null, null].map(async (_solver, idx) => {
            const address = solversWallets[idx] ? await getSolverPDA(solversWallets[idx])[0] : null
            return address
        }))

        const solverTokenAccounts = fullSolverWallets.reduce((acc, curr) => {
            const i = Object.keys(acc).length + 1;
            return {
                ...acc,
                [`solverTokenAccount${i}`]: curr
            }
        }, {})

        const solvers = solverAccounts.reduce((acc, curr) => {
            const i = Object.keys(acc).length + 1;
            return {
                ...acc,
                [`solver${i}`]: curr
            }
        }, {})


        const protocolPda = getProtocolPDA();
        const feeCollector = getFeeCollectorPDA(mint);
        const bountyPda = getBountyPDA(id);
        const escrowPDA = getEscrowPDA(bountyPda[0]);
        const sandMintPDA = getSandMint();

        let completeBountyIx: web3.TransactionInstruction;
        if (relayer && (await this.accountExists(relayer))) {
            completeBountyIx = await this.program.methods.completeBountyAsRelayer().accounts({
                payer: completer,
                protocol: protocolPda[0],
                sandMint: sandMintPDA[0],
                feeCollector: feeCollector[0],
                bounty: bountyPda[0],
                escrow: escrowPDA[0],
                ...solverTokenAccounts,
                ...solvers,
                relayer: relayer,
            }).instruction();
        } else {
            completeBountyIx = await this.program.methods.completeBounty().accounts({
                payer: completer,
                protocol: protocolPda[0],
                sandMint: sandMintPDA[0],
                feeCollector: feeCollector[0],
                bounty: bountyPda[0],
                escrow: escrowPDA[0],
                ...solverTokenAccounts,
                ...solvers,
            }).instruction();
        }


        return {
            vtx: await this.createVersionedTransaction([completeBountyIx], completer),
            ix: completeBountyIx,
            protocolAccountPda: getProtocolPDA()[0],
        }
    }

    // create instructions to create PDAs 
    getOrCreateAssociatedTokenAccountsIxs = async (
        {
            mint,
            payer,
            solverWallets,

        }: { mint: web3.PublicKey, payer: web3.PublicKey, solverWallets: web3.PublicKey[] }) => {
        let solvers = (await Promise.all(solverWallets.map(async (solver) => {
            return getOrCreateAssociatedTokenAccountIx(this.connection, payer, mint, solver)
        }))).filter((solver) => {
            return solver.instruction !== null
        })
        return solvers;
    }


    addRelayer = async (relayerAddress: web3.PublicKey) => {
        const protocolPda = getProtocolPDA();
        const relayerPda = getRelayerPDA(relayerAddress);
        const addRelayerIx = await this.program.methods.addRelayer(relayerAddress).accounts({
            signer: this.signer,
            protocol: protocolPda[0],
            relayer: relayerPda[0],
        }).instruction();

        return {
            vtx: await this.createVersionedTransaction([addRelayerIx], this.signer),
            ix: addRelayerIx,
            protocolAccountPda: getProtocolPDA()[0],
            relayerPda: relayerPda[0],
        }
    }


    removeRelayer = async (relayerAddress: web3.PublicKey) => {
        const protocolPda = getProtocolPDA();
        const relayerPda = getRelayerPDA(relayerAddress);
        const removeRelayerIx = await this.program.methods.removeRelayer().accounts({
            signer: this.signer,
            protocol: protocolPda[0],
            relayer: relayerPda[0],
        }).instruction();

        return {
            vtx: await this.createVersionedTransaction([removeRelayerIx], this.signer),
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
            vtx: await this.createVersionedTransaction([createDomainIx], this.signer),
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
            vtx: await this.createVersionedTransaction([deactivateDomainIx], this.signer),
            ix: deactivateDomainIx,
            domainPda: domainPda[0],
        }
    }


    addBountyDenomination = async ({ mint }: { mint: web3.PublicKey }) => {
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
            vtx: await this.createVersionedTransaction([addBountyDenominationIx]),
            ix: addBountyDenominationIx,
            protocolAccountPda: getProtocolPDA()[0],
            denominationPda: denominationPda[0],
        }
    }

    getAllBounties = async () => {
        return this.program.account.bounty.all();
    }

    getAllBountiesByUser = async (address: web3.PublicKey) => {
        const memcmpFilters = [
            {
                memcmp: {
                    offset: 8,
                    bytes: address.toBase58()
                }
            }
        ]

        return await this.program.account.bounty.all(memcmpFilters)
    }

    getDomain = async ({ address }: { address: web3.PublicKey }) => {
        return this.program.account.domain.fetch(address)
    }




}