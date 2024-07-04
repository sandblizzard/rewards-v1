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
import { getProtocolPDA, getSandMint, getSolverPDA } from "../sdk-ts/src";
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
  const id = Math.floor(Math.random() * 1000000);
  const bountyAmount = new anchor.BN(1000000);

  // global variables to be init
  let bonkMint: anchor.web3.PublicKey;
  let creatorBonkTokenAccount: anchor.web3.PublicKey;

  // Setup test environment
  before(async () => {
    try {

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

      // register solvers 
      console.log("Registering solvers...")
      const initializeSolver = await bountySdk.registerSolver(wallet.publicKey);
      await sendAndConfirmTransaction(provider.connection, await initializeSolver.vtx, [wallet.payer])
      const initializeSolver2 = await bountySdk.registerSolver(user.publicKey);
      await sendAndConfirmTransaction(provider.connection, await initializeSolver2.vtx, [userWallet.payer])

      console.log("Finished setting up the test environment..")
    }
    catch (e) {
      console.log("error", e)
      throw e
    }
  });


  it('Create a bounty -> Should succeed', async () => {
    // create bounty 
    try {
      const createBounty = await bountySdk.createBounty(
        {
          id,
          bountyCreator: wallet.publicKey,
          mint: bonkMint,
          organization,
          team,
          platform,
          domainType: 'issues'
        }
      );
      await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])


      // DOnate to the bounty
      const donateToBounty = await bountySdk.donateToBounty({
        bountyId: id,
        mint: bonkMint,
        amount: bountyAmount,
        payer: wallet.publicKey
      });
      await sendAndConfirmTransaction(provider.connection, await donateToBounty.vtx, [wallet.payer])



      let createdBounty = await program.account.bounty.fetch(
        createBounty.bounty
      );

      expect(Buffer.from(createdBounty.idBytes).readBigInt64LE()).to.equal(BigInt(id));
      // check that the bountyAmount is correct
      // check that donate amount is of length 1
      console.log("Created Bounty", JSON.stringify(createdBounty))
      expect(createdBounty.donateAmount[0].toString()).to.equal(bountyAmount.toString());
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

  it.only('Create bounty and try to complete it as a relayer -> Should Succeed', async () => {
    const bountyId = Math.floor(Math.random() * 1000000);
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
      bountyCreator: user.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [userWallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(Buffer.from(createdBounty.idBytes).readBigInt64LE()).to.equal(BigInt(bountyId));

    /// propose solution to bounty
    const proposeSolution = await bountySdk.proposeSolution(
      {
        bountyId,
        solution: "Just a solution",
        solver: userWallet.publicKey
      }
    );

    await sendAndConfirmTransaction(provider.connection, await proposeSolution.vtx, [userWallet.payer])
    // check that the solution was proposed
    const bountySolution = await program.account.bountySolution.fetch(
      proposeSolution.bountySolution
    );
    const bountyAccount2 = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(bountySolution.bounty.toString()).to.equal(createBounty.bounty.toString());
    expect(bountySolution.solution).to.equal("Just a solution");
    expect(bountyAccount2.solverSolutions.length).to.equal(1);
    expect(bountyAccount2.solverSolutions[0].toString()).to.equal(proposeSolution.bountySolution.toString());


    // try to complete bounty
    console.log("Completing bounty with id", bountyId, "...")
    const completeBounty = await bountySdk.completeBounty(
      {
        id: bountyId,
        relayer: getRelayerPDA(relayerWallet.publicKey)[0],
        completer: relayerWallet.publicKey,
        mint: bonkMint,
        solversWallets: [userWallet.publicKey]
      }
    );
    await sendAndConfirmTransaction(provider.connection, await completeBounty.vtx, [relayerWallet.payer])
    const bountyAccount = await program.account.bounty.fetch(
      createBounty.bounty,
      'confirmed'
    );
    console.log("Bounty completed by", JSON.stringify(bountyAccount))
    expect(bountyAccount.completedBy.toString()).to.equal(relayerWallet.publicKey.toString());
    expect(bountyAccount.state.completed).to.exist
    expect(bountyAccount.solvedBy.length).to.equal(1);
    expect(bountyAccount.solvedBy[0].toString()).to.equal(userWallet.publicKey.toString());

    // get sand token balance of user
    const sandMintPda = await getSandMint();
    const sandMint = await spl.getMint(provider.connection, sandMintPda[0]);

    const sandTokenAccount = await getAssociatedTokenAddress(
      sandMintPda[0],
      userWallet.publicKey,
    );
    const sandTokenBalance = await provider.connection.getTokenAccountBalance(sandTokenAccount);
    expect(sandTokenBalance.value.amount).to.equal('0');

    // check the claimable amount of the user
    const claimableAmount = await getSolverPDA(userWallet.publicKey);
    const claimableAmountInfo = await program.account.solver.fetch(claimableAmount[0]);
    const claimableAmountValue = claimableAmountInfo.claimableRewards.toString();
    const protocolPda = await getProtocolPDA();
    const protocolAccount = await program.account.protocol.fetch(protocolPda[0]);


    // test the claimable amount being equal to the 10.pow(sandMint.decimals) * protocolAccount.emission
    const tokenDecimals = new anchor.BN(10).pow(new anchor.BN(sandMint.decimals));
    const expectedAmount = protocolAccount.emission.mul(tokenDecimals);
    expect(claimableAmountValue).to.equal(expectedAmount.toString());

    // try to claim the amount
    const claimAmount = await bountySdk.claimReward(
      userWallet.publicKey
    );
    await sendAndConfirmTransaction(provider.connection, await claimAmount.vtx, [userWallet.payer])
    // refetch the sand token balance
    const sandTokenBalanceAfter = await provider.connection.getTokenAccountBalance(sandTokenAccount);
    expect(sandTokenBalanceAfter.value.amount).to.equal(expectedAmount.toString());


  });

  it('Create bounty and try to complete it as the creator -> Should Succeed', async () => {
    const id = Math.floor(Math.random() * 1000000);
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyCreator: userWallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [userWallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(Buffer.from(createdBounty.idBytes).readBigInt64LE()).to.equal(BigInt(id));
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
    const id = Math.floor(Math.random() * 1000000);
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    });
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(Buffer.from(createdBounty.idBytes).readBigInt64LE()).to.equal(BigInt(id));

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
    const id = Math.floor(Math.random() * 1000000);
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    });
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(Buffer.from(createdBounty.idBytes).readBigInt64LE()).to.equal(BigInt(id));

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
    const id = Math.floor(Math.random() * 1000000);
    // allow main wallet to create bounty
    const createBounty = await bountySdk.createBounty({
      id,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])

    let createdBounty = await program.account.bounty.fetch(
      createBounty.bounty
    );
    expect(Buffer.from(createdBounty.idBytes).readBigInt64LE()).to.equal(BigInt(id));

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
    const id = Math.floor(Math.random() * 1000000);
    const organization = 'sandblizzard_test_doesnt_exist';
    const createBounty = await bountySdk.createBounty({
      id,
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    )
    await sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])
  });

  it('try to create a bounty with a domain that is not active -> should fail', async () => {
    const id = Math.floor(Math.random() * 1000000);
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
      bountyCreator: wallet.publicKey,
      mint: bonkMint,
      organization, team, platform, domainType: 'issues'
    }
    );
    await expect(sendAndConfirmTransaction(provider.connection, await createBounty.vtx, [wallet.payer])).to.be.rejectedWith(Error)
  })

  it('try to get all bounties from one user -> should succeed', async () => {
    const bounties = await bountySdk.getAllBountiesByUser(
      wallet.publicKey
    );
    expect(bounties.length).to.equal(5);
  });

});
