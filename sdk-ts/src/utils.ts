import * as spl from '@solana/spl-token';
import type { Connection, PublicKey, Commitment } from '@solana/web3.js';
import { TransactionInstruction } from '@solana/web3.js';
import * as web3 from "@solana/web3.js"

/**
 * sendAndConfirmTransaction is a simple wrapper around web3's sendAndConfirmTransaction
 * @param connection 
 * @param transaction 
 * @param latestBlockhash 
 * @param signers 
 * @returns 
 */
export const sendAndConfirmTransaction = async (
  connection: web3.Connection,
  transaction: web3.VersionedTransaction,

  signers?: web3.Signer[],
  latestBlockhash?: {
    blockhash: string;
    lastValidBlockHeight: number;
  },

) => {
  try {
    if (!latestBlockhash) {
      latestBlockhash = await connection.getLatestBlockhash();
    }
    if (signers && signers.length !== 0) {
      transaction.sign(signers);
    }
    const signature = await connection.sendTransaction(transaction, {
      skipPreflight: false,
    });
    const confirmation = await connection.confirmTransaction({
      signature: signature,
      ...latestBlockhash
    });
    return {
      confirmation,
      signature
    }
  } catch (err) {
    console.log("err", err)
    throw err
  }
}

export const getOrCreateAssociatedTokenAccountIx = async (
  connection: Connection,
  payer: PublicKey,
  mint: PublicKey,
  owner: PublicKey,
  allowOwnerOffCurve = false,
  commitment: Commitment = 'finalized',
  programId = spl.TOKEN_PROGRAM_ID,
  associatedTokenProgramId = spl.ASSOCIATED_TOKEN_PROGRAM_ID
): Promise<{
  instruction: TransactionInstruction | null;
  address: PublicKey;
}> => {
  const associatedToken = await spl.getAssociatedTokenAddress(
    mint,
    owner,
    allowOwnerOffCurve,
    programId,
    associatedTokenProgramId
  );

  // This is the optimal logic, considering TX fee, client-side computation, RPC roundtrips and guaranteed idempotent.
  // Sadly we can't do this atomically.
  let account: spl.Account;
  try {
    account = await spl.getAccount(
      connection,
      associatedToken,
      commitment,
      programId
    );
    return {
      instruction: null,
      address: associatedToken,
    };
  } catch (error: unknown) {
    // TokenAccountNotFoundError can be possible if the associated address has already received some lamports,
    // becoming a system account. Assuming program derived addressing is safe, this is the only case for the
    // TokenInvalidAccountOwnerError in this code path.
    if (
      error instanceof spl.TokenAccountNotFoundError ||
      error instanceof spl.TokenInvalidOwnerError
    ) {
      // As this isn't atomic, it's possible others can create associated accounts meanwhile.
      try {
        const transaction = new TransactionInstruction(
          spl.createAssociatedTokenAccountInstruction(
            payer,
            associatedToken,
            owner,
            mint,
            programId,
            associatedTokenProgramId
          )
        );
        return {
          instruction: transaction,
          address: associatedToken,
        };
      } catch (error: unknown) {
        // Ignore all errors; for now there is no API-compatible way to selectively ignore the expected
        // instruction error if the associated account exists already.
      }

      // Now this should always succeed
      account = await spl.getAccount(
        connection,
        associatedToken,
        commitment,
        programId
      );
    } else {
      throw error;
    }
  }
  if (!account.mint.equals(mint)) throw new spl.TokenInvalidMintError();
  if (!account.owner.equals(owner)) throw new spl.TokenInvalidOwnerError();

  return {
    address: associatedToken,
    instruction: null,
  };
};
