import * as anchor from "@coral-xyz/anchor";

export function getPoolConfigPda(
  programId: anchor.web3.PublicKey,
  secretSeed: anchor.BN
) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("liquid_pool"), secretSeed.toArrayLike(Buffer, "le", 8)],
    programId
  )[0];
}

export function getMintLpPda(
  programId: anchor.web3.PublicKey,
  poolConfigPDA: anchor.web3.PublicKey
) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint_lp"), poolConfigPDA.toBuffer()],
    programId
  )[0];
}
