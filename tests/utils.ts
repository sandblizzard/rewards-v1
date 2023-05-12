import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { Bounty } from '../target/types/bounty';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import * as spl from '@solana/spl-token';
import { Wallet } from '@project-serum/anchor/dist/cjs/provider';
import { getOrCreateAssociatedTokenAccountIx } from '../app/src/helper';
/**
 *
 * @param program
 * @param protocolPDA
 * @returns
 */
export const createRelayer = async (
  program: anchor.Program<Bounty>,
  protocolPDA: PublicKey
) => {
  const relayerKeys = anchor.web3.Keypair.generate();
  const relayerOne = findProgramAddressSync(
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
        protocol: protocolPDA,
        relayer: relayerOne[0],
      })
      .rpc();
    console.log('Successfully added relayer!');
    return relayerOne;
  } catch (err) {
    console.log('Failed to add relayer ', err);
    throw new Error(err);
  }
};

/**
 * createDomain - creates a domain
 * @param program
 * @param domain
 * @param subDomain
 * @param id
 * @returns
 */
export const createDomain = async (
  program: anchor.Program<Bounty>,
  protocolPDA: PublicKey,
  domainType: string,
  platform: string,
  repo: string,
  subDomain: string
) => {
  const domainPDA = findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
      anchor.utils.bytes.utf8.encode(platform),
      anchor.utils.bytes.utf8.encode(subDomain),
      anchor.utils.bytes.utf8.encode(domainType),
      anchor.utils.bytes.utf8.encode(repo),
    ],
    program.programId
  );

  try {
    await program.methods
      .createDomain(domainType, platform, repo, subDomain)
      .accounts({
        protocol: protocolPDA,
        domain: domainPDA[0],
      })
      .rpc();
    console.log('Successfully created domain!');
    return domainPDA;
  } catch (err) {
    console.log('Failed to create domain ', err);
    throw new Error(err);
  }
};

/**
 * createBounty
 */
export const createBounty = async (
  wallet: Wallet,
  program: anchor.Program<Bounty>,
  domain: string,
  subDomain: string,
  id: string,
  bountyAmount: anchor.BN,
  bountyMint: PublicKey
) => {
  const bountyPDA = findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
      anchor.utils.bytes.utf8.encode(domain),
      anchor.utils.bytes.utf8.encode(subDomain),
      anchor.utils.bytes.utf8.encode(id),
    ],
    program.programId
  );

  const escrowPDA = findProgramAddressSync(
    [bountyPDA[0].toBytes()],
    program.programId
  );

  const creatorBountyTokenAccount = await spl.getAssociatedTokenAddress(
    bountyMint,
    wallet.publicKey
  );

  try {
    await program.methods
      .createBounty(id, bountyAmount)
      .accounts({
        bounty: bountyPDA[0],
        creatorAccount: creatorBountyTokenAccount,
        mint: bountyMint,
        escrow: escrowPDA[0],
      })
      .rpc();
    return { bountyPDA, escrowPDA };
  } catch (err) {
    console.log(`failed to create bounty: cause: ${err}`);
    throw new Error(err);
  }
};
