import {
  Keypair,
  TransactionBuilder,
  Networks,
  Operation,
  Asset,
  BASE_FEE,
  Memo,
  Horizon
} from '@stellar/stellar-sdk';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// Cuenta #1 (fuente - envía pagos)
const SECRET_KEY = 'SCHKVPBYYFG4KBG2JIKVXSNTPIQZYH4CQTSZIJ3L6KVEKG4SL22W7HLS';
const CUENTA_1 = 'GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF';

// Destinatarios: Cuentas #2, #3, #4
const destinatarios = [
  { numero: 1, publicKey: 'GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI', memo: 'Pago-001' },
  { numero: 2, publicKey: 'GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ', memo: 'Pago-002' },
  { numero: 3, publicKey: 'GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V', memo: 'Pago-003' }
];

async function enviarPago(sourceKey, destination, amount, memo) {
  try {
    const sourceKeys = Keypair.fromSecret(sourceKey);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: destination,
        asset: Asset.native(),
        amount: amount.toString()
      }))
      .addMemo(Memo.text(memo))
      .setTimeout(30)
      .build();
    
    transaction.sign(sourceKeys);
    
    const result = await server.submitTransaction(transaction);
    return result;
    
  } catch (error) {
    console.error('Error:', error.message);
    throw error;
  }
}

async function sistemaPagos(destinos, amount) {
  console.log('╔════════════════════════════════════════╗');
  console.log('💸 SISTEMA DE PAGOS AUTOMATIZADO');
  console.log('╚════════════════════════════════════════╝\n');
  
  const resultados = [];
  
  for (const destino of destinos) {
    try {
      console.log(`💰 [${destino.numero}/3] Enviando ${amount} XLM...`);
      console.log(`   Destino: ${destino.publicKey.substring(0, 8)}...`);
      console.log(`   Memo: ${destino.memo}`);
      
      const result = await enviarPago(SECRET_KEY, destino.publicKey, amount, destino.memo);
      
      resultados.push({
        numero: destino.numero,
        destino: destino.publicKey,
        monto: amount,
        memo: destino.memo,
        hash: result.hash,
        estado: '✅ EXITOSO'
      });
      
      console.log(`   ✅ Hash: ${result.hash}\n`);
      
    } catch (error) {
      console.error(`   ❌ Error: ${error.message}\n`);
      
      resultados.push({
        numero: destino.numero,
        destino: destino.publicKey,
        monto: amount,
        memo: destino.memo,
        estado: '❌ FALLIDO'
      });
    }
  }
  
  console.log('╔════════════════════════════════════════╗');
  console.log('📊 RESUMEN DE TRANSACCIONES');
  console.log('╚════════════════════════════════════════╝\n');
  
  resultados.forEach(r => {
    console.log(`${r.estado} - Pago ${r.numero}`);
    console.log(`   Monto: ${r.monto} XLM`);
    console.log(`   Memo: ${r.memo}`);
    if (r.hash) console.log(`   Hash: ${r.hash}`);
    console.log('');
  });
  
  return resultados;
}

const transacciones = await sistemaPagos(destinatarios, '2');

