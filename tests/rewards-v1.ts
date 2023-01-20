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
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { assert, expect } from 'chai';
import { getOrCreateAssociatedTokenAccountIx } from '../app/src/helper';

const program = anchor.workspace.Bounty as Program<Bounty>;
let collection_mint: anchor.web3.PublicKey;
let bonk_mint: anchor.web3.PublicKey;
describe('bounty', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const { wallet } = program.provider as anchor.AnchorProvider;
  it('Is initialized!', async () => {
    // Add your test here.
    const domain = 'sandblizzard';
    const subDomain = 'rewards_v1';
    const id = '123';
    const bountyAmount = new anchor.BN(1000000);
    const feeCollector = anchor.web3.Keypair.generate();
    const bountyPDA = findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
        anchor.utils.bytes.utf8.encode(domain),
        anchor.utils.bytes.utf8.encode(subDomain),
        anchor.utils.bytes.utf8.encode(id),
      ],
      program.programId
    );

    const protocolPDA = findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD')],
      program.programId
    );

    const escrowPDA = findProgramAddressSync(
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
      console.log('Failed to initialize test ', program.programId);
      console.log(err);
    }

    try {
      // initialize

      // Initialize protocol
      const res = await program.methods
        .initialize()
        .accounts({
          protocol: protocolPDA[0],
          feeCollector: feeCollector.publicKey,
          collection: collection_mint,
        })
        .rpc();
      assert(res);
    } catch (err) {
      console.log('Failed to initialize protocol ', program.programId);
      console.log(err);
    }

    // CREATE_BOUNTY
    try {
      // FIXME: find an ok mint

      bonk_mint = await createMint(
        program.provider.connection,
        (wallet as NodeWallet).payer,
        wallet.publicKey,
        wallet.publicKey,
        6
      );

      const creatorAccount = await createAssociatedTokenAccount(
        program.provider.connection,
        (wallet as NodeWallet).payer,
        bonk_mint,
        wallet.publicKey
      );
      await mintTo(
        program.provider.connection,
        (wallet as NodeWallet).payer,
        bonk_mint,
        creatorAccount,
        wallet.publicKey,
        1_000_000_000
      );

      console.log('bountyPDA[0] ', bountyPDA[0].toString());
      const resCreateBounty = await program.methods
        .createBounty(domain, subDomain, id, bountyAmount)
        .accounts({
          bounty: bountyPDA[0],
          creatorAccount,
          mint: bonk_mint,
          escrow: escrowPDA[0],
        })
        .rpc();
      let createBounty = await program.account.bounty.fetch(bountyPDA[0]);

      console.log('bounty: ', createBounty);
      assert(createBounty.state === 'started');
    } catch (err) {
      console.log('TOKEN_PROGRAM_ID_ ', TOKEN_PROGRAM_ID);
      console.log('Failed to create bounty');
      console.log(err);
    }
    // add relayer

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

    // try to complte bounty

    const feeCollectorAccount = await createAssociatedTokenAccount(
      program.provider.connection,
      (wallet as NodeWallet).payer,
      bonk_mint,
      feeCollector.publicKey
    );
    try {
      let creatorAccount = await getAssociatedTokenAddress(
        bonk_mint,
        wallet.publicKey
      );
      await program.methods
        .completeBounty()
        .accounts({
          protocol: protocolPDA[0],
          feeCollector: feeCollectorAccount,
          relayer: relayer[0],
          bounty: bountyPDA[0],
          escrow: escrowPDA[0],
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
});
