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
  import { Ticket,TicketShema,LottoGame,LottoGameShema,GameCount,GameCountShema,Authority,AuthorityShema } from "./models";
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const privatekey = [209,202,75,77,51,59,102,81,8,45,50,58,209,54,134,238,29,107,221,66,98,156,30,20,186,236,255,189,136,8,36,169,49,191,167,29,47,172,73,19,16,188,51,135,9,154,137,226,181,182,26,127,251,38,99,119,117,149,77,134,182,216,216,215]
  const payer = Keypair.fromSecretKey(Uint8Array.from(privatekey));
 
  const program_id =  new PublicKey("GCM1cTD8Ha5Q7Xh9kfNXET8joT6kXwBhuHnjSLRCvXvv");
