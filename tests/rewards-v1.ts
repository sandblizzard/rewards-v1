import * as anchor from "@coral-xyz/anchor"
import { BountySdk, BOUNTY_PROGRAM_ID, getFeeCollectorPDA, getDenominationPDA, getRelayerPDA, Bounty, getDomainPDA } from '../sdk-ts/dist/cjs';
import {
  TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createMint,
  mintTo,
  getAssociatedTokenAddress,
} from '@solana/spl-token';
import * as spl from "@solana/spl-token"
import * as web3 from '@solana/web3.js';
import { assert, config, expect, use } from 'chai';
import * as chaiAsPromised from "chai-as-promised"
import { sendAndConfirmTransaction } from "../sdk-ts/src/utils";
use(chaiAsPromised.default)

/**
 * topUpAccount is a helper function to top up an account with SOL
 * @param connection 
 * @param wallet 
 * @returns 
 */
const topUpAccount = async (connection: anchor.web3.Connection, wallet: anchor.Wallet) => {
  const latestBlockhash = await connection.getLatestBlockhash();

  const fromAirdropSig = await connection.requestAirdrop(
    wallet.publicKey,
    10 * anchor.web3.LAMPORTS_PER_SOL
  );
  return await connection.confirmTransaction({
    signature: fromAirdropSig,
    ...latestBlockhash
  });
}



describe('bounty', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const keypair = web3.Keypair.generate();
  const user = web3.Keypair.generate();

  const wallet = new anchor.Wallet(keypair);
  const userWallet = new anchor.Wallet(user);

  anchor.setProvider(provider);
  const program = anchor.workspace.Bounty as anchor.Program<Bounty>;
  const bountySdk = new BountySdk(
    wallet.publicKey,
    provider.connection
  );

  // set global variables
  const organization = 'sandblizzard';
  const team = 'rewards_v1';
  const platform = 'github';
  const id = Math.floor(Math.random() * 1000000).toString();
  const bountyAmount = new anchor.BN(1000000);

  // global variables to be init
  let bonkMint: anchor.web3.PublicKey;
  let creatorBonkTokenAccount: anchor.web3.PublicKey;

  // Setup test environment
  before(async () => {
    const latestBlockhash = await provider.connection.getLatestBlockhash();

    await topUpAccount(provider.connection, wallet);
    await topUpAccount(provider.connection, userWallet);
    console.log("Airdrop done...")


    // bonk is one bounty reward tokens
    bonkMint = await createMint(
      provider.connection,
      (wallet).payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    // mint bonk to the creator
    const protocolOwnerBonkTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      (wallet).payer,
      bonkMint,
      wallet.publicKey
    );

    const userBonkTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      (userWallet).payer,
      bonkMint,
      userWallet.publicKey
    );
    await spl.mintTo(
      provider.connection,
      (wallet).payer,
      bonkMint,
      protocolOwnerBonkTokenAccount,
      wallet.publicKey,
      1_000_000_000
    );
    console.log("Minted bonk to owner...")


    await spl.transfer(
      provider.connection,
      (wallet).payer,
      protocolOwnerBonkTokenAccount,
      userBonkTokenAccount,
      wallet.publicKey,
      500_000_000
    );
    console.log("Minted bonk to user...")

    console.log("Initializes Protocol...")
    const initializeProtocol = await bountySdk.initializeProtocol();
    await sendAndConfirmTransaction(provider.connection, await initializeProtocol.vtx, [wallet.payer])

    console.log("Initializes Fee Collector...")
    const initDenomination = await bountySdk.addBountyDenomination(
      {
        mint: bonkMint,
      }
    );
    await sendAndConfirmTransaction(provider.connection, await initDenomination.vtx, [wallet.payer])

    // create domain 
    console.log("Creates Domain...")
    const createDomain = await bountySdk.createDomain(
      {
        platform,
        organization,
        team,
        domainType: 'issues'
      }
    );
    await sendAndConfirmTransaction(provider.connection, await createDomain.vtx, [wallet.payer])

    console.log("Finished setting up the test environment..")
  });

  it('Create a bounty -> Should succeed', async () => {
    // create bounty 
    try {
      const createBounty = await bountySdk.createBounty(
        {
          id,
          bountyAmount: bountyAmount,
          bountyCreator: wallet.publicKey,
          mint: bonkMint,
          organization,
          team,
          platform,
          domainType: 'issues'
        }
      );
      await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

      let createdBounty = await program.account.bounty.fetch(
        createBounty.bounty
      );
      expect(createdBounty.id).to.equal(id);
    } catch (e) {
      console.log("error", e)
      throw e
    }

  });



  it('Add and remove Relayer -> Should Succees', async () => {
    // add relayer - relayer should be  pk + BOUNTY_SANDBLIZZARD
    const relayerKeys = web3.Keypair.generate();

    // create relayer 
    const createRelayer = await bountySdk.addRelayer(
      relayerKeys.publicKey,
    );
    await sendAndConfirmTransaction(provider.connection, await createRelayer.vtx, [wallet.payer])

    console.log("Get relayer account from key", relayerKeys.publicKey.toString())
    const relayerAccount = await program.account.relayer.fetch(createRelayer.relayerPda);
    expect(relayerAccount, `relayer not found after creation`).not.eq(undefined);
    expect(relayerAccount.active, `relayer is not active`).eq(true);

    // Remove relayer
    const removeRelayer = await bountySdk.removeRelayer(
      relayerKeys.publicKey,
    );
    await sendAndConfirmTransaction(provider.connection, await removeRelayer.vtx, [wallet.payer])

    const deactivatedRelayer = await program.account.relayer.fetch(createRelayer.relayerPda);
    expect(deactivatedRelayer.active, `relayer is not deactivated`).eq(false);
  });

  it('Create bounty and try to complete it as a relayer -> Should Succeed', async () => {
    const bountyId = Math.floor(Math.random() * 1000000).toString();
    // add relayer - relayer should be  pk + BOUNTY_SANDBLIZZARD
    const relayerKeys = web3.Keypair.generate();
    const relayerWallet = new anchor.Wallet(relayerKeys);
    await topUpAccount(provider.connection, relayerWallet);

    // create relayer 
    const createRelayer = await bountySdk.addRelayer(
      relayerKeys.publicKey,
    );
    await sendAndConfirmTransaction(provider.connection, await createRelayer.vtx, [wallet.payer])

    console.log("Creating bounty with id", bountyId, "...")
    const createBounty = await bountySdk.createBounty({
      id: bountyId,
      bountyAmount,
      bountyCreator: user.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [userWallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(bountyId);

    // try to complete bounty
    console.log("Completing bounty with id", bountyId, "...")
    const completeBounty = await bountySdk.completeBounty(
      {
        id: bountyId,
        relayer: getRelayerPDA(relayerWallet.publicKey)[0],
        completer: relayerWallet.publicKey,
        mint: bonkMint,
        solversWallets: [user.publicKey]
      }
    );
    await sendAndConfirmTransaction(provider.connection, await completeBounty.vtx, [relayerWallet.payer])
    const bountyAccount = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(bountyAccount.completedBy.toString()).to.equal(relayerWallet.publicKey.toString());
    expect(bountyAccount.state.completed).to.exist
  });

  it('Create bounty and try to complete it as the creator -> Should Succeed', async () => {
    const id = Math.floor(Math.random() * 1000000).toString();
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyAmount,
      bountyCreator: userWallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [userWallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(id);
    expect(createdBounty.state.created).to.exist

    // try to complete bounty as anyone
    const completeBounty = await bountySdk.completeBounty(
      {
        id,
        mint: bonkMint,
        completer: user.publicKey,
        solversWallets: [user.publicKey]
      }
    );

    await sendAndConfirmTransaction(provider.connection, await completeBounty.vtx, [userWallet.payer])
    const bountyAccount = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(bountyAccount.completedBy.toString()).to.equal(user.publicKey.toString());
    expect(bountyAccount.state.completed).to.exist;
  });

  it('Create bounty and try to complete it non creator -> Should Fail', async () => {
    const id = Math.floor(Math.random() * 1000000).toString();
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyAmount,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    });
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(id);

    // try to complete bounty as anyone
    const completeBounty = await bountySdk.completeBounty(
      {
        id,
        mint: bonkMint,
        completer: user.publicKey,
        solversWallets: [user.publicKey]
      }
    );

    await expect(sendAndConfirmTransaction(provider.connection, await completeBounty.vtx, [userWallet.payer])).to.be.rejectedWith(Error)
  });

  it('Create bounty and complete it with no solver -> Should Succeed', async () => {
    const id = Math.floor(Math.random() * 1000000).toString();
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyAmount,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    });
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(id);

    // try to complete bounty as anyone
    const completeBounty = await bountySdk.completeBounty(
      {
        id,
        mint: bonkMint,
        completer: wallet.publicKey,
        solversWallets: [user.publicKey]
      }
    );

    await sendAndConfirmTransaction(provider.connection, await completeBounty.vtx, [wallet.payer])
    const bountyAccount = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(bountyAccount.completedBy.toString()).to.equal(wallet.publicKey.toString());
    expect(bountyAccount.state.completed).to.exist;
  })

  it('Create bounty and try to complete it with a relayer that is not active -> Should Fail', async () => {
    const id = Math.floor(Math.random() * 1000000).toString();
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyAmount,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(createdBounty.id).to.equal(id);

    // add relayer - relayer should be  pk + BOUNTY_SANDBLIZZARD
    const relayerKeys = web3.Keypair.generate();
    const relayerWallet = new anchor.Wallet(relayerKeys);
    await topUpAccount(provider.connection, relayerWallet);

    // create relayer 
    const createRelayer = await bountySdk.addRelayer(
      relayerKeys.publicKey,
    );
    await sendAndConfirmTransaction(provider.connection, await createRelayer.vtx, [wallet.payer])

    // deactivate relayer
    const deactivateRelayer = await bountySdk.removeRelayer(
      relayerKeys.publicKey,
    );
    await sendAndConfirmTransaction(provider.connection, await deactivateRelayer.vtx, [wallet.payer])

    // try to complete bounty as anyone
    const completeBounty = await bountySdk.completeBounty(
      {
        id,
        relayer: getRelayerPDA(relayerWallet.publicKey)[0],
        mint: bonkMint,
        completer: relayerWallet.publicKey,
        solversWallets: [user.publicKey]
      }
    );

    await expect(sendAndConfirmTransaction(provider.connection, await completeBounty.vtx, [relayerWallet.payer])).to.be.rejectedWith(Error)
  })

  it('try to create same domain twice -> should fail', async () => {
    const domainType = 'issues';
    const platform = 'github';
    const organization = 'sandblizzard_test';
    const domain1 = await bountySdk.createDomain(
      {
        platform,
        organization,
        team,
        domainType
      }
    );
    await sendAndConfirmTransaction(provider.connection, await domain1.vtx, [wallet.payer])
    const domain1Account = await program.account.domain.fetch(
      domain1.domainPda
    );
    expect(domain1Account.data.domainType).to.equal(domainType);
    expect(domain1Account.data.organization).to.equal(organization);
    expect(domain1Account.data.team).to.equal(team);
    expect(domain1Account.data.platform).to.equal(platform);

    const domain2 = await bountySdk.createDomain(
      {
        platform,
        organization,
        team,
        domainType
      }
    );
    await expect(sendAndConfirmTransaction(provider.connection, await domain2.vtx, [wallet.payer])).to.be.rejectedWith(Error)
  });

  it("try to create a bounty with a domain that doesn't exist -> should succeed", async () => {
    const id = Math.floor(Math.random() * 1000000).toString();
    const organization = 'sandblizzard_test_doesnt_exist';
    const createBounty = await bountySdk.createBounty({
      id,
      bountyAmount,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    )
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])
  });

  it('try to create a bounty with a domain that is not active -> should fail', async () => {
    const id = Math.floor(Math.random() * 1000000).toString();
    const organization = 'sandblizzard_test_inactive';
    // create domain
    const createDomain = await bountySdk.createDomain(
      {
        platform,
        organization,
        team,
        domainType: 'issues'
      }
    );
    await sendAndConfirmTransaction(provider.connection, await createDomain.vtx, [wallet.payer])

    // deactivate domain
    const deactivateDomain = await bountySdk.deactivateDomain(
      {
        platform,
        organization,
        team,
        domainType: 'issues'
      }
    );
    await sendAndConfirmTransaction(provider.connection, await deactivateDomain.vtx, [wallet.payer])

    // try to create bounty with inactive domain
    const createBounty = await bountySdk.createBounty({
      id,
      bountyAmount,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await expect(sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])).to.be.rejectedWith(Error)
  })

});
