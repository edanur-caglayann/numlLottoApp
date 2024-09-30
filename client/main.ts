import {
    Connection,
    Keypair,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram,
    TransactionInstruction,
    LAMPORTS_PER_SOL,
    Transaction,
    sendAndConfirmTransaction,
  
  } from "@solana/web3.js";
  import {deserialize, deserializeUnchecked, serialize } from "borsh";
  import { Ticket,TicketShema,LottoGame,LottoGameShema,GameCount,GameCountShema, DrawData, DrawDataShema} from "./models";
import BN from "bn.js";
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const privatekey = [209,202,75,77,51,59,102,81,8,45,50,58,209,54,134,238,29,107,221,66,98,156,30,20,186,236,255,189,136,8,36,169,49,191,167,29,47,172,73,19,16,188,51,135,9,154,137,226,181,182,26,127,251,38,99,119,117,149,77,134,182,216,216,215]
  const payer = Keypair.fromSecretKey(Uint8Array.from(privatekey));
 
  const program_id =  new PublicKey("GCM1cTD8Ha5Q7Xh9kfNXET8joT6kXwBhuHnjSLRCvXvv");
  const gameCount = new PublicKey("9FmgST2J6SqEVDdR9nZxApWVkvqExnNcTGFsxrqcJ4Jd");
  const lottoGame = new PublicKey("37qRdCyfJgTmemyCZHeU29jbrypyJbh4gMCe33afXEK5");
  const ticketAcc = new PublicKey("DcwVfkhQXeuPACsfrkZQpDaTir4YmHaKRB9eEgkASthk");

  const game_count = async() => {
      const game_Count = new GameCount();
      game_Count.game_count = 0;
  
      const encoded = serialize(GameCountShema, game_Count);
      const concat = Uint8Array.of(1, ...encoded);
  
      const gameCountPDA = PublicKey.findProgramAddressSync([Buffer.from("count")],program_id);
  
      const instruction = new TransactionInstruction({
        keys: [
          {pubkey: payer.publicKey, isSigner: true, isWritable: true},
          {pubkey: gameCountPDA[0], isSigner: false, isWritable: true},
          {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        data: Buffer.from(concat),
        programId: program_id
      })
      const message = new TransactionMessage({
        instructions: [instruction],
        payerKey: payer.publicKey,
        recentBlockhash: (await connection.getLatestBlockhash()).blockhash
      }).compileToV0Message();
    
      
      const tx = new VersionedTransaction(message);
       tx.sign([payer]);
    
      connection.sendTransaction(tx);
      console.log("Game Count PDA => " + gameCountPDA[0].toString())
  }
    
  const create_lotto_game = async() => {
    const lottoGame = new LottoGame();
    lottoGame.gameid = 1;
    lottoGame.number_of_participants = 0;
    lottoGame.winning_numbers = new Uint8Array([0, 0, 0, 0, 0]);
    lottoGame.number_of_winner = 0;
    lottoGame.prize_pool = 0;
    lottoGame.prize_amount = 15;
    lottoGame.is_active = 1;

    const encoded = serialize(LottoGameShema, lottoGame);
    const concat = Uint8Array.of(2, ...encoded);

    const gameCountData = await connection.getAccountInfo(gameCount);
    const gameCountDatadeserizalize = deserialize(GameCountShema, GameCount, gameCountData!.data);

    const convert_type: bigint =  BigInt (gameCountDatadeserizalize.game_count); 
    const add = BigInt(1) + convert_type;

    const lottoGamePDA = PublicKey.findProgramAddressSync([Buffer.from("lotto_game"),Buffer.from(add.toString())],program_id);

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: gameCount, isSigner: false, isWritable: true},
        {pubkey: lottoGamePDA[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},

      ],
      data: Buffer.from(concat),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("Lotto Game account => " + lottoGamePDA[0])
    
  }

  const read_game = async() => {
    const game_inf = await connection.getAccountInfo(lottoGame)
    const game_inf_deserizalize = deserialize(LottoGameShema, LottoGame, game_inf!.data);

   console.log("Game id = " + game_inf_deserizalize.gameid);
   console.log("Is the game active? = " + game_inf_deserizalize.is_active);
   console.log("Number of participants = " + game_inf_deserizalize.number_of_participants);
   console.log("Prize Amount = " + game_inf_deserizalize.prize_amount);
   console.log("Prize pool = " + game_inf_deserizalize.prize_pool);
   console.log("Number of winner = " + game_inf_deserizalize.number_of_winner);
   console.log("Winner numbers = " + game_inf_deserizalize.winning_numbers);
   console.log("Ticket money = " + game_inf_deserizalize.ticket_money);
   console.log("Lamports = " + game_inf?.lamports.toString());

  }

  const ticket = async(participant_numbers:Uint8Array) => {
    
    const lottoGameData = await connection.getAccountInfo(lottoGame);
    const lottoGameDatadeserizalize = deserialize(LottoGameShema, LottoGame, lottoGameData!.data);

    lottoGameDatadeserizalize.number_of_participants += 1;

    const ticketPDA = PublicKey.findProgramAddressSync([Buffer.from("ticket"),Buffer.from(lottoGameDatadeserizalize.gameid.toString()),Buffer.from("gameNo"),Buffer.from(lottoGameDatadeserizalize.number_of_participants.toString()) ],program_id);
    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: lottoGame, isSigner: false, isWritable: true},
        {pubkey: ticketPDA[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},

      ],
      data: Buffer.from([3, ...participant_numbers]),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("Ticket account => " + ticketPDA[0])
  }

  const read_ticket = async() => {
    const ticket_inf = await connection.getAccountInfo(ticketAcc)
    const ticket_inf_deserizalize = deserialize(TicketShema, Ticket, ticket_inf!.data);

    console.log("Game id = " + ticket_inf_deserizalize.gameid);
    console.log("User address = " + ticket_inf_deserizalize.user_address);
    console.log("Participant numbers = " + ticket_inf_deserizalize.participant_numbers);
  }

  const draw = async(prizeAmount:bigint, winningNumbers:Uint8Array ) => {
    const drawdata = new DrawData();
    drawdata.prize_amount = BigInt(prizeAmount);
    drawdata.winning_numbers = winningNumbers;

    const encoded = serialize(DrawDataShema, drawdata);
    const concat = Uint8Array.of(4, ...encoded);

    const instruction = new TransactionInstruction({
        keys: [
            {pubkey: payer.publicKey, isSigner: true, isWritable: true}, 
            {pubkey: lottoGame, isSigner: false, isWritable: true},
            {pubkey: ticketAcc, isSigner: false, isWritable: true},
        ],
        data:Buffer.from(concat),
        programId: program_id, 
    });

    const message = new TransactionMessage({
        instructions: [instruction],
        payerKey: payer.publicKey,
        recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();

    const tx = new VersionedTransaction(message);
    tx.sign([payer]); 

    connection.sendTransaction(tx);
    console.log("Prize updated!!");
    console.log(`Lotto Game Account: ${lottoGame.toBase58()}`);
    console.log(`Payer Account: ${payer.publicKey.toBase58()}`);
  }

  const claim_prize = async() => {

    const lottoGameData = await connection.getAccountInfo(lottoGame);
    const lottoGameDatadeserizalize = deserialize(LottoGameShema, LottoGame, lottoGameData!.data);

    const ticketData = await connection.getAccountInfo(ticketAcc);
    const ticketDatadeserizalize = deserialize(TicketShema, Ticket, ticketData!.data);

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: lottoGame, isSigner: false, isWritable: true},
        {pubkey: ticketAcc, isSigner: false, isWritable: true},
      ],
      data: Buffer.from([5]),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("The prize has been distributed!");
    console.log(`Lotto Game Account: ${lottoGame.toBase58()}`);
    console.log(`Ticket Account: ${ticketAcc.toBase58()}`);
    console.log(`Payer Account: ${payer.publicKey.toBase58()}`);

  }

  // game_count()
  // create_lotto_game()
  // read_game()
  // ticket(new Uint8Array([9, 1, 3, 5, 6]))
  // read_ticket()
  // draw(BigInt(15),new Uint8Array([9, 1, 3, 5, 6]))
  claim_prize()