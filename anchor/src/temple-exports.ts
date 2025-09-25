// Here we export some useful types and functions for interacting with the Anchor program.
import { Account, address, getBase58Decoder, SolanaClient } from 'gill'
import { SolanaClusterId } from '@wallet-ui/react'
import { getProgramAccountsDecoded } from './helpers/get-program-accounts-decoded'
import { Temple, TEMPLE_DISCRIMINATOR, TEMPLE_PROGRAM_ADDRESS, getTempleDecoder } from './client/js'
import TempleIDL from '../target/idl/temple.json'

export type TempleAccount = Account<Temple, string>

// Re-export the generated IDL and type
export { TempleIDL }

// This is a helper function to get the program ID for the Temple program depending on the cluster.
export function getTempleProgramId(cluster: SolanaClusterId) {
  switch (cluster) {
    case 'solana:devnet':
    case 'solana:testnet':
      // This is the program ID for the Temple program on devnet and testnet.
      return address('6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF')
    case 'solana:mainnet':
    default:
      return TEMPLE_PROGRAM_ADDRESS
  }
}

export * from './client/js'

export function getTempleProgramAccounts(rpc: SolanaClient['rpc']) {
  return getProgramAccountsDecoded(rpc, {
    decoder: getTempleDecoder(),
    filter: getBase58Decoder().decode(TEMPLE_DISCRIMINATOR),
    programAddress: TEMPLE_PROGRAM_ADDRESS,
  })
}
