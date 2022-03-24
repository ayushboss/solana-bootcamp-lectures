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
  var args = process.argv.slice(2);
  const programId = new PublicKey(args[0]);
  const echo = args[1];

  const connection = new Connection("https://api.devnet.solana.com/");

  const feePayer = new Keypair();
  const echoBuffer = new Keypair();

  console.log("Requesting Airdrop of 1 SOL...");
  await connection.requestAirdrop(feePayer.publicKey, 2e9);
  console.log("Airdrop received");

  let createIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: echoBuffer.publicKey,
    /** Amount of lamports to transfer to the created account */
    lamports: await connection.getMinimumBalanceForRentExemption(echo.length),
    /** Amount of space in bytes to allocate to the created account */
    space: echo.length,
    /** Public key of the program to assign as the owner of the created account */
    programId: programId,
  });

  const idx = Buffer.from(new Uint8Array([0]));

  const messageLen = Buffer.from(new Uint8Array((new BN(echo.length)).toArray("le", 4)));
  const message = Buffer.from(echo, "ascii");

  console.log(new Uint8Array((new BN(echo.length)).toArray("le", 4)));
  console.log(Buffer.byteLength(messageLen));
  console.log(Buffer.concat([idx, messageLen, message]));

  let echoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: echoBuffer.publicKey,
        isSigner: false,
        isWritable: true,
      },
    ],
    programId: programId,
    data: Buffer.concat([idx, messageLen, message]),
  });


  let tx = new Transaction();
  tx.add(createIx).add(echoIx);

  let txid = await sendAndConfirmTransaction(
    connection,
    tx,
    [feePayer, echoBuffer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);


  data = (await connection.getAccountInfo(echoBuffer.publicKey, "confirmed")).data;

  const bufferSeed = new BN(40);
  const bufferSeedByteArray = new Uint8Array(bufferSeed.toArray("le", 8));

  const authorizedBufferKey = (await PublicKey.findProgramAddress(
    [Buffer.from("authority"), feePayer.publicKey.toBuffer(), Buffer.from(bufferSeedByteArray)],
    programId
  ))[0];


  console.log("data: " + Buffer.concat([idx, messageLen, message]).toString());
  console.log("data2: " + Buffer.concat([Buffer.from(new Uint8Array([1])), Buffer.from(new Uint8Array(bufferSeed.toArray("le", 8))), Buffer.from(new Uint8Array((new BN(100)).toArray("le", 8)))]).toString());

  const idx2 = Buffer.from(new Uint8Array([1]));

  let authorizedEchoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: authorizedBufferKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.concat([Buffer.from(new Uint8Array([1])), Buffer.from(new Uint8Array(bufferSeed.toArray("le", 8))), Buffer.from(new Uint8Array((new BN(100)).toArray("le", 8)))]),
    programId: programId,
  });;

  let tx2 = new Transaction();
  tx2.add(authorizedEchoIx);

  let txid2 = await sendAndConfirmTransaction(
    connection,
    tx2,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  );

  console.log(`https://explorer.solana.com/tx/${txid2}?cluster=devnet`);

  data2 = (await connection.getAccountInfo(authorizedBufferKey, "confirmed")).data;

  console.log("Echo Buffer Text:", data.toString());
  console.log("testing part 3");
  
  const idx3 = Buffer.from(new Uint8Array([2]));
  const authEchoInput = args[2];

  console.log(Buffer.concat([idx3, Buffer.from(new Uint8Array(new BN(authEchoInput.length).toArray("le", 8))), Buffer.from(authEchoInput, "ascii")]));

  let officialAuthorizedEchoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: authorizedBufferKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: feePayer.publicKey,
        isSigner: true,
        isWritable: false,
      },
    ],
    data: Buffer.concat([idx3, Buffer.from(new Uint8Array(new BN(authEchoInput.length).toArray("le", 4))), Buffer.from(authEchoInput, "ascii")]),
    programId: programId,
  });

  let tx3 = new Transaction();
  tx3.add(officialAuthorizedEchoIx);

  let txid3 = await sendAndConfirmTransaction(
    connection,
    tx3,
    [feePayer],
    {
      skipPreflight: true,
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    }
  )

  console.log(`https://explorer.solana.com/tx/${txid3}?cluster=devnet`);
  const data3 = (await connection.getAccountInfo(authorizedBufferKey, "confirmed")).data;
  console.log("Printing authorized echo buffer: " + data3.toString());
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
