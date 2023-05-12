import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import * as anchor from '@project-serum/anchor';
import { Bounty } from '../target/types/bounty';
import { PublicKey } from '@solana/web3.js';

/**
 * get domain PDA
 * @param program
 * @param platform
 * @param organization
 * @param team
 * @param domainType
 * @returns
 */
export const getDomainPDA = (
  program: anchor.Program<Bounty>,
  platform: string,
  organization: string,
  team: string,
  domainType: string
) => {
  return findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
      anchor.utils.bytes.utf8.encode(platform),
      anchor.utils.bytes.utf8.encode(organization),
      anchor.utils.bytes.utf8.encode(team),
      anchor.utils.bytes.utf8.encode(domainType),
    ],
    program.programId
  );
};

/**
 * getDenominationPDA
 * @param program
 * @param mint
 * @returns
 */
export const getDenominationPDA = (
  program: anchor.Program<Bounty>,
  mint: PublicKey
) => {
  return findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
      anchor.utils.bytes.utf8.encode('DENOMINATION'),
      mint.toBytes(),
    ],
    program.programId
  );
};

export const getFeeCollectorPDA = (
  program: anchor.Program<Bounty>,
  mint: PublicKey
) => {
  return findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode('BOUNTY_SANDBLIZZARD'),
      anchor.utils.bytes.utf8.encode('FEE_COLLECTOR'),
      mint.toBytes(),
    ],
    program.programId
  );
};
