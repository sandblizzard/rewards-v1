import type { Connection, PublicKey, Commitment } from '@solana/web3.js';
import { TransactionInstruction } from '@solana/web3.js';
export declare const getOrCreateAssociatedTokenAccountIx: (connection: Connection, payer: PublicKey, mint: PublicKey, owner: PublicKey, allowOwnerOffCurve?: boolean, commitment?: Commitment, programId?: PublicKey, associatedTokenProgramId?: PublicKey) => Promise<{
    instruction: TransactionInstruction | null;
    address: PublicKey;
}>;
//# sourceMappingURL=utils.d.ts.map