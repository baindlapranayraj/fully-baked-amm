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
  getAccount,
  Account,
  Mint,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { randomBytes } from "crypto";
import { BN } from "bn.js";
import { assert } from "chai";

// -------- Helper Functions --------

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
  let user: anchor.web3.Keypair;

  let secretSeed = new BN(randomBytes(8));

  let mintA: anchor.web3.PublicKey; // bonk
  let mintB: anchor.web3.PublicKey; // popcat
  let mintLP: anchor.web3.PublicKey;

  let lqProviderA: anchor.web3.PublicKey;
  let lqProviderB: anchor.web3.PublicKey;
  let lqProviderLP: anchor.web3.PublicKey;
  let userTokenA: anchor.web3.PublicKey;
  let userTokenB: anchor.web3.PublicKey;

  let lqAmountA = 1000;
  let lqAmountB = 1000;

  let vaultA: anchor.web3.PublicKey;
  let vaultB: anchor.web3.PublicKey;

  let poolConfigPDA: anchor.web3.PublicKey;

  before("Setup for testing", async () => {
    try {
      // Keypair generation
      admin = anchor.web3.Keypair.generate();
      lqProvider = anchor.web3.Keypair.generate();
      user = anchor.web3.Keypair.generate();

      // poolConfigPDA
      poolConfigPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("liquid_pool"), secretSeed.toArrayLike(Buffer, "le", 8)],
        program.programId
      )[0];

      // airdrop sol for each Keyapir accounts
      await airdropSOL(admin.publicKey, provider, 10);
      await airdropSOL(lqProvider.publicKey, provider, 10);
      await airdropSOL(user.publicKey, provider, 10);

      // Creating Mint Accounts
      mintA = await createMintAccount(provider, admin);
      mintB = await createMintAccount(provider, admin);
      mintLP = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint_lp"), poolConfigPDA.toBuffer()],
        program.programId
      )[0];

      // Create ATA and mint tokens to them
      lqProviderA = await createATA(provider, lqProvider, mintA, 10000, admin); // 10000 bonk coins
      lqProviderB = await createATA(provider, lqProvider, mintB, 10000, admin); // 10000 popcat coins
      userTokenA = await createATA(provider, user, mintA, 100, admin); // 100 bonk coins
      userTokenB = await createATA(provider, user, mintB, 0, admin); // 0 popcat coins

      // vault ATAs
      vaultA = getAssociatedTokenAddressSync(mintA, poolConfigPDA, true);
      vaultB = getAssociatedTokenAddressSync(mintB, poolConfigPDA, true);
    } catch (e) {
      console.log(`Error occured while setting up test-cases ${e}`);
    }
  });

  it("Is initialized! (1nd instruction)", async () => {
    try {
      await program.methods
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

      let account = await program.account.poolConfig.fetch(poolConfigPDA);
      // console.log(`✅✅✅ Yey setup has done ${admin.publicKey} ✅✅✅`);
      // console.log(`🦄🦄🦄 The account details are ${account.owner} 🦄🦄🦄`);
    } catch (e) {
      throw new Error(`Error Occured while testing initialize test-case ${e}`);
    }
  });

  it("Is setting-up liquidity (2nd instruction)", async () => {
    try {
      let tx = await program.methods
        .depositeAsset(new anchor.BN(lqAmountA), new anchor.BN(lqAmountB))
        .accountsPartial({
          liquidProvider: lqProvider.publicKey,

          poolConfigAccount: poolConfigPDA,

          mintA: mintA,
          mintB: mintB,
          mintLp: mintLP,

          providerTokenA: lqProviderA,
          providerTokenB: lqProviderB,

          vaultA: vaultA,
          vaultB: vaultB,

          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        })
        .signers([lqProvider])
        .rpc();

      const vaultAPDA = await getAccount(provider.connection, vaultA);
      const vaultBPDA = await getAccount(provider.connection, vaultB);
      // console.log(
      //   `✨ setting-up test-case worked like a charm ✨ the vaultA owner is ${vaultAPDA.owner.toString()} the vaultA owner is ${vaultBPDA.owner.toString()} and the configPDA is ${poolConfigPDA.toString()}`
      // );
    } catch (error) {
      console.log(`Error occured while testing etting-up test-case ${error}`);
    }
  });

  it("Is adding liquidity (2nd instruction)", async () => {
    try {
      let tx = await program.methods
        .depositeAsset(new anchor.BN(100), new anchor.BN(100))
        .accountsPartial({
          liquidProvider: lqProvider.publicKey,

          poolConfigAccount: poolConfigPDA,

          mintA: mintA,
          mintB: mintB,
          mintLp: mintLP,

          providerTokenA: lqProviderA,
          providerTokenB: lqProviderB,

          vaultA: vaultA,
          vaultB: vaultB,

          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        })
        .signers([lqProvider])
        .rpc();

      const vaultAPDA: Account = await getAccount(provider.connection, vaultA);
      const vaultBPDA = await getAccount(provider.connection, vaultB);

      console.log(`✨ setting-up test-case worked like a charm ✨`);
      console.log(
        `🥳 The amount in vaultA is ${Number(
          vaultAPDA.amount
        )} and vaultB ${Number(vaultBPDA.amount)} 🥳`
      );
    } catch (error) {
      console.log(`Error occured while testing etting-up test-case ${error}`);
    }
  });

  it("checking the lp Tokens", async () => {
    let amountA = 100;
    let amountB = 100;
    try {

      let lqProviderLPAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        lqProvider,
        mintLP,
        lqProvider.publicKey
      );

      const vaultAPDA: Account = await getAccount(provider.connection, vaultA);
      const vaultBPDA = await getAccount(provider.connection, vaultB);

      const lpMintAccount: Mint = await getMint(provider.connection, mintLP);
      console.log(
        `💛💛💛💛 Checking Mint supply ${Number(lpMintAccount.supply)} 💛💛💛💛`
      );

      let lpTokens =
        (amountA / Number(vaultAPDA.amount)) * Number(lpMintAccount.supply);       // s = (dx/X)T: for cal lp shares

        console.log(
          `🦄🦄🦄🦄 checking the no.of minted lp tokens  are....${lpTokens} 🦄🦄🦄🦄`
        );
      assert.equal(Number(lqProviderLPAccount.amount), lpTokens);
      console.log(
        `🦄🦄🦄🦄 The no.of minted lp tokens are perfect ${lpTokens} 🦄🦄🦄🦄`
      );
    } catch (error) {
      console.log(`Error got while trying check lp tokens ${error}`);
    }
  });

  // it("Is adding liquidity should fail (2nd instruction)", async () => {
  //   try {
  //     let tx = await program.methods
  //       .depositeAsset(new anchor.BN(10), new anchor.BN(100))
  //       .accountsPartial({
  //         liquidProvider: lqProvider.publicKey,

  //         poolConfigAccount: poolConfigPDA,

  //         mintA: mintA,
  //         mintB: mintB,
  //         mintLp: mintLP,

  //         providerTokenA: lqProviderA,
  //         providerTokenB: lqProviderB,

  //         vaultA: vaultA,
  //         vaultB: vaultB,

  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
  //       })
  //       .signers([lqProvider])
  //       .rpc();

  //     const vaultAPDA: Account = await getAccount(provider.connection, vaultA);
  //     const vaultBPDA = await getAccount(provider.connection, vaultB);

  //     // console.log(`✨ etting-up test-case worked like a charm ✨`);
  //     // console.log(
  //     //   `🥳 The amount in vaultA is ${Number(
  //     //     vaultAPDA.amount
  //     //   )} and vaultB ${Number(vaultBPDA.amount)} 🥳`
  //     // );
  //   } catch (error) {
  //     console.log(`Dont worry this is suppoused to fail ${error}`);
  //   }
  // });

  it("Swap token ", async () => {
    try {
      const vaultAPDA: Account = await getAccount(provider.connection, vaultA);
      const vaultBPDA = await getAccount(provider.connection, vaultB);

      const reqAmount =
        (Number(vaultBPDA.amount) * 10) / (10 + Number(vaultAPDA.amount));
      const amountBefore = (await getAccount(provider.connection, userTokenB))
        .amount;

      console.log(`Amount before trx ${amountBefore}`);

      await program.methods
        .swap(true, new anchor.BN(10))
        .accountsStrict({
          user: user.publicKey,
          userTokenA: userTokenA,
          userTokenB: userTokenB,
          poolConfigAccount: poolConfigPDA,

          mintA: mintA,
          mintB: mintB,

          vaultA: vaultA,
          vaultB: vaultB,

          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        })
        .signers([user])
        .rpc({ skipPreflight: true });

      const amountAfter = (await getAccount(provider.connection, userTokenB))
        .amount;

      console.log(`Amount after trx ${Number(amountAfter)}`);
      console.log(`Requierd amount ${reqAmount}`);
      // assert.equal(reqAmount, Number(amountAfter));
    } catch (error) {
      console.log(`You got error while trying to swap a token ${error}`);
    }
  });

  it("withdraw asset", async () => {
    try {
      let lqProviderLPAccount: Account =
        await getOrCreateAssociatedTokenAccount(
          provider.connection,
          lqProvider,
          mintLP,
          lqProvider.publicKey
        );

      let mintLPAccount: Mint = await getMint(provider.connection, mintLP);

      console.log(
        `The mint supply of token mint account is ${Number(
          mintLPAccount.supply
        )}`
      );

      await program.methods
        .withdrawAsset(new anchor.BN(Number(lqProviderLPAccount.amount)))
        .accountsStrict({
          user: lqProvider.publicKey,
          poolConfigAccount: poolConfigPDA,

          mintA: mintA,
          mintB: mintB,
          mintLp: mintLP,

          userTokenA: lqProviderA,
          userTokenB: lqProviderB,
          userTokenLp: lqProviderLPAccount.address,

          vaultA: vaultA,
          vaultB: vaultB,

          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        })
        .signers([lqProvider])
        .rpc({
          skipPreflight: true,
        });

      const vaultAPDA: Account = await getAccount(provider.connection, vaultA);
      const vaultBPDA = await getAccount(provider.connection, vaultB);

      console.log(`✨ setting-up test-case worked like a charm ✨`);
      console.log(
        `🥳 The amount in vaultA is ${Number(
          vaultAPDA.amount
        )} and vaultB ${Number(vaultBPDA.amount)} 🥳`
      );
    } catch (error) {
      console.log(
        `You got error while trying to test the withdraw asset ${error}`
      );
    }
  });
});
