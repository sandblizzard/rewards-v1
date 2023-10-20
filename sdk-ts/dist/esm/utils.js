import * as spl from '@solana/spl-token';
import { TransactionInstruction } from '@solana/web3.js';
export const getOrCreateAssociatedTokenAccountIx = async (connection, payer, mint, owner, allowOwnerOffCurve = false, commitment = 'finalized', programId = spl.TOKEN_PROGRAM_ID, associatedTokenProgramId = spl.ASSOCIATED_TOKEN_PROGRAM_ID) => {
    const associatedToken = await spl.getAssociatedTokenAddress(mint, owner, allowOwnerOffCurve, programId, associatedTokenProgramId);
    // This is the optimal logic, considering TX fee, client-side computation, RPC roundtrips and guaranteed idempotent.
    // Sadly we can't do this atomically.
    let account;
    try {
        account = await spl.getAccount(connection, associatedToken, commitment, programId);
        return {
            instruction: null,
            address: associatedToken,
        };
    }
    catch (error) {
        // TokenAccountNotFoundError can be possible if the associated address has already received some lamports,
        // becoming a system account. Assuming program derived addressing is safe, this is the only case for the
        // TokenInvalidAccountOwnerError in this code path.
        if (error instanceof spl.TokenAccountNotFoundError ||
            error instanceof spl.TokenInvalidOwnerError) {
            // As this isn't atomic, it's possible others can create associated accounts meanwhile.
            try {
                const transaction = new TransactionInstruction(spl.createAssociatedTokenAccountInstruction(payer, associatedToken, owner, mint, programId, associatedTokenProgramId));
                return {
                    instruction: transaction,
                    address: associatedToken,
                };
            }
            catch (error) {
                // Ignore all errors; for now there is no API-compatible way to selectively ignore the expected
                // instruction error if the associated account exists already.
            }
            // Now this should always succeed
            account = await spl.getAccount(connection, associatedToken, commitment, programId);
        }
        else {
            throw error;
        }
    }
    if (!account.mint.equals(mint))
        throw new spl.TokenInvalidMintError();
    if (!account.owner.equals(owner))
        throw new spl.TokenInvalidOwnerError();
    return {
        address: associatedToken,
        instruction: null,
    };
};
//# sourceMappingURL=utils.js.map