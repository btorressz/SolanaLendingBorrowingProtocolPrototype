//IN PROGRESS 
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from '@coral-xyz/anchor';
import { Program, AnchorProvider, web3, BN } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { SolanaLendingBorrowingProtocolPrototype } from '../target/types/SolanaLendingBorrowingProtocolPrototype';
import type { Errors } from "../target/types/errors";


describe('lending_borrowing', () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Errors as anchor.Program<Errors>;
  
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.LendingBorrowing as Program<SolanaLendingBorrowingProtocolPrototype>;


  let adminKeypair = web3.Keypair.generate();
  let lenderKeypair = web3.Keypair.generate();
  let borrowerKeypair = web3.Keypair.generate();
  let liquidatorKeypair = web3.Keypair.generate();

  let lenderAccount: PublicKey;
  let borrowerAccount: PublicKey;
  let collateralAccount: PublicKey;
  let tokenAccount: PublicKey;
  let interestRateAccount: PublicKey;
  let proposalAccount: PublicKey;
  let insuranceFundAccount: PublicKey;

  const amount = new BN(1000);
  const collateral = new BN(2000);
  const interestRate = 5;
  const minCollateralRatio = new BN(150);
  const penalty = new BN(10);

  before(async () => {
    // Airdrop SOL to the admin, lender, borrower, and liquidator
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(adminKeypair.publicKey, 1e9)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(lenderKeypair.publicKey, 1e9)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(borrowerKeypair.publicKey, 1e9)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(liquidatorKeypair.publicKey, 1e9)
    );

    // Initialize accounts
    lenderAccount = await PublicKey.createWithSeed(
      lenderKeypair.publicKey,
      "lender",
      program.programId
    );
    borrowerAccount = await PublicKey.createWithSeed(
      borrowerKeypair.publicKey,
      "borrower",
      program.programId
    );
    collateralAccount = await PublicKey.createWithSeed(
      borrowerKeypair.publicKey,
      "collateral",
      program.programId
    );
    tokenAccount = await PublicKey.createWithSeed(
      adminKeypair.publicKey,
      "token",
      program.programId
    );
    interestRateAccount = await PublicKey.createWithSeed(
      adminKeypair.publicKey,
      "interest_rate",
      program.programId
    );
    proposalAccount = await PublicKey.createWithSeed(
      adminKeypair.publicKey,
      "proposal",
      program.programId
    );
    insuranceFundAccount = await PublicKey.createWithSeed(
      adminKeypair.publicKey,
      "insurance_fund",
      program.programId
    );
  });

  it('Initialize the protocol', async () => {
    const tx = await program.methods.initialize()
      .accounts({
        payer: adminKeypair.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([adminKeypair])
      .rpc();

    console.log("Initialize transaction signature", tx);
  });

  it('Lend tokens', async () => {
    const tx = await program.methods.lend(amount)
      .accounts({
        lender: lenderKeypair.publicKey,
        lenderAccount,
        tokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([lenderKeypair])
      .rpc();

    console.log("Lend transaction signature", tx);
  });

  it('Borrow tokens with collateral', async () => {
    const tx = await program.methods.borrow(amount, collateral, interestRate)
      .accounts({
        borrower: borrowerKeypair.publicKey,
        borrowerAccount,
        collateralAccount,
        tokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([borrowerKeypair])
      .rpc();

    console.log("Borrow transaction signature", tx);
  });

  it('Repay borrowed tokens', async () => {
    const tx = await program.methods.repay(amount)
      .accounts({
        borrower: borrowerKeypair.publicKey,
        borrowerAccount,
        tokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([borrowerKeypair])
      .rpc();

    console.log("Repay transaction signature", tx);
  });

  it('Liquidate under-collateralized loan', async () => {
    const tx = await program.methods.liquidate(borrowerKeypair.publicKey, minCollateralRatio, penalty)
      .accounts({
        liquidator: liquidatorKeypair.publicKey,
        borrowerAccount,
        collateralAccount,
        tokenAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([liquidatorKeypair])
      .rpc();

    console.log("Liquidate transaction signature", tx);
  });

  it('Update interest rates', async () => {
    const tx = await program.methods.updateInterestRate(3, 2)
      .accounts({
        admin: adminKeypair.publicKey,
        interestRate: interestRateAccount,
      })
      .signers([adminKeypair])
      .rpc();

    console.log("Update interest rate transaction signature", tx);
  });

  it('Create a governance proposal', async () => {
    const tx = await program.methods.propose()
      .accounts({
        proposer: adminKeypair.publicKey,
        proposal: proposalAccount,
        systemProgram: SystemProgram.programId,
      })
      .signers([adminKeypair])
      .rpc();

    console.log("Create proposal transaction signature", tx);
  });

  it('Vote on a proposal', async () => {
    const tx = await program.methods.vote(true)
      .accounts({
        voter: adminKeypair.publicKey,
        proposal: proposalAccount,
      })
      .signers([adminKeypair])
      .rpc();

    console.log("Vote transaction signature", tx);
  });

  it('Deposit to insurance fund', async () => {
    const tx = await program.methods.depositToInsuranceFund(amount)
      .accounts({
        admin: adminKeypair.publicKey,
        insuranceFund: insuranceFundAccount,
      })
      .signers([adminKeypair])
      .rpc();

    console.log("Deposit to insurance fund transaction signature", tx);
  });
});
