import { LAMPORTS_PER_SOL } from '@solana/web3.js';

import {
  establishConnection,
  calculatePayfees,
  establishPayer,
  loadProgram,
  storeNumber,
  getNumber,
} from './demo';

const solPath = 'dist/program/demo.so';

function randNumber(): number {
  let num = Math.random() * 100;
  return parseInt(num + '');
}

async function main() {
  try {
  
    console.log("Establish connection...");
    const res = await establishConnection('http://localhost:8899');
    console.log("Connection to cluster established: ", res.version);

    const { connection } = res;

    console.log("Caculate pay for fees...");
    const fees = await calculatePayfees(solPath, connection);
    console.log("Fees:", fees / LAMPORTS_PER_SOL);

    console.log("Establish payer...");
    const payer = await establishPayer(fees, connection);
    console.log("Using account ", payer.publicKey.toBase58(), " to load the program.");

    const program = await loadProgram(solPath, payer, connection);
    
    const { programId, pubkey } = program;
    console.log('Program loaded to account:', programId.toBase58());
    console.log('And created account: ', pubkey.toBase58(), 'to demonstrate the demo.');
    
    let num = randNumber();
    console.log("Prepare to store the number: ", num);

    await storeNumber(num + '', pubkey, programId, payer, connection);
    console.log("Stored number success!");

    let storedNum = await getNumber(pubkey, connection);
    
    console.log("Get stored number from the chain: ", storedNum);

  } catch(err) {
    console.log(err);
  }
}

main();