import * as anchor from "@coral-xyz/anchor"
import { BountySdk, BOUNTY_PROGRAM_ID, getFeeCollectorPDA, getDenominationPDA, getRelayerPDA, Bounty } from '../sdk-ts/dist/cjs';
import {
  TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  mintTo,
  getAssociatedTokenAddress,
} from '@solana/spl-token';
import * as web3 from '@solana/web3.js';
import { assert, expect } from 'chai';


let collection_mint: anchor.web3.PublicKey;
describe('bounty', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const keypair = web3.Keypair.generate();
  const user = web3.Keypair.generate();

  const wallet = new anchor.Wallet(keypair);

  const userWallet = new anchor.Wallet(user);
  anchor.setProvider(provider);
  const program = anchor.workspace.Solstreams as anchor.Program<Bounty>;
  const bountySdk = new BountySdk(
    wallet.publicKey,
    user.publicKey,
    provider.connection
  );

  // set global variables
  const organization = 'sandblizzard';
  const team = 'rewards_v1';
  const platform = 'github';
  const id = '123';
  const bountyAmount = new anchor.BN(1000000);

  // global variables to be init
  let bonkMint: anchor.web3.PublicKey;
  let creatorBonkTokenAccount: anchor.web3.PublicKey;

  // Setup test environment
  before(async () => {

    try {
      const fromAirdropSig = await provider.connection.requestAirdrop(
        wallet.publicKey,
        10 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(fromAirdropSig);

      collection_mint = await createMint(
        provider.connection,
        (wallet).payer,
        wallet.publicKey,
        wallet.publicKey,
        0
      );
    } catch (err) {
      console.log(err);
      throw new Error(err);
    }

    // bonk is one bounty reward tokens
    bonkMint = await createMint(
      provider.connection,
      (wallet).payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    // mint bonk to the creator
    creatorBonkTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      (wallet).payer,
      bonkMint,
      wallet.publicKey
    );
    await mintTo(
      provider.connection,
      (wallet).payer,
      bonkMint,
      creatorBonkTokenAccount,
      wallet.publicKey,
      1_000_000_000
    );

  });

  it('Create a bounty -> Should succeed', async () => {
    const bountyId = "4433"
    const ixs = new Array<web3.TransactionInstruction>();
    const createDomainVtx = await bountySdk.createDomainVtx(
      {
        platform,
        organization,
        team,
        domainType: 'issues'
      }
    );
    createDomainVtx.sign([wallet.payer])
    const sig = await provider.connection.sendTransaction(createDomainVtx);

    const createBountyVtx = await bountySdk.createBountyVtx(
      {
        id: bountyId,
        bountyAmount: bountyAmount,
        bountyCreator: user.publicKey,
        mint: bonkMint

      }
    );
    createBountyVtx.sign([wallet.payer])
    const txSig = await provider.connection.sendTransaction(createBountyVtx);


    // let createdBounty = await program.account.bounty.fetch(
    //   createBounty.bounty
    // );
    // expect(createdBounty.id).to.equal(id);
  });

  it('Add and remove Relayer -> Should Succees', async () => {
    // add relayer - relayer should be  pk + BOUNTY_SANDBLIZZARD
    const relayerKeys = web3.Keypair.generate();
    const createRelayer = await bountySdk.addRelayer(
      relayerKeys.publicKey,
    );
    const createRelayerIx = createRelayer.ix
    const tx = new web3.Transaction().add(createRelayerIx);
    const signedTx = await wallet.signTransaction(tx);
    const txSig = await provider.sendAndConfirm(signedTx);


    const relayerAccount = await program.account.relayer.fetch(createRelayer.relayerPda[0]);
    expect(relayerAccount, `relayer not found after creation`).not.eq(undefined);
    expect(relayerAccount.active, `relayer is not active`).eq(true);

    // Remove relayer
    const removeRelayer = await bountySdk.removeRelayer(
      relayerKeys.publicKey,
    );
    const removeRelayerIx = removeRelayer.ix
    const tx2 = new web3.Transaction().add(removeRelayerIx);
    const signedTx2 = await wallet.signTransaction(tx2);
    const txSig2 = await provider.sendAndConfirm(signedTx2);


    const deactivatedRelayer = await program.account.relayer.fetch(createRelayer.relayerPda[0]);
    expect(deactivatedRelayer.active, `relayer is not deactivated`).eq(false);
  });

  it('Create bounty and try to complete it -> Should Succeed', async () => {
    const bountyId = '42343';
    const createBounty = await bountySdk.createBounty(
      bountyId,
      bountyAmount,
      user.publicKey,
      bonkMint
    );
    const createBountyIx = createBounty.ix
    const tx = new web3.Transaction().add(createBountyIx);
    const signedTx = await wallet.signTransaction(tx);
    const txSig = await provider.sendAndConfirm(signedTx);

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(bountyId);

    // try to complete bounty
    const feeCollectorPDA = getFeeCollectorPDA(bonkMint)
    const bountyDenomination = getDenominationPDA(bonkMint);

    const completeBounty = await bountySdk.completeBounty(
      {
        id: bountyId,
        relayer: getRelayerPDA(wallet.publicKey)[0],
        mint: bonkMint,
        solversWallets: [user.publicKey]
      }
    );
    const completeBountyIx = completeBounty.ix
    const tx2 = new web3.Transaction().add(completeBountyIx);
    const signedTx2 = await wallet.signTransaction(tx2);
    const txSig2 = await provider.sendAndConfirm(signedTx2);

  });

  it('Create bounty and try to complete it not as a creator or relayer -> Should Fail', async () => {
    const aUser = await anchor.web3.Keypair.generate();
    const id = '78957';
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty(
      id,
      bountyAmount,
      user.publicKey,
      bonkMint
    );
    const createBountyIx = createBounty.ix
    const tx = new web3.Transaction().add(createBountyIx);
    const signedTx = await wallet.signTransaction(tx);
    const txSig = await provider.sendAndConfirm(signedTx);

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(id);

    // try to complete bounty as anyone
    const feeCollectorPDA = getFeeCollectorPDA(bonkMint)
    const bountyDenomination = getDenominationPDA(bonkMint);


    const completeBounty = await bountySdk.completeBounty(
      {
        id,
        relayer: getRelayerPDA(user.publicKey)[0], // user is not a relayer
        mint: bonkMint,
        solversWallets: [user.publicKey]
      }
    );
  });

  it('create multiple domains', async () => {
    const domainType = 'issues';
    const platform = 'github';
    const repo = 'rewards-v1';
    const subDomain = 'sanddblizzard';

    const domain1 = await bountySdk.createDomain(
      {
        platform,
        organization,
        team,
        domainType
      }
    );

    expect(domain1.domainPda).to.not.be.undefined;

    const domain2 = await bountySdk.createDomain(
      {
        platform,
        organization,
        team,
        domainType
      }
    );
    expect(domain2.domainPda).to.not.be.undefined;
  });

  it('create a domain and deactive it -> should succeed', async () => { });
});
