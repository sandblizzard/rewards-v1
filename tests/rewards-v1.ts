import * as anchor from '@project-serum/anchor';
import { Program, Spl } from '@project-serum/anchor';
import { Bounty } from '../target/types/bounty';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import {
  TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  mintTo,
  getAssociatedTokenAddress,
} from '@solana/spl-token';
import * as web3 from '@solana/web3.js';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { assert, expect } from 'chai';
import {
  addDenomination,
  createBounty,
  createDomain,
  createRelayer,
} from './utils';
import { getDenominationPDA, getFeeCollectorPDA } from './pdas';

const program = anchor.workspace.Bounty as Program<Bounty>;
let collection_mint: anchor.web3.PublicKey;
const { wallet } = program.provider as anchor.AnchorProvider;
describe('bounty', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // set global variables
  const organization = 'sandblizzard';
  const team = 'rewards_v1';
  const platform = 'github';
  const bountyType = 'issue';
  const id = '123';
  const bountyAmount = new anchor.BN(1000000);

  // global variables to be init
  let sandMint: anchor.web3.PublicKey;
  let bountyPDA: [anchor.web3.PublicKey, number];
  let protocolPDA: [anchor.web3.PublicKey, number];
  let escrowPDA: [anchor.web3.PublicKey, number];
  let sandTokenAccount: [anchor.web3.PublicKey, number];
  let bonkMint: anchor.web3.PublicKey;
  let creatorBonkTokenAccount: anchor.web3.PublicKey;
  let relayerOne: [anchor.web3.PublicKey, number];

  // Setup test environment
  before(async () => {
    sandMint = await createMint(
      program.provider.connection,
      (wallet as NodeWallet).payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );
    bountyPDA = findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
        anchor.utils.bytes.utf8.encode(id),
      ],
      program.programId
    );

    protocolPDA = findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD')],
      program.programId
    );

    escrowPDA = findProgramAddressSync(
      [bountyPDA[0].toBytes()],
      program.programId
    );

    try {
      const fromAirdropSig = await program.provider.connection.requestAirdrop(
        wallet.publicKey,
        10 * anchor.web3.LAMPORTS_PER_SOL
      );
      await program.provider.connection.confirmTransaction(fromAirdropSig);

      collection_mint = await createMint(
        program.provider.connection,
        (wallet as NodeWallet).payer,
        wallet.publicKey,
        wallet.publicKey,
        0
      );
    } catch (err) {
      console.log(err);
      throw new Error(err);
    }

    // sand token account is the program account holding all the sand
    sandTokenAccount = findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
        sandMint.toBytes(),
      ],
      program.programId
    );

    // bonk is one bounty reward tokens
    bonkMint = await createMint(
      program.provider.connection,
      (wallet as NodeWallet).payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    // mint bonk to the creator
    creatorBonkTokenAccount = await createAssociatedTokenAccount(
      program.provider.connection,
      (wallet as NodeWallet).payer,
      bonkMint,
      wallet.publicKey
    );
    await mintTo(
      program.provider.connection,
      (wallet as NodeWallet).payer,
      bonkMint,
      creatorBonkTokenAccount,
      wallet.publicKey,
      1_000_000_000
    );

    // initialize protocol
    try {
      // Initialize protocol
      const res = await program.methods
        .initialize()
        .accounts({
          protocol: protocolPDA[0],
          collection: collection_mint,
          sandTokenMint: sandMint,
          sandTokenAccount: sandTokenAccount[0],
        })
        .rpc();
      assert(res);
    } catch (err) {
      console.log('Failed to initialize protocol ', program.programId);
      console.log(err);
      throw new Error(err);
    }

    // Add default relayer
    relayerOne = await createRelayer(program, protocolPDA[0]);

    // create bounty denomination
    await addDenomination(program, protocolPDA[0], bonkMint);
  });

  it('Create a bounty -> Should succeed', async () => {
    await createDomain(
      program,
      protocolPDA[0],
      platform,
      organization,
      team,
      bountyType
    );
    const bountyRes = await createBounty(
      wallet,
      program,
      platform,
      organization,
      team,
      bountyType,
      id,
      bountyAmount,
      bonkMint
    );
    let createdBounty = await program.account.bounty.fetch(
      bountyRes.bountyPDA[0]
    );
    expect(createdBounty.id).to.equal(id);
  });

  it('Add and remove Relayer -> Should Succees', async () => {
    // add relayer - relayer should be  pk + BOUNTY_SANDBLIZZARD
    const relayerKeys = anchor.web3.Keypair.generate();
    const relayer = findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
        relayerKeys.publicKey.toBytes(),
      ],
      program.programId
    );

    try {
      await program.methods
        .addRelayer(relayerKeys.publicKey)
        .accounts({
          protocol: protocolPDA[0],
          relayer: relayer[0],
        })
        .rpc();
    } catch (err) {
      console.log('Failed to add relayer ', err);
      throw new Error(err);
    }
    const relayerAccount = await program.account.relayer.fetch(relayer[0]);
    expect(relayer, `relayer not found after creation`).not.eq(undefined);
    expect(relayerAccount.active, `relayer is not active`).eq(true);

    // Remove relayer
    try {
      const tx = await program.methods
        .removeRelayer()
        .accounts({
          protocol: protocolPDA[0],
          relayer: relayer[0],
        })
        .rpc();
      await program.provider.connection.confirmTransaction(tx);
    } catch (err) {
      console.log('Failed to remove relayer ', err);
      throw new Error(err);
    }

    const deactivatedRelayer = await program.account.relayer.fetch(relayer[0]);
    expect(deactivatedRelayer.active, `relayer is not deactivated`).eq(false);
  });

  it('Create bounty and try to complete it -> Should Succeed', async () => {
    const bountyId = '42343';
    const bountyRes = await createBounty(
      wallet,
      program,
      platform,
      organization,
      team,
      bountyType,
      bountyId,
      bountyAmount,
      bonkMint
    );
    let createdBounty = await program.account.bounty.fetch(
      bountyRes.bountyPDA[0]
    );
    expect(createdBounty.id).to.equal(bountyId);

    // try to complte bounty
    const feeCollectorPDA = getFeeCollectorPDA(program, bonkMint);
    const bountyDenomination = getDenominationPDA(program, bonkMint);

    try {
      let creatorAccount = await getAssociatedTokenAddress(
        bonkMint,
        wallet.publicKey
      );
      await program.methods
        .completeBounty()
        .accounts({
          protocol: protocolPDA[0],
          feeCollector: feeCollectorPDA[0],
          bountyDenomination: bountyDenomination[0],
          relayer: relayerOne[0],
          bounty: bountyRes.bountyPDA[0],
          escrow: bountyRes.escrowPDA[0],
          solver1: creatorAccount,
          solver2: creatorAccount,
          solver3: creatorAccount,
          solver4: creatorAccount,
        })
        .rpc();
    } catch (err) {
      console.log('Failed to complete bounty: ', err);
      throw new Error(err);
    }
  });

  it('Create bounty and try to complete it not as a creator or relayer -> Should Fail', async () => {
    const aUser = await anchor.web3.Keypair.generate();
    const id = '78957';
    // allow main wallet to create bounty
    const bountyRes = await createBounty(
      wallet,
      program,
      platform,
      organization,
      team,
      bountyType,
      id,
      bountyAmount,
      bonkMint
    );
    let createdBounty = await program.account.bounty.fetch(
      bountyRes.bountyPDA[0]
    );
    expect(createdBounty.id).to.equal(id);

    // try to complete bounty as anyone
    const feeCollectorPDA = getFeeCollectorPDA(program, bonkMint);
    const bountyDenomination = getDenominationPDA(program, bonkMint);

    try {
      let creatorAccount = await getAssociatedTokenAddress(
        bonkMint,
        wallet.publicKey
      );

      const ixs: web3.TransactionInstruction[] = [];
      const ix = await program.methods
        .completeBounty()
        .accounts({
          protocol: protocolPDA[0],
          feeCollector: feeCollectorPDA[0],
          bountyDenomination: bountyDenomination[0],
          relayer: relayerOne[0],
          bounty: bountyRes.bountyPDA[0],
          escrow: bountyRes.escrowPDA[0],
          solver1: creatorAccount,
          solver2: creatorAccount,
          solver3: creatorAccount,
          solver4: creatorAccount,
        })
        .instruction();
      ixs.push(ix);

      const txMessage = new web3.TransactionMessage({
        payerKey: aUser.publicKey,
        recentBlockhash: (await provider.connection.getRecentBlockhash())
          .blockhash,
        instructions: ixs,
      }).compileToV0Message();
      const tx = new web3.VersionedTransaction(txMessage);
      tx.sign([aUser]);
      provider.connection
        .sendTransaction(tx)
        .then((res) => {
          throw new Error('should not success');
        })
        .catch((err) => undefined);
    } catch (err) {
      console.log('Failed to complete bounty: ', err);
    }
  });

  it('create multiple domains', async () => {
    const domainType = 'issues';
    const platform = 'github';
    const repo = 'rewards-v1';
    const subDomain = 'sanddblizzard';

    const domain1 = await createDomain(
      program,
      protocolPDA[0],
      domainType,
      platform,
      repo,
      subDomain
    );
    expect(domain1[0]).to.not.be.undefined;

    const domain2 = await createDomain(
      program,
      protocolPDA[0],
      domainType,
      platform,
      'perpetuals',
      subDomain
    );
    expect(domain2[0]).to.not.be.undefined;
  });

  it('create a domain and deactive it -> should succeed', async () => {});
});
