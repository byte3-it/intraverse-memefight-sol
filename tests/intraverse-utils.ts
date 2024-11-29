import * as anchor from "@coral-xyz/anchor";

export function findPoolLpMint(poolPubkey: anchor.web3.PublicKey, programId: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("lp"), poolPubkey.toBuffer()],
    programId
  );
}
