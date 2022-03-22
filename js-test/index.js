const { ApiPromise, WsProvider } = require('@polkadot/api');

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const rpc = {
  sumStorage: {
    get: {
      description: "Gets the sum of the two storage values in sum-storage pallet via a runtime api.",
      params: [],
      type: "u32",
    }
  }
}

async function main() {
  // Construct the actual api
  const api = await ApiPromise.create({
    provider: wsProvider,
    rpc,
  });


  // Query raw storage values, the oldschool way
  const v1 = ( await api.query.templateModule.something());
  const v2 = ( await api.query.templateModule.something2() );
  console.log(`The individual storage values are ${v1}, and ${v2}.`);
  console.log(`The sum calculated in javascript is ${v1 + v2}\n`);

  // Query the custom RPC that uses the runtimeAPI
  let directSum = ( await api.rpc.sumStorage.get() ).toNumber();
  console.log(`The sum queried directly from the RPC is ${directSum}`);
}

main().catch(console.error).finally(() => process.exit());
