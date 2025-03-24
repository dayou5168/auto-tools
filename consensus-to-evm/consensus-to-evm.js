import { activateWallet, createConnection } from '@autonomys/auto-utils';
import { transferToDomainAccount20Type } from '@autonomys/auto-xdm';
import dotenv from 'dotenv';
dotenv.config();

const PRIVATE_KEY = process.env.PRIVATE_KEY;
const WS_ENDPOINT = process.env.WS_ENDPOINT;
const EVM_ADDRESS = process.env.EVM_ADDRESS

const private_node_api = await createConnection(WS_ENDPOINT);

(async () => {
  const { api, accounts, address } = await activateWallet({
    networkId: 'taurus',
    uri: PRIVATE_KEY,
    api: private_node_api
  });

  //Check if account is loaded.
  const account = accounts[0];
  if (!account) {
    throw new Error('No account available');
  }

  // print source address to validate that it's your address. for debuging.
  console.log(address);

  const tx = await transferToDomainAccount20Type(
    api,
    '0', // Receiver domain (0 is Auto EVM on the Taurus testnet)
    EVM_ADDRESS, // Receiver EVM address
    '1000000' // Amount in smallest unit (Shannon)
  );
  const hash = await tx.signAndSend(account);
  console.log(hash.toHuman());
  await api.disconnect();
})();
