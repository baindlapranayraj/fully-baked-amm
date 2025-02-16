import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FullyBackedAmm } from "../target/types/fully_backed_amm";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  getMint,
  mintToChecked,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { randomBytes } from "crypto";
import { BN } from "bn.js";

const airdropSOL = async (
  to: anchor.web3.PublicKey,
  provider: anchor.AnchorProvider,
  amount: number
) => {
  try {
    const tx = await provider.connection.requestAirdrop(
      to,
      anchor.web3.LAMPORTS_PER_SOL * amount
    );

    await provider.connection.confirmTransaction(tx, "confirmed");
  } catch (e) {
    console.log(`U got an error while trying to airdrop 'SOL: ${e}`);
  }
};

const createMintAccount = async (
  provider: anchor.AnchorProvider,
  payer: anchor.web3.Keypair
) => {
  try {
    const mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      6
    );

    return mint;
  } catch (e) {
    console.log(`Error while trying to create Mint Account ${e}`);
  }
};

const createATA = async (
  provider: anchor.AnchorProvider,
  payer: anchor.web3.Keypair,
  mint: anchor.web3.PublicKey,
  amount: number,
  mintAuth: anchor.web3.Keypair
) => {
  try {
    // create ATA
    const ata = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      payer.publicKey
    );

    // Mint tokens to ATA
    mintTokens(provider, payer, mint, ata.address, amount, 6, mintAuth);

    return ata.address;
  } catch (e) {
    console.log(`Error while trying to create ATA ${e}`);
  }
};

const mintTokens = async (
  provider: anchor.AnchorProvider,
  payer: anchor.web3.Keypair,
  mint: anchor.web3.PublicKey,
  destination: anchor.web3.PublicKey,
  amount: number,
  decimal: number,
  mintAuth: anchor.web3.Keypair
) => {
  try {
    await mintToChecked(
      provider.connection,
      mintAuth,
      mint,
      destination,
      mintAuth,
      amount * 1_000_000,
      decimal
    );
  } catch (e) {
    console.log(`Error while trying to mint tokens ${e}`);
  }
};

describe("fully-backed-amm", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FullyBackedAmm as Program<FullyBackedAmm>;

  let admin: anchor.web3.Keypair;
  let lqProvider: anchor.web3.Keypair;
  let secretSeed = new BN(randomBytes(8));

  let mintA: anchor.web3.PublicKey; // bonk
  let mintB: anchor.web3.PublicKey; // popcat
  let mintLP: anchor.web3.PublicKey;

  let lqProviderA: anchor.web3.PublicKey;
  let lqProviderB: anchor.web3.PublicKey;

  let vaultA: anchor.web3.PublicKey;
  let vaultB: anchor.web3.PublicKey;

  let poolConfigPDA: anchor.web3.PublicKey;

  before("Setup for testing", async () => {
    try {
      // Keypair generation
      admin = anchor.web3.Keypair.generate();
      lqProvider = anchor.web3.Keypair.generate();

      // airdrop sol for each Keyapir accounts
      await airdropSOL(admin.publicKey, provider, 10);
      await airdropSOL(lqProvider.publicKey, provider, 10);

      // Creating Mint Accounts
      mintA = await createMintAccount(provider, admin);
      mintB = await createMintAccount(provider, admin);

      // Create ATA and mint tokens to them
      lqProviderA = await createATA(provider, lqProvider, mintA, 10000, admin); // 10000 bonk coins
      lqProviderB = await createATA(provider, lqProvider, mintB, 10000, admin); // 10000 popcat coins

      // poolConfigPDA
      poolConfigPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("liquid_pool"), secretSeed.toArrayLike(Buffer, "le", 8)],
        program.programId
      )[0];

      // vault ATAs
      vaultA = getAssociatedTokenAddressSync(mintA, poolConfigPDA, true);
      vaultB = getAssociatedTokenAddressSync(mintB, poolConfigPDA, true);

      mintLP = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint_lp"), poolConfigPDA.toBuffer()],
        program.programId
      )[0];

      console.log(` 🦄🦄🦄 The mintLP in test-side is ${mintLP.toString()} 🦄🦄`)

    } catch (e) {
      console.log(`Error occured while setting up test-cases ${e}`);
    }
  });

  it("Is initialized!", async () => {
    try {
      console.log(`✅✅✅✅✅✅ Testing the 1st case ✅✅✅✅✅✅`);

      let trx = await program.methods
        .initialize(secretSeed)
        .accountsStrict({
          admin: admin.publicKey,

          mintA: mintA,
          mintB: mintB,
          mintLp: mintLP,

          vaultA: vaultA,
          vaultB: vaultB,
          poolConfigAccount: poolConfigPDA,

          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        })
        .signers([admin])
        .rpc();

      console.log(`✅✅✅ Yey setup has done ${trx.toString()} ✅✅✅`);
    } catch (e) {
      throw new Error(`Error Occured while testing initialize test-case ${e}`);
    }
  });
});


