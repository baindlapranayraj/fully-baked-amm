import { beforeAll } from "@jest/globals";
import { setupEnv } from "./setup";
import { IContextAccounts } from "./types";
import { randomBytes } from "crypto";
import * as anchor from "@coral-xyz/anchor";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getMintLpPda, getPoolConfigPda } from "./helpers";

describe("Initialize Pool", () => {
  let adminAccount: IContextAccounts;
  let lpProviderAccount: IContextAccounts;
  let mintAAccount: IContextAccounts;
  let mintBAccount: IContextAccounts;
  let secretSeed = new anchor.BN(randomBytes(8));

  beforeAll(async () => {
    const { admin, lpProvider, mintA, mintB } = await setupEnv();

    adminAccount = admin;
    lpProviderAccount = lpProvider;
    mintAAccount = mintA;
    mintBAccount = mintB;
  });

  test("Initialize Pool", async () => {
    const poolConfigPda = getPoolConfigPda(
      adminAccount.program.programId,
      secretSeed
    );

    const mintLpPda = getMintLpPda(
      adminAccount.program.programId,
      poolConfigPda
    );

    const vaultA = getAssociatedTokenAddressSync(
      mintAAccount.keypair.publicKey,
      poolConfigPda,
      true
    );

    const vaultB = getAssociatedTokenAddressSync(
      mintBAccount.keypair.publicKey,
      poolConfigPda,
      true
    );
    let tx = await adminAccount.program.methods
      .initialize(secretSeed)
      .accountsStrict({
        admin: adminAccount.keypair.publicKey,
        mintA: mintAAccount.keypair.publicKey,
        mintB: mintBAccount.keypair.publicKey,
        poolConfigAccount: poolConfigPda,
        mintLp: mintLpPda,
        // vaultA: vaultA,
        // vaultB: vaultB,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      })
      .rpc({ skipPreflight: true });

    console.log(tx);
  });
});
