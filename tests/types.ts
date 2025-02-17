import { Keypair } from "@solana/web3.js";

import { Program } from "@coral-xyz/anchor";
import { BankrunProvider } from "anchor-bankrun";
import { FullyBackedAmm } from "../target/types/fully_backed_amm";

export interface IContextAccounts {
  keypair: Keypair;
  provider: BankrunProvider;
  program: Program<FullyBackedAmm>;
}
