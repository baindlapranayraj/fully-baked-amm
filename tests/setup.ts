import { Keypair, SystemProgram, Transaction } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { FullyBackedAmm } from "../target/types/fully_backed_amm";

import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { ProgramTestContext } from "solana-bankrun";
import {
  createInitializeMint2Instruction,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

const IDL = require("../target/idl/fully_backed_amm.json");

function getAccount(keypair: Keypair, context: ProgramTestContext) {
  const provider = new BankrunProvider(context);
  provider.wallet = new NodeWallet(keypair);
  const program = new Program<FullyBackedAmm>(IDL, provider);

  return {
    keypair,
    provider,
    program,
  };
}

export async function setupEnv() {
  const adminKeypair = Keypair.generate();
  const lpProviderKeypair = Keypair.generate();
  const mintAKeypair = Keypair.generate();
  const mintBKeypair = Keypair.generate();

  const context = await startAnchor(
    "",
    [],
    [
      {
        address: adminKeypair.publicKey,
        info: {
          lamports: 10_000_000,
          data: Buffer.alloc(0),
          owner: SYSTEM_PROGRAM_ID,
          executable: false,
        },
      },
      {
        address: lpProviderKeypair.publicKey,
        info: {
          lamports: 10_000_000,
          data: Buffer.alloc(0),
          owner: SYSTEM_PROGRAM_ID,
          executable: false,
        },
      },
    ]
  );

  let rent = await context.banksClient.getRent();

  const createAccountAIx = SystemProgram.createAccount({
    fromPubkey: adminKeypair.publicKey,
    newAccountPubkey: mintAKeypair.publicKey,
    space: MINT_SIZE,
    lamports: Number(await rent.minimumBalance(BigInt(MINT_SIZE))),
    programId: TOKEN_PROGRAM_ID,
  });

  const createMintAIx = createInitializeMint2Instruction(
    mintAKeypair.publicKey,
    6,
    adminKeypair.publicKey,
    null,
    TOKEN_PROGRAM_ID
  );

  const createAccountBIx = SystemProgram.createAccount({
    fromPubkey: adminKeypair.publicKey,
    newAccountPubkey: mintBKeypair.publicKey,
    space: MINT_SIZE,
    lamports: Number(await rent.minimumBalance(BigInt(MINT_SIZE))),
    programId: TOKEN_PROGRAM_ID,
  });

  const createMintBIx = createInitializeMint2Instruction(
    mintBKeypair.publicKey,
    6,
    adminKeypair.publicKey,
    null,
    TOKEN_PROGRAM_ID
  );

  const blockhash = context.lastBlockhash;
  const tx = new Transaction();
  tx.recentBlockhash = blockhash;

  tx.add(createAccountAIx, createMintAIx, createAccountBIx, createMintBIx);
  tx.sign(adminKeypair, mintAKeypair, mintBKeypair);

  await context.banksClient.processTransaction(tx);

  return {
    context,
    client: context.banksClient,
    admin: getAccount(adminKeypair, context),
    lpProvider: getAccount(lpProviderKeypair, context),
    mintA: getAccount(mintAKeypair, context),
    mintB: getAccount(mintBKeypair, context),
  };
}
