import { error } from "console"
import * as bounty from "../sdk-ts/dist/cjs/index"
import * as web3 from "@solana/web3.js"
import { sendAndConfirmTransaction } from "../sdk-ts/dist/cjs/utils"
import * as bs58 from "bs58";


async function main() {
    // get the relayer from the args
    const args = process.argv.slice(2)
    if (args.length !== 1) throw error(400, 'Invalid number of arguments')
    const relayerArg = args[0]
    // load wallet from env 
    const secretKey = process.env.WALLET_SECRET_KEY
    if (!secretKey) throw error(400, 'No wallet secret key found')

    const wallet = web3.Keypair.fromSecretKey(bs58.decode(secretKey))

    // setup connection from env rpc url
    const rpcUrl = process.env.RPC_URL
    if (!rpcUrl) throw error(400, 'No rpc url found')
    console.log("Connecting to rpc url: ", rpcUrl)
    const connection = new web3.Connection(rpcUrl)
    const latestBlockhash = await connection.getLatestBlockhash();
    console.log(`Latest blockhash: ${latestBlockhash.blockhash}`)
    // create initialize identity transaction
    const bountySdk = new bounty.BountySdk(wallet.publicKey, connection);
    const relayer = new web3.PublicKey(relayerArg)
    const addRelayer = await bountySdk.addRelayer(relayer)
    await sendAndConfirmTransaction(connection, await addRelayer.vtx, [wallet], latestBlockhash)
    console.log(`Relayer ${addRelayer.relayerPda.toBase58()} added for ${relayer.toString()}`)
}

main().catch(err => {
    console.error(err)
    process.exit(1)
})