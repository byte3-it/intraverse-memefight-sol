import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IntraverseMemefight } from "../target/types/intraverse_memefight";
import { assert, expect } from "chai";
import { createMintAndVault, createTokenAccount, getTokenAccountAmount, transferTokens } from "./utils";
import { findPoolAuthorityMint } from "./intraverse-utils";
import { getAccount, getMint, transfer } from "@solana/spl-token";

describe("intraverse-memefight", () => {
  // Configure the client to use the local cluster.
  const defaultProvider = anchor.AnchorProvider.env();
  anchor.setProvider(defaultProvider);

  const program = anchor.workspace.IntraverseMemefight as Program<IntraverseMemefight>;

  it("pool initialization", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const poolLpKp = anchor.web3.Keypair.generate();
    const [poolMint] = await createMintAndVault(defaultProvider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
      })
      .signers([poolKp, poolLpKp])
      .rpc();

    // Fetch the newly created account from the cluster.
    const account = await program.account.pool.fetch(poolKp.publicKey);

    // Check it's state was initialized.
    expect(account.activationTh.toNumber()).to.be.eq(new anchor.BN(1234).toNumber());
    expect(account.isOpen).to.be.true;
    expect(account.poolLpMint.toBase58()).to.be.eq(poolLpKp.publicKey.toBase58());
    expect(account.mint.toBase58()).to.be.eq(poolMint.toBase58());
    expect(account.owner.toBase58()).to.be.eq(defaultProvider.wallet.publicKey.toBase58());

    // check authority of poolLpMint
    const poolMint_info = await getMint(defaultProvider.connection, poolLpKp.publicKey);
    const poolLpMint_info = await getMint(defaultProvider.connection, poolLpKp.publicKey);
    expect(poolMint_info.decimals).to.be.eq(poolLpMint_info.decimals);

    const [poolAuthority] = findPoolAuthorityMint(poolKp.publicKey, program.programId);
    expect(poolLpMint_info.mintAuthority.toBase58()).to.be.eq(poolAuthority.toBase58());
  });

  it("authority can change a pool", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const poolLpKp = anchor.web3.Keypair.generate();
    const [poolMint] = await createMintAndVault(defaultProvider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
      })
      .signers([poolKp, poolLpKp])
      .rpc();

    // Fetch the newly created account from the cluster.
    let account = await program.account.pool.fetch(poolKp.publicKey);
    assert.ok(account.isOpen == true);

    await program.methods
      .updatePool({
        isOpen: false,
        activationTh: new anchor.BN(1000),
      })
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    account = await program.account.pool.fetch(poolKp.publicKey);
    assert.ok(account.isOpen == false);
    assert.ok(account.activationTh.toNumber() == 1000);

    await program.methods
      .updatePool({
        isOpen: true,
        activationTh: new anchor.BN(2000),
      })
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    account = await program.account.pool.fetch(poolKp.publicKey);
    assert.ok(account.isOpen == true);
    assert.ok(account.activationTh.toNumber() == 2000);
  });

  it("cannot deposit on a pool if is closed", async () => {
    const poolKp = anchor.web3.Keypair.generate();
    const poolLpKp = anchor.web3.Keypair.generate();
    const [poolMint, poolVault] = await createMintAndVault(defaultProvider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
      })
      .signers([poolKp, poolLpKp])
      .rpc();

    await program.methods
      .updatePool({
        isOpen: false,
        activationTh: new anchor.BN(1234),
      })
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    const userLpTokenAccount = await createTokenAccount(
      defaultProvider,
      poolLpKp.publicKey,
      defaultProvider.wallet.publicKey
    );

    try {
      await program.methods
        .deposit(new anchor.BN(100))
        .accounts({
          poolMint: poolMint,
          poolLpMint: poolLpKp.publicKey,
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
    const poolLpKp = anchor.web3.Keypair.generate();
    const [poolMint, poolVault] = await createMintAndVault(defaultProvider, 1000);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
      })
      .signers([poolKp, poolLpKp])
      .rpc();

    const userLpTokenAccount = await createTokenAccount(
      defaultProvider,
      poolLpKp.publicKey,
      defaultProvider.wallet.publicKey
    );

    await program.methods
      .deposit(new anchor.BN(100))
      .accounts({
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
        userTokenAccount: poolVault,
        userLpTokenAccount: userLpTokenAccount,
        pool: poolKp.publicKey,
      })
      .rpc();

    await program.methods
      .updatePool({
        isOpen: false,
        activationTh: new anchor.BN(1234),
      })
      .accounts({
        pool: poolKp.publicKey,
      })
      .rpc();

    try {
      await program.methods
        .withdraw(new anchor.BN(10))
        .accounts({
          poolMint: poolMint,
          poolLpMint: poolLpKp.publicKey,
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
    const poolLpKp = anchor.web3.Keypair.generate();
    const initialMintAmount = 5000;
    const [poolMint, poolVault] = await createMintAndVault(defaultProvider, initialMintAmount);

    await program.methods
      .initializePool(new anchor.BN(1234))
      .accounts({
        pool: poolKp.publicKey,
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
      })
      .signers([poolKp, poolLpKp])
      .rpc();

    const userLpTokenAccount = await createTokenAccount(
      defaultProvider,
      poolLpKp.publicKey,
      defaultProvider.wallet.publicKey
    );

    const depositedMint = 2000;
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
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
    assert.equal(await getTokenAccountAmount(defaultProvider, poolVault), initialMintAmount - depositedMint);
    assert.equal(await getTokenAccountAmount(defaultProvider, poolTreasury), depositedMint);
    assert.equal(await getTokenAccountAmount(defaultProvider, userLpTokenAccount), depositedMint);

    const withdrawAmount = 1000;
    await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({
        poolMint: poolMint,
        poolLpMint: poolLpKp.publicKey,
        userTokenAccount: poolVault,
        userLpTokenAccount: userLpTokenAccount,
        pool: poolKp.publicKey,
      })
      .rpc();

    // check that the token accounts have the correct amount
    assert.equal(
      await getTokenAccountAmount(defaultProvider, poolVault),
      initialMintAmount - depositedMint + withdrawAmount
    );
    assert.equal(await getTokenAccountAmount(defaultProvider, poolTreasury), depositedMint - withdrawAmount);
    assert.equal(await getTokenAccountAmount(defaultProvider, userLpTokenAccount), depositedMint - withdrawAmount);
  });

  it("create a competition", async () => {
    const poolKp_a = anchor.web3.Keypair.generate();
    const poolLpKp_a = anchor.web3.Keypair.generate();
    const poolKp_b = anchor.web3.Keypair.generate();
    const poolLpKp_b = anchor.web3.Keypair.generate();
    const initialMintAmount = 5000;
    const [poolMint_a, poolVault_a] = await createMintAndVault(defaultProvider, initialMintAmount);
    const [poolMint_b, poolVault_b] = await createMintAndVault(defaultProvider, initialMintAmount);

    // init pools
    await program.methods
      .initializePool(new anchor.BN(100))
      .accounts({
        pool: poolKp_a.publicKey,
        poolMint: poolMint_a,
        poolLpMint: poolLpKp_a.publicKey,
      })
      .signers([poolKp_a, poolLpKp_a])
      .rpc();
    await program.methods
      .initializePool(new anchor.BN(100))
      .accounts({
        pool: poolKp_b.publicKey,
        poolMint: poolMint_b,
        poolLpMint: poolLpKp_b.publicKey,
      })
      .signers([poolKp_b, poolLpKp_b])
      .rpc();

    // deposit enough to create a competition

    const userLpTokenAccount_a = await createTokenAccount(
      defaultProvider,
      poolLpKp_a.publicKey,
      defaultProvider.wallet.publicKey
    );
    const userLpTokenAccount_b = await createTokenAccount(
      defaultProvider,
      poolLpKp_b.publicKey,
      defaultProvider.wallet.publicKey
    );

    const depositedMint = 200;
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        poolMint: poolMint_a,
        poolLpMint: poolLpKp_a.publicKey,
        userTokenAccount: poolVault_a,
        userLpTokenAccount: userLpTokenAccount_a,
        pool: poolKp_a.publicKey,
      })
      .rpc();
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        poolMint: poolMint_b,
        poolLpMint: poolLpKp_b.publicKey,
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
        poolALpMint: poolLpKp_a.publicKey,
        poolB: poolKp_b.publicKey,
        poolBLpMint: poolLpKp_b.publicKey,
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
    expect(competition.owner.toBase58()).to.be.eq(defaultProvider.wallet.publicKey.toBase58());

    expect(competition.poolA.toBase58()).to.be.eq(poolKp_a.publicKey.toBase58());
    expect(competition.poolB.toBase58()).to.be.eq(poolKp_b.publicKey.toBase58());
  });

  it.only("pool reset after competition", async () => {
    const initialMintAmount = 5000;

    // init pool A
    const poolKp_a = anchor.web3.Keypair.generate();
    const poolLpKp_a = anchor.web3.Keypair.generate();
    const [poolMint_a, poolVault_a] = await createMintAndVault(defaultProvider, initialMintAmount);
    console.log("initialize pool a");
    await program.methods
      .initializePool(new anchor.BN(100))
      .accounts({
        pool: poolKp_a.publicKey,
        poolMint: poolMint_a,
        poolLpMint: poolLpKp_a.publicKey,
      })
      .signers([poolKp_a, poolLpKp_a])
      .rpc();

    // init pool B
    const poolKp_b = anchor.web3.Keypair.generate();
    const poolLpKp_b = anchor.web3.Keypair.generate();
    const [poolMint_b, poolVault_b] = await createMintAndVault(defaultProvider, initialMintAmount);
    console.log("initialize pool b");
    await program.methods
      .initializePool(new anchor.BN(100))
      .accounts({
        pool: poolKp_b.publicKey,
        poolMint: poolMint_b,
        poolLpMint: poolLpKp_b.publicKey,
      })
      .signers([poolKp_b, poolLpKp_b])
      .rpc();

    // * * * * * * * * * * * * * * * * * * * *
    // create user A and give them some tokens

    const userProvider_a = new anchor.AnchorProvider(
      defaultProvider.connection,
      new anchor.Wallet(anchor.web3.Keypair.generate())
    );
    console.log("user a: ", userProvider_a.publicKey.toBase58());

    const userA_airdropSig = await userProvider_a.connection.requestAirdrop(
      userProvider_a.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await userProvider_a.connection.confirmTransaction(userA_airdropSig);
    const userTokenAccount_a = await createTokenAccount(userProvider_a, poolMint_a, userProvider_a.publicKey);
    await transferTokens(defaultProvider, poolVault_a, userTokenAccount_a, defaultProvider.wallet.publicKey, 1000);
    const tempInfo = await getAccount(userProvider_a.connection, userTokenAccount_a);
    console.log("user a token account: owner", tempInfo.owner.toBase58());
    console.log("user a token account: amount", tempInfo.amount);
    const userLpTokenAccount_a = await createTokenAccount(
      userProvider_a,
      poolLpKp_a.publicKey,
      userProvider_a.publicKey
    );

    // create user B and give them some tokens

    const userProvider_b = new anchor.AnchorProvider(
      anchor.getProvider().connection,
      new anchor.Wallet(anchor.web3.Keypair.generate())
    );
    console.log("user b: ", userProvider_b.publicKey.toBase58());

    const userB_airdropSig = await userProvider_a.connection.requestAirdrop(
      userProvider_b.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await userProvider_b.connection.confirmTransaction(userB_airdropSig);
    const userTokenAccount_b = await createTokenAccount(userProvider_b, poolMint_b, userProvider_a.publicKey);
    await transferTokens(defaultProvider, poolVault_b, userTokenAccount_b, defaultProvider.wallet.publicKey, 1000);
    const userLpTokenAccount_b = await createTokenAccount(
      userProvider_b,
      poolLpKp_b.publicKey,
      userProvider_b.publicKey
    );

    // deposit enough to create a competition
    const depositedMint = 500;

    console.log("user a deposits on pool a");
    anchor.setProvider(userProvider_a);
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        pool: poolKp_a.publicKey,
        poolMint: poolMint_a,
        poolLpMint: poolLpKp_a.publicKey,
        userTokenAccount: userTokenAccount_a,
        userLpTokenAccount: userLpTokenAccount_a,
      })
      .rpc();

    console.log("user b deposits on pool b");
    anchor.setProvider(userProvider_b);
    await program.methods
      .deposit(new anchor.BN(depositedMint))
      .accounts({
        pool: poolKp_b.publicKey,
        poolMint: poolMint_b,
        poolLpMint: poolLpKp_b.publicKey,
        userTokenAccount: userTokenAccount_b,
        userLpTokenAccount: userLpTokenAccount_b,
      })
      .rpc();

    // TODO check token accounts

    return;

    // create competition
    const competitionKp = anchor.web3.Keypair.generate();
    await program.methods
      .createCompetition()
      .accounts({
        poolA: poolKp_a.publicKey,
        poolALpMint: poolLpKp_a.publicKey,
        poolB: poolKp_b.publicKey,
        poolBLpMint: poolLpKp_b.publicKey,
        competition: competitionKp.publicKey,
      })
      .signers([competitionKp])
      .rpc();
  });
});
