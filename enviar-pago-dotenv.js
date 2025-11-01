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
import dotenv from 'dotenv';

// Cargar variables de entorno
dotenv.config();

const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// Obtener valores desde variables de entorno
const SECRET_KEY = process.env.SECRET_KEY || 'SBXXX...';
const DESTINATION = process.env.DESTINATION || 'GBYYY...';

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

async function enviarPago(amount, memo = '') {
  try {
    if (SECRET_KEY === 'SBXXX...' || DESTINATION === 'GBYYY...') {
      console.log('⚠️  CONFIGURACIÓN REQUERIDA:');
      console.log('Crea un archivo .env con:');
      console.log('SECRET_KEY=tu_secret_key_aqui');
      console.log('DESTINATION=public_key_destino_aqui\n');
      return;
    }
    
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

// Ejecutar
const args = process.argv.slice(2);
const amount = args[0] || '25';
const memo = args[1] || '¡Mi primer pago con código! 🚀';

enviarPago(amount, memo);

