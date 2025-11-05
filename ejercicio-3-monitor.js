import { Horizon } from '@stellar/stellar-sdk';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');

// Cuentas a monitorear: Cuentas #1, #2, #3, #4
const cuentasAMonitorear = [
  'GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF', // Cuenta #1
  'GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI', // Cuenta #2
  'GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ', // Cuenta #3
  'GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V'  // Cuenta #4
];

async function monitorearBalances(publicKeys) {
  console.log('╔════════════════════════════════════════╗');
  console.log('📊 MONITOR DE CUENTAS');
  console.log('╚════════════════════════════════════════╝\n');

  for (let i = 0; i < publicKeys.length; i++) {
    const pk = publicKeys[i];

    try {
      const account = await server.loadAccount(pk);

      const xlmBalance = account.balances[0].balance;
      const numTrustlines = account.balances.length - 1;
      const sequenceNumber = account.sequenceNumber();

      console.log(`📍 Cuenta ${i + 1}:`);
      console.log(`   Public Key: ${pk}`);
      console.log(`   💰 Balance: ${xlmBalance} XLM`);
      console.log(`   🔗 Trustlines: ${numTrustlines}`);
      console.log(`   🔢 Sequence: ${sequenceNumber}`);
      console.log('');

    } catch (error) {
      console.error(`❌ Error en ${pk.substring(0, 8)}...: ${error.message}`);
    }
  }
}

await monitorearBalances(cuentasAMonitorear);

