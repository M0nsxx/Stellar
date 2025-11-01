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

// ⚠️ CONFIGURACIÓN: Reemplaza con tus valores
const SECRET_KEY = 'SBXXX...'; // Tu secret key
const AMOUNT_POR_CUENTA = '10'; // XLM a enviar a cada cuenta

// Función principal de airdrop
async function airdrop(cuentas, amount) {
  console.log('🎁 INICIANDO AIRDROP\n');
  console.log(`📤 Enviando ${amount} XLM a ${cuentas.length} cuentas...\n`);
  
  if (cuentas.length === 0) {
    console.log('❌ No hay cuentas destinatarias');
    return;
  }
  
  let exitosos = 0;
  let fallidos = 0;
  
  for (let i = 0; i < cuentas.length; i++) {
    const cuenta = cuentas[i];
    
    console.log(`📧 [${i + 1}/${cuentas.length}] Enviando a ${cuenta.substring(0, 12)}...`);
    
    try {
      // Cargar cuenta origen para cada pago (necesario para sequence number)
      const sourceKeys = Keypair.fromSecret(SECRET_KEY);
      const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
      
      // Validar balance antes de enviar
      const balance = parseFloat(sourceAccount.balances[0].balance);
      const amountNum = parseFloat(amount);
      
      if (balance < amountNum + 0.5) { // 0.5 XLM de reserve mínimo
        console.log(`⚠️  Balance insuficiente. Balance: ${balance} XLM\n`);
        break;
      }
      
      // Construir transacción
      const transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: networkPassphrase
      })
        .addOperation(Operation.payment({
          destination: cuenta,
          asset: Asset.native(),
          amount: amount.toString()
        }))
        .addMemo(Memo.text(`Airdrop #${i + 1}`))
        .setTimeout(30)
        .build();
      
      // Firmar y enviar
      transaction.sign(sourceKeys);
      const result = await server.submitTransaction(transaction);
      
      console.log(`✅ Enviado exitosamente | Hash: ${result.hash.substring(0, 16)}...\n`);
      exitosos++;
      
      // Esperar 2 segundos entre pagos para evitar problemas de sequence number
      if (i < cuentas.length - 1) {
        await new Promise(resolve => setTimeout(resolve, 2000));
      }
      
    } catch (error) {
      console.error(`❌ Error: ${error.message}\n`);
      fallidos++;
    }
  }
  
  console.log('╔═══════════════════════════════════╗');
  console.log('📊 RESUMEN DEL AIRDROP');
  console.log('╚═══════════════════════════════════╝\n');
  console.log(`✅ Exitosos: ${exitosos}`);
  console.log(`❌ Fallidos: ${fallidos}`);
  console.log(`📦 Total enviado: ${exitosos * parseFloat(amount)} XLM\n`);
}

// Ejemplo de uso
const destinatarios = [
  'GBXXX...',
  'GBYYY...',
  'GBZZZ...',
  // Agrega más cuentas aquí
];

// Ejecutar airdrop
if (destinatarios.length > 0 && destinatarios[0] !== 'GBXXX...') {
  airdrop(destinatarios, AMOUNT_POR_CUENTA);
} else {
  console.log('⚠️  CONFIGURACIÓN REQUERIDA:');
  console.log('1. Reemplaza SECRET_KEY con tu secret key');
  console.log('2. Agrega las public keys de destino en el array destinatarios\n');
  console.log('Ejemplo de uso:');
  console.log('const destinatarios = [');
  console.log("  'GAUKLCK6LDYFDDECPCEGQIDN3C4E6MDAX5PA6WU5MRU63M2YA2BYOACV',");
  console.log("  'GBXXX...',");
  console.log('];\n');
}

