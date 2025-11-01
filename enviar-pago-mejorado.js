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

// ⚠️ REEMPLAZA ESTAS LLAVES CON LAS TUYAS
const SECRET_KEY = 'SBXXX...'; // Tu secret key
const DESTINATION = 'GBYYY...'; // Cuenta destino

// Función para validar amount
function validarAmount(amount) {
  const num = parseFloat(amount);
  if (isNaN(num) || num <= 0) {
    throw new Error('Amount debe ser un número positivo');
  }
  if (num > 1000000) {
    throw new Error('Amount demasiado grande (máximo 1,000,000 XLM)');
  }
  return num;
}

// Función principal mejorada
async function enviarPago(amount, memo = '') {
  try {
    console.log('🚀 Iniciando pago...\n');
    
    // Validar amount
    const amountValidado = validarAmount(amount);
    
    // Paso 1: Cargar tu cuenta
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    const balanceActual = parseFloat(sourceAccount.balances[0].balance);
    console.log(`Balance actual: ${balanceActual} XLM\n`);
    
    // Validar balance antes de enviar
    const baseReserve = 0.5;
    const subentryReserve = sourceAccount.subentry_count * 0.5;
    const totalReserve = baseReserve + subentryReserve;
    const balanceDisponible = balanceActual - totalReserve;
    
    if (amountValidado > balanceDisponible) {
      throw new Error(`Balance insuficiente. Disponible: ${balanceDisponible.toFixed(7)} XLM`);
    }
    
    // Paso 2: Construir transacción
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: DESTINATION,
        asset: Asset.native(),
        amount: amountValidado.toString()
      }))
      .addMemo(memo ? Memo.text(memo) : Memo.none())
      .setTimeout(30)
      .build();
    
    // Paso 3: Firmar
    transaction.sign(sourceKeys);
    
    // Paso 4: Enviar
    const result = await server.submitTransaction(transaction);
    
    console.log('🎉 ¡PAGO EXITOSO!\n');
    console.log(`💰 Enviaste: ${amountValidado} XLM`);
    console.log(`🔗 Hash: ${result.hash}\n`);
    
    return result;
    
  } catch (error) {
    console.error('❌ ERROR:', error.message);
    throw error;
  }
}

// Función para enviar múltiples pagos
async function enviarVariosPagos(destinatarios, amount) {
  console.log(`📤 Enviando ${amount} XLM a ${destinatarios.length} destinatarios...\n`);
  
  for (let i = 0; i < destinatarios.length; i++) {
    const dest = destinatarios[i];
    console.log(`📧 Enviando a destinatario ${i + 1}/${destinatarios.length}: ${dest.substring(0, 8)}...`);
    
    try {
      // Cargar cuenta para cada pago
      const sourceKeys = Keypair.fromSecret(SECRET_KEY);
      const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
      
      const transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: networkPassphrase
      })
        .addOperation(Operation.payment({
          destination: dest,
          asset: Asset.native(),
          amount: amount.toString()
        }))
        .addMemo(Memo.text(`Pago ${i + 1} de ${destinatarios.length}`))
        .setTimeout(30)
        .build();
      
      transaction.sign(sourceKeys);
      const result = await server.submitTransaction(transaction);
      
      console.log(`✅ Enviado a ${dest.substring(0, 12)}... | Hash: ${result.hash.substring(0, 16)}...\n`);
      
      // Esperar un poco entre pagos para evitar problemas de sequence number
      await new Promise(resolve => setTimeout(resolve, 2000));
      
    } catch (error) {
      console.error(`❌ Error enviando a ${dest.substring(0, 8)}...: ${error.message}\n`);
    }
  }
  
  console.log('✅ Proceso de envío múltiple completado\n');
}

// Función para enviar múltiples operaciones en una sola transacción
async function enviarMultiplesPagosEnUnaTx(destinatarios, amount) {
  console.log(`📤 Enviando ${amount} XLM a ${destinatarios.length} destinatarios en UNA transacción...\n`);
  
  try {
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    console.log(`Balance actual: ${sourceAccount.balances[0].balance} XLM\n`);
    
    // Construir transacción con múltiples operaciones
    let transactionBuilder = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    });
    
    // Agregar una operación de pago por cada destinatario
    destinatarios.forEach((dest, index) => {
      transactionBuilder = transactionBuilder.addOperation(Operation.payment({
        destination: dest,
        asset: Asset.native(),
        amount: amount.toString()
      }));
    });
    
    const transaction = transactionBuilder
      .addMemo(Memo.text(`Pago múltiple a ${destinatarios.length} cuentas`))
      .setTimeout(30)
      .build();
    
    transaction.sign(sourceKeys);
    const result = await server.submitTransaction(transaction);
    
    console.log('🎉 ¡PAGOS MÚLTIPLES EXITOSOS!\n');
    console.log(`💰 ${destinatarios.length} pagos de ${amount} XLM cada uno`);
    console.log(`📦 Fee total: ${BASE_FEE * (destinatarios.length + 1)} stroops (1 sola transacción)`);
    console.log(`🔗 Hash: ${result.hash}\n`);
    
    return result;
    
  } catch (error) {
    console.error('❌ ERROR:', error.message);
    throw error;
  }
}

// Ejecutar según argumentos
const args = process.argv.slice(2);

if (args.includes('--multiples')) {
  const cuentas = args.filter((arg, i) => args[i - 1] === '--cuentas' || (i > 0 && !args[i - 1].startsWith('--')));
  const amountIdx = args.indexOf('--amount') + 1;
  const amount = args[amountIdx] || '10';
  
  // Ejemplo de uso:
  // node enviar-pago-mejorado.js --multiples --cuentas GBXXX... GBYYY... GBZZZ... --amount 10
  if (cuentas.length > 0 && args.includes('--cuentas')) {
    enviarVariosPagos(cuentas, amount);
  } else {
    console.log('Uso: --multiples --cuentas PUBLIC_KEY1 PUBLIC_KEY2 ... --amount CANTIDAD');
  }
} else if (args.includes('--una-tx')) {
  const cuentas = args.filter(arg => arg.startsWith('G'));
  const amount = args.find(arg => !isNaN(parseFloat(arg))) || '10';
  
  if (cuentas.length > 0) {
    enviarMultiplesPagosEnUnaTx(cuentas, amount);
  } else {
    console.log('Uso: --una-tx PUBLIC_KEY1 PUBLIC_KEY2 ... CANTIDAD');
  }
} else {
  // Uso normal
  const amount = args[0] || '25';
  const memo = args[1] || '¡Mi primer pago con código! 🚀';
  enviarPago(amount, memo);
}

