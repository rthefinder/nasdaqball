import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Nciball } from "../target/types/nciball";
import { expect } from "chai";

describe("nciball", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Nciball as Program<Nciball>;
  const authority = provider.wallet;

  let indexStatePda: anchor.web3.PublicKey;

  const TOTAL_SUPPLY = new anchor.BN(1_000_000_000);
  const TRANSACTION_FEE_BPS = 600; // 6%
  const REWARDS_ALLOCATION_BPS = 4000; // 40%
  const BUYBACK_ALLOCATION_BPS = 3000; // 30%
  const LIQUIDITY_ALLOCATION_BPS = 2000; // 20%
  const BURN_ALLOCATION_BPS = 1000; // 10%

  before(async () => {
    [indexStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("index_state")],
      program.programId
    );
  });

  it("Initializes the nasdaqball program", async () => {
    const tx = await program.methods
      .initialize(
        TOTAL_SUPPLY,
        TRANSACTION_FEE_BPS,
        REWARDS_ALLOCATION_BPS,
        BUYBACK_ALLOCATION_BPS,
        LIQUIDITY_ALLOCATION_BPS,
        BURN_ALLOCATION_BPS
      )
      .accounts({
        indexState: indexStatePda,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize transaction signature:", tx);

    const indexState = await program.account.indexState.fetch(indexStatePda);

    expect(indexState.authority.toString()).to.equal(authority.publicKey.toString());
    expect(indexState.totalSupply.toString()).to.equal(TOTAL_SUPPLY.toString());
    expect(indexState.circulatingSupply.toString()).to.equal(TOTAL_SUPPLY.toString());
    expect(indexState.transactionFeeBps).to.equal(TRANSACTION_FEE_BPS);
    expect(indexState.rewardsAllocationBps).to.equal(REWARDS_ALLOCATION_BPS);
    expect(indexState.buybackAllocationBps).to.equal(BUYBACK_ALLOCATION_BPS);
    expect(indexState.liquidityAllocationBps).to.equal(LIQUIDITY_ALLOCATION_BPS);
    expect(indexState.burnAllocationBps).to.equal(BURN_ALLOCATION_BPS);
    expect(indexState.rebalanceCycles.toString()).to.equal("0");
    expect(indexState.paused).to.equal(false);
  });

  it("Validates fee allocation must sum to 10000 bps", async () => {
    const [badIndexState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("index_state"), Buffer.from("bad")],
      program.programId
    );

    try {
      await program.methods
        .initialize(
          TOTAL_SUPPLY,
          TRANSACTION_FEE_BPS,
          5000, // Invalid: doesn't sum to 10000
          3000,
          2000,
          1000
        )
        .accounts({
          indexState: badIndexState,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.toString()).to.include("InvalidFeeAllocation");
    }
  });

  it("Validates transaction fee maximum (10%)", async () => {
    const [highFeeState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("index_state"), Buffer.from("highfee")],
      program.programId
    );

    try {
      await program.methods
        .initialize(
          TOTAL_SUPPLY,
          1500, // 15% - too high!
          REWARDS_ALLOCATION_BPS,
          BUYBACK_ALLOCATION_BPS,
          LIQUIDITY_ALLOCATION_BPS,
          BURN_ALLOCATION_BPS
        )
        .accounts({
          indexState: highFeeState,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.toString()).to.include("FeeTooHigh");
    }
  });

  it("Fetches index state correctly", async () => {
    const indexState = await program.account.indexState.fetch(indexStatePda);

    console.log("Index State:");
    console.log("  Total Supply:", indexState.totalSupply.toString());
    console.log("  Circulating Supply:", indexState.circulatingSupply.toString());
    console.log("  Rewards Pool:", indexState.rewardsPool.toString());
    console.log("  Buyback Pool:", indexState.buybackPool.toString());
    console.log("  Liquidity Pool:", indexState.liquidityPool.toString());
    console.log("  Rebalance Cycles:", indexState.rebalanceCycles.toString());

    expect(indexState).to.exist;
    expect(indexState.totalSupply.gt(new anchor.BN(0))).to.be.true;
  });
});
