import { serialize, deserialize, Schema } from "borsh";

export class Ticket { 
  gameid: number = 0;
  user_address: Uint8Array = new Uint8Array(32);
  participant_numbers:Uint8Array = new Uint8Array(5);


  constructor(fields: {gameid:number; participant_numbers:Uint8Array; user_address:Uint8Array} | undefined = undefined) {
    if (fields) {
      this.gameid = fields.gameid;
      this.participant_numbers = fields.participant_numbers;
      this.user_address = fields.user_address;
    }
  }
}

export const TicketShema = new Map([
  [Ticket, {
    kind: "struct",
    fields: [
      ["gameid", "u8"],
      ["user_address", ["u8",32]],
      ["participant_numbers", ["u8",5]],
    ]
  }]
]);

export class LottoGame { 
  gameid: number = 0;
  number_of_participants: number = 0;
  winning_numbers: Uint8Array = new Uint8Array(5);
  number_of_winner: number = 0;
  prize_pool: number = 0; 
  prize_amount: number = 0; 
  is_active: number = 0;
  ticket_money: number = 0; 

  constructor(fields: {gameid:number; number_of_participants:number; winning_numbers:Uint8Array; number_of_winner:number; prize_pool:number; prize_amount:number; is_active:number; ticket_money:number} | undefined = undefined) {
    if (fields) {
      this.gameid = fields.gameid;
      this.number_of_participants = fields.number_of_participants;
      this.winning_numbers = fields.winning_numbers;
      this.number_of_winner = fields.number_of_winner;
      this.prize_pool = fields.prize_pool;
      this.prize_amount = fields.prize_amount;
      this.is_active = fields.is_active;
      this.ticket_money = fields.ticket_money;
    }
  }
}
  
  export const LottoGameShema = new Map([
    [LottoGame, {
      kind: "struct",
      fields: [
        ["gameid", "u8"],
        ["number_of_participants", "u8"],
        ["winning_numbers", ["u8", 5]],
        ["number_of_winner", "u8"],
        ["prize_pool", "u64"],  
        ["prize_amount", "u64"], 
        ["is_active", "u8"],
        ["ticket_money", "u64"],
      ]
    }]
]);

  export class GameCount { 
    game_count: number = 0;
  
    constructor(fields: {game_count:number} | undefined = undefined) {
      if (fields) {
        this.game_count = fields.game_count;
      }
    }
  }
  
  export const GameCountShema = new Map([
    [GameCount, {
      kind: "struct",
      fields: [
        ["game_count", "u8"],
      ]
    }]
  ]);


  export class DrawData { 
    prize_amount: bigint = BigInt(0);
    winning_numbers:Uint8Array = new Uint8Array(5);

    constructor(fields: {prize_amount:bigint;winning_numbers:Uint8Array;} | undefined = undefined) {
      if (fields) {
        this.prize_amount = fields.prize_amount;
        this.winning_numbers = fields.winning_numbers;
      }
    }
  }
  
  export const DrawDataShema = new Map([
    [DrawData, {
      kind: "struct",
      fields: [
        ["prize_amount", "u64"],
        ["winning_numbers",  ["u8",5]],

      ]
    }]
  ]);