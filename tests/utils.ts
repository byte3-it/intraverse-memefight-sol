import * as anchor from "@coral-xyz/anchor";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  createMintToInstruction,
  createTransferInstruction,
  getAccount,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export async function createTokenAccount(
  provider: anchor.AnchorProvider,
  mint: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey
) {
  const tx = new anchor.web3.Transaction();

  const aToken = await getAssociatedTokenAddress(mint, owner);
  tx.add(createAssociatedTokenAccountInstruction(provider.wallet.publicKey, aToken, owner, mint));
  const signature = await provider.sendAndConfirm(tx);
  // console.log("createTokenAccount signature: ", signature);

  return aToken;
}

export async function createMintAndVault(provider: anchor.AnchorProvider, amount: number, decimals: number = 6) {
  const mint = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();

  const createMintIx = await createMintInstructions(provider, mint.publicKey, decimals, authority.publicKey);
  const aToken = await getAssociatedTokenAddress(mint.publicKey, provider.wallet.publicKey);

  const aTokenCreationIx = createAssociatedTokenAccountInstruction(
    provider.wallet.publicKey,
    aToken,
    provider.wallet.publicKey,
    mint.publicKey
  );
  const mintToIx = createMintToInstruction(mint.publicKey, aToken, authority.publicKey, amount);

  const tx = new anchor.web3.Transaction();
  tx.add(...createMintIx);
  tx.add(aTokenCreationIx);
  tx.add(mintToIx);

  const signature = await provider.sendAndConfirm(tx, [mint, authority]);

  return [mint.publicKey, aToken];
}

export async function transferTokens(
  provider: anchor.AnchorProvider,
  source: anchor.web3.PublicKey,
  destination: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
  amount: number | bigint
) {
  const tx = new anchor.web3.Transaction();
  tx.add(createTransferInstruction(source, destination, owner, amount));
  return await provider.sendAndConfirm(tx);
}

export async function createMintInstructions(
  provider: anchor.AnchorProvider,
  mint: anchor.web3.PublicKey,
  decimals: number,
  authority: anchor.web3.PublicKey
) {
  return [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(mint, decimals, authority, null),
  ];
}

export async function getTokenAccountAmount(
  provider: anchor.AnchorProvider,
  tokenAccount: anchor.web3.PublicKey
): Promise<number> {
  return Number((await getAccount(provider.connection, tokenAccount, undefined, TOKEN_PROGRAM_ID)).amount);
}
