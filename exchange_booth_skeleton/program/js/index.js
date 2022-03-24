const {
  Connection,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
} = require("@solana/web3.js");

const BN = require("bn.js");

const main = async () => {
  const connection = new Connection("https://api.devnet.solana.com/");

  
  var args = process.argv.slice(2);
  const programId = new PublicKey(args[0]);
  
  const feePayer = new Keypair();
  
  console.log("Requesting Airdrop of 1 SOL...");
  await connection.requestAirdrop(feePayer.publicKey, 2e9);
  console.log("Airdrop received");

  const exchangeBoothKey = (await PublicKey.findProgramAddress(
    [Buffer.from("exchange_booth"), feePayer.publicKey.toBuffer()], programId
  ))[0];

  const mint1 = new Keypair();
  const mint2 = new Keypair();

  let mint1AccountCreateIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: mint1.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(100),
    space: 100,
    programId: programId
  })

  let mint2AccountCreateIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: mint2.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(100),
    space: 100,
    programId: programId
  })

  const user_ta_mint1 = new Keypair();
  const user_ta_mint2 = new Keypair();

  let userMint1AccountCreateIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: user_ta_mint1.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(200),
    space: 73,
    programId: programId
  });

  let userMint2AccountCreateIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: user_ta_mint2.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(200),
    space: 73,
    programId: programId
  });

  let accountCreationTransactions = new Transaction()
  accountCreationTransactions.add(mint1AccountCreateIx).add(mint2AccountCreateIx).add(userMint1AccountCreateIx).add(userMint2AccountCreateIx);

  let createTx = await sendAndConfirmTransaction(
    connection,
    accountCreationTransactions,
    [feePayer, mint1, mint2, user_ta_mint1, user_ta_mint2],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );
  
  const oracleAddressKey = (await PublicKey.findProgramAddress(
    [Buffer.from("oracle"), mint1.publicKey.toBuffer(), mint2.publicKey.toBuffer(), feePayer.publicKey.toBuffer()], programId
  ))[0];

  const vault1AddressKey = (await PublicKey.findProgramAddress(
    [Buffer.from("vault_from_mint"), feePayer.publicKey.toBuffer(), exchangeBoothKey.toBuffer(), mint1.publicKey.toBuffer()], programId
  ))[0];

  const vault2AddressKey = (await PublicKey.findProgramAddress(
    [Buffer.from("vault_from_mint"), feePayer.publicKey.toBuffer(), exchangeBoothKey.toBuffer(), mint2.publicKey.toBuffer()], programId
  ))[0];


  const instruction_idx = Buffer.from(new Uint8Array([0]));

  let buf = new ArrayBuffer(64);
  let flt = new Float64Array(buf);
  flt[0] = 1.2221;
  
  
  let bytes = new Uint8Array(new Float64Array([1.254]).buffer)
  console.log(bytes);

  let bytes2 = new Uint8Array(new Float64Array([1.000]).buffer)
  console.log(bytes2);

  let initializeExchangeBoothIx = new TransactionInstruction({
    keys: [
      {
        pubkey: exchangeBoothKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: mint1.publicKey,
        isSigner: false,
        isWritable: false
      },
      {
        pubkey: mint2.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: oracleAddressKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vault1AddressKey,
        isSigner: false,
        isWritable: true
      },
      {
        pubkey: vault2AddressKey,
        isSigner: false, 
        isWritable: true,
      },
      {
        pubkey: user_ta_mint1.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user_ta_mint2.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.concat([instruction_idx, Buffer.from(bytes2), Buffer.from(bytes)]),
    programId: programId,
  });

  let initExchangeBoothTransaction = new Transaction();
  initExchangeBoothTransaction.add(initializeExchangeBoothIx);

  let txid = await sendAndConfirmTransaction(
    connection,
    initExchangeBoothTransaction,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );
  
  console.log("\n\n-----------Transaction 1-------------");

  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);
  
  
  // const data = (await connection.getAccountInfo(vault2AddressKey, "confirmed")).data;
  
  //END OF INSTRUCTION 1, START OF INSTRUCTION 2
  //
  //

  console.log("\n\n-----------Transaction 2-------------");
  
  const instruction_idx_2 = Buffer.from(new Uint8Array([1]));
  
  let depositAmount = new Uint8Array(new Float64Array([1.43]).buffer)

  let deposit_ix = new TransactionInstruction({
    keys: [
      {
        pubkey: exchangeBoothKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: user_ta_mint1.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint1.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: vault1AddressKey,
        isSigner: false,
        isWritable: true,
      },
    ],
    
    data: Buffer.concat([instruction_idx_2,  Buffer.from(depositAmount)]),
    programId: programId,
  })

  let transaction2 = new Transaction();
  transaction2.add(deposit_ix);

  let txid2 = await sendAndConfirmTransaction(
    connection,
    transaction2,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );

  console.log(`https://explorer.solana.com/tx/${txid2}?cluster=devnet`);
  
  // END OF TRANSACTION 2, STARTING TRANSACTION 3
  //
  //
  //
  // 
  // 
  
  console.log("\n\n-----------Transaction 3-------------");
  
  const instruction_idx_3 = Buffer.from(new Uint8Array([2]));
  let withdrawalAmount = new Uint8Array(new Float64Array([1.43]).buffer)

  let withdraw_ix = new TransactionInstruction({
    keys: [
      {
        pubkey: exchangeBoothKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: user_ta_mint1.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint1.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: vault1AddressKey,
        isSigner: false,
        isWritable: true,
      },
    ],
    
    data: Buffer.concat([instruction_idx_3,  Buffer.from(withdrawalAmount)]),
    programId: programId,
  })

  let transaction3 = new Transaction();
  transaction3.add(withdraw_ix);

  let txid3 = await sendAndConfirmTransaction(
    connection,
    transaction3,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );

  console.log(`https://explorer.solana.com/tx/${txid3}?cluster=devnet`);

  // END OF TRANSACTION 3, START OF TRANSACTION 4
  //
  //
  //
  //

  console.log("\n\n-----------Transaction 4-------------");
  
  const instruction_idx_4 = Buffer.from(new Uint8Array([3]));

  let exchange_ix = new TransactionInstruction({
    keys: [
      {
        pubkey: exchangeBoothKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: user_ta_mint1.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user_ta_mint2.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vault1AddressKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: vault2AddressKey,
        isSigner: false,
        isWritable: true
      },
      {
        pubkey: oracleAddressKey,
        isSigner: false,
        isWritable: false,
      }
    ],
    
    data: Buffer.concat([instruction_idx_4,  Buffer.from(new Uint8Array(new BN(1).toArray("le", 8)))]),
    programId: programId,
  })

  // let transaction3 = new Transaction();
  // transaction3.add(withdraw_ix);

  // let txid3 = await sendAndConfirmTransaction(
  //   connection,
  //   transaction3,
  //   [feePayer],
  //   {
  //     skipPreflight: true,
  //     preflightCommitment: "confirmed",
  //     commitment: "confirmed",
  //   }
  // );

  // console.log(`https://explorer.solana.com/tx/${txid3}?cluster=devnet`);
  // console.log("\n\n");


};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
