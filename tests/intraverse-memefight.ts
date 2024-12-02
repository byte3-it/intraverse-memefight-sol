import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IntraverseMemefight } from "../target/types/intraverse_memefight";
import { assert, expect } from "chai";
import { createMintAndVault, createTokenAccount, getTokenAccountAmount } from "./utils";
import { findPoolLpMint } from "./intraverse-utils";

describe("intraverse-memefight", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IntraverseMemefight as Program<IntraverseMemefight>;

  it("pool initialization", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const [poolMint] = await createMintAndVault(provider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
      })
      .signers([poolKp])
      .rpc();

    // Fetch the newly created account from the cluster.
    const account = await program.account.pool.fetch(poolKp.publicKey);

    // Check it's state was initialized.
    assert.ok(account.activationTh.eq(new anchor.BN(1234)));
    assert.ok(account.isOpen);
    assert.ok(account.mint.equals(poolMint));
    assert.ok(account.owner.equals(provider.wallet.publicKey));

    // TODO check authority of poolLpMint
  });

  it("authority can toggle isOpen", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const [poolMint] = await createMintAndVault(provider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
      })
      .signers([poolKp])
      .rpc();

    // Fetch the newly created account from the cluster.
    let account = await program.account.pool.fetch(poolKp.publicKey);
    assert.ok(account.isOpen == true);

    await program.methods
      .togglePool()
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    account = await program.account.pool.fetch(poolKp.publicKey);
    assert.ok(account.isOpen == false);

    await program.methods
      .togglePool()
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    account = await program.account.pool.fetch(poolKp.publicKey);
    assert.ok(account.isOpen == true);
  });

  it("cannot deposit on a pool if is closed", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const [poolMint, poolVault] = await createMintAndVault(provider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
      })
      .signers([poolKp])
      .rpc();

    await program.methods
      .togglePool()
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    const [poolLpMint] = findPoolLpMint(poolKp.publicKey, program.programId);

    const userLpTokenAccount = await createTokenAccount(provider, poolLpMint, provider.wallet.publicKey);

    try {
      await program.methods
        .deposit(new anchor.BN(100))
        .accounts({
          poolMint: poolMint,
          userTokenAccount: poolVault,
          userLpTokenAccount: userLpTokenAccount,
          pool: poolKp.publicKey,
        })
        .rpc();
      expect(true).to.be.false;
    } catch (err) {
      expect(err.message).to.contain("Error Code: PoolIsClosed.");
    }
  });

  it("cannot withdraw on a pool if is closed", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const [poolMint, poolVault] = await createMintAndVault(provider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
      })
      .signers([poolKp])
      .rpc();

    const [poolLpMint] = findPoolLpMint(poolKp.publicKey, program.programId);
    const userLpTokenAccount = await createTokenAccount(provider, poolLpMint, provider.wallet.publicKey);

    await program.methods
      .deposit(new anchor.BN(100))
      .accounts({
        poolMint: poolMint,
        userTokenAccount: poolVault,
        userLpTokenAccount: userLpTokenAccount,
        pool: poolKp.publicKey,
      })
      .rpc();

    await program.methods
      .togglePool()
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    try {
      await program.methods
        .withdraw(new anchor.BN(10))
        .accounts({
          poolMint: poolMint,
          userTokenAccount: poolVault,
          userLpTokenAccount: userLpTokenAccount,
          pool: poolKp.publicKey,
        })
        .rpc();
      expect(true).to.be.false;
    } catch (err) {
      expect(err.message).to.contain("Error Code: PoolIsClosed.");
    }
  });

  it("an account can deposit on the pool and then withdraw", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const initialMintAmount = 5000;
    const [poolMint, poolVault] = await createMintAndVault(provider, initialMintAmount);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
      })
      .signers([poolKp])
      .rpc();

    const [poolLpMint] = findPoolLpMint(poolKp.publicKey, program.programId);

    const userLpTokenAccount = await createTokenAccount(provider, poolLpMint, provider.wallet.publicKey);

    const depositedMint = 2000;
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        poolMint: poolMint,
        userTokenAccount: poolVault,
        userLpTokenAccount: userLpTokenAccount,
        pool: poolKp.publicKey,
      })
      .rpc();

    const [poolTreasury] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("treasury"), poolKp.publicKey.toBuffer(), poolMint.toBuffer()],
      program.programId
    );

    // check that the token accounts have the correct amount
    assert.equal(await getTokenAccountAmount(provider, poolVault), initialMintAmount - depositedMint);
    assert.equal(await getTokenAccountAmount(provider, poolTreasury), depositedMint);
    assert.equal(await getTokenAccountAmount(provider, userLpTokenAccount), depositedMint);

    const withdrawAmount = 1000;
    await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({
        poolMint: poolMint,
        userTokenAccount: poolVault,
        userLpTokenAccount: userLpTokenAccount,
        pool: poolKp.publicKey,
      })
      .rpc();

    // check that the token accounts have the correct amount
    assert.equal(await getTokenAccountAmount(provider, poolVault), initialMintAmount - depositedMint + withdrawAmount);
    assert.equal(await getTokenAccountAmount(provider, poolTreasury), depositedMint - withdrawAmount);
    assert.equal(await getTokenAccountAmount(provider, userLpTokenAccount), depositedMint - withdrawAmount);
  });

  it("create a competition", async () => {
    const poolKp_a = anchor.web3.Keypair.generate();
    const poolKp_b = anchor.web3.Keypair.generate();
    const initialMintAmount = 5000;
    const [poolMint_a, poolVault_a] = await createMintAndVault(provider, initialMintAmount);
    const [poolMint_b, poolVault_b] = await createMintAndVault(provider, initialMintAmount);

    // init pools
    await program.methods
      .initializePool(new anchor.BN(100))
      .accounts({
        pool: poolKp_a.publicKey,
        poolMint: poolMint_a,
      })
      .signers([poolKp_a])
      .rpc();
    await program.methods
      .initializePool(new anchor.BN(100))
      .accounts({
        pool: poolKp_b.publicKey,
        poolMint: poolMint_b,
      })
      .signers([poolKp_b])
      .rpc();

    // deposit enough to create a competition
    const [poolLpMint_a] = findPoolLpMint(poolKp_a.publicKey, program.programId);
    const [poolLpMint_b] = findPoolLpMint(poolKp_b.publicKey, program.programId);

    const userLpTokenAccount_a = await createTokenAccount(provider, poolLpMint_a, provider.wallet.publicKey);
    const userLpTokenAccount_b = await createTokenAccount(provider, poolLpMint_b, provider.wallet.publicKey);

    const depositedMint = 200;
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        poolMint: poolMint_a,
        userTokenAccount: poolVault_a,
        userLpTokenAccount: userLpTokenAccount_a,
        pool: poolKp_a.publicKey,
      })
      .rpc();
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        poolMint: poolMint_b,
        userTokenAccount: poolVault_b,
        userLpTokenAccount: userLpTokenAccount_b,
        pool: poolKp_b.publicKey,
      })
      .rpc();

    // create competition
    const competitionKp = anchor.web3.Keypair.generate();
    await program.methods
      .createCompetition()
      .accounts({
        poolA: poolKp_a.publicKey,
        poolB: poolKp_b.publicKey,
        competition: competitionKp.publicKey,
      })
      .signers([competitionKp])
      .rpc();

    // check that the pools are now closed
    let account_pool_a = await program.account.pool.fetch(poolKp_a.publicKey);
    let account_pool_b = await program.account.pool.fetch(poolKp_b.publicKey);
    expect(account_pool_a.isOpen).to.be.false;
    expect(account_pool_b.isOpen).to.be.false;

    // check that the competition is created
    let competition = await program.account.competition.fetch(competitionKp.publicKey);
    expect(competition.isAWinner).to.be.null;

    expect(competition.owner.toBase58()).to.be.eq(account_pool_a.owner.toBase58());
    expect(competition.owner.toBase58()).to.be.eq(account_pool_b.owner.toBase58());
    expect(competition.owner.toBase58()).to.be.eq(provider.wallet.publicKey.toBase58());

    expect(competition.poolA.toBase58()).to.be.eq(poolKp_a.publicKey.toBase58());
    expect(competition.poolB.toBase58()).to.be.eq(poolKp_b.publicKey.toBase58());
  });
});
