import readline from 'readline';
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
import fs from 'fs';

const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// Archivo para guardar la cuenta cargada
const ARCHIVO_CUENTA = 'wallet-cuenta.json';

// Variable global para la cuenta cargada
let cuentaActual = null;

// Configurar readline
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

// Función helper para hacer preguntas
function pregunta(pregunta) {
  return new Promise((resolve) => {
    rl.question(pregunta, (respuesta) => {
      resolve(respuesta);
    });
  });
}

// Función para crear nueva cuenta
async function crearCuenta() {
  console.log('\n🔐 Generando nueva cuenta...\n');
  
  const pair = Keypair.random();
  const publicKey = pair.publicKey();
  const secretKey = pair.secret();
  
  console.log('✅ ¡Cuenta creada!\n');
  console.log('📧 PUBLIC KEY:', publicKey);
  console.log('🔑 SECRET KEY:', secretKey);
  
  // Fondear con Friendbot
  console.log('\n💰 Fondeando con Friendbot...');
  
  try {
    const response = await fetch(
      `https://friendbot.stellar.org/?addr=${publicKey}`
    );
    
    const result = await response.json();
    
    if (result.successful || response.ok) {
      console.log('✅ ¡Cuenta fondeada con 10,000 XLM!');
      console.log('🔗 Transaction hash:', result.hash);
    }
  } catch (error) {
    console.error('❌ Error al fondear:', error.message);
  }
  
  console.log('\n⚠️  IMPORTANTE: Guarda estas llaves en un lugar seguro');
  
  // Preguntar si quiere cargar esta cuenta
  const cargar = await pregunta('\n¿Deseas cargar esta cuenta ahora? (s/n): ');
  if (cargar.toLowerCase() === 's' || cargar.toLowerCase() === 'si') {
    cuentaActual = {
      publicKey,
      secretKey
    };
    
    // Guardar en archivo
    fs.writeFileSync(ARCHIVO_CUENTA, JSON.stringify(cuentaActual, null, 2));
    console.log('\n✅ Cuenta cargada y guardada en wallet-cuenta.json');
  }
}

// Función para cargar cuenta existente
async function cargarCuenta() {
  console.log('\n📂 Cargar cuenta existente\n');
  
  // Intentar cargar desde archivo
  if (fs.existsSync(ARCHIVO_CUENTA)) {
    const cargarArchivo = await pregunta(`¿Cargar desde ${ARCHIVO_CUENTA}? (s/n): `);
    if (cargarArchivo.toLowerCase() === 's' || cargarArchivo.toLowerCase() === 'si') {
      try {
        const data = JSON.parse(fs.readFileSync(ARCHIVO_CUENTA, 'utf8'));
        cuentaActual = data;
        console.log('✅ Cuenta cargada desde archivo');
        console.log('📧 Public Key:', cuentaActual.publicKey.substring(0, 16) + '...');
        return;
      } catch (error) {
        console.error('❌ Error cargando archivo:', error.message);
      }
    }
  }
  
  // Cargar manualmente
  const secretKey = await pregunta('Ingresa tu Secret Key: ');
  
  try {
    const keys = Keypair.fromSecret(secretKey);
    cuentaActual = {
      publicKey: keys.publicKey(),
      secretKey: secretKey
    };
    
    // Guardar en archivo
    fs.writeFileSync(ARCHIVO_CUENTA, JSON.stringify(cuentaActual, null, 2));
    console.log('✅ Cuenta cargada exitosamente');
    console.log('📧 Public Key:', cuentaActual.publicKey);
  } catch (error) {
    console.error('❌ Error: Secret key inválida');
    cuentaActual = null;
  }
}

// Función para ver balance
async function verBalance() {
  if (!cuentaActual) {
    console.log('\n❌ No hay cuenta cargada. Por favor carga una cuenta primero.');
    return;
  }
  
  console.log('\n🔍 Consultando balance...\n');
  
  try {
    const account = await server.loadAccount(cuentaActual.publicKey);
    
    console.log('╔═══════════════════════════════════╗');
    console.log('📊 INFORMACIÓN DE CUENTA');
    console.log('╚═══════════════════════════════════╝\n');
    
    console.log(`📧 Account ID: ${account.id}`);
    console.log(`🔢 Sequence Number: ${account.sequenceNumber()}\n`);
    
    console.log('╔═══════════════════════════════════╗');
    console.log('💰 BALANCES');
    console.log('╚═══════════════════════════════════╝\n');
    
    account.balances.forEach((balance, index) => {
      if (balance.asset_type === 'native') {
        console.log(`${index + 1}. 🌟 XLM (Lumens):`);
        console.log(`   Total: ${balance.balance} XLM`);
        
        const baseReserve = 0.5;
        const subentryReserve = account.subentry_count * 0.5;
        const totalReserve = baseReserve + subentryReserve;
        const available = parseFloat(balance.balance) - totalReserve;
        
        console.log(`   Bloqueado: ${totalReserve.toFixed(7)} XLM`);
        console.log(`   Disponible: ${available.toFixed(7)} XLM\n`);
      } else {
        console.log(`${index + 1}. 🪙 ${balance.asset_code}:`);
        console.log(`   Balance: ${balance.balance}`);
        console.log(`   Emisor: ${balance.asset_issuer.substring(0, 8)}...\n`);
      }
    });
  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

// Función para enviar pago
async function enviarPago() {
  if (!cuentaActual) {
    console.log('\n❌ No hay cuenta cargada. Por favor carga una cuenta primero.');
    return;
  }
  
  console.log('\n💸 Enviar pago\n');
  
  const destination = await pregunta('Public Key del destinatario: ');
  const amount = await pregunta('Cantidad (XLM): ');
  const memo = await pregunta('Memo (opcional, presiona Enter para omitir): ');
  
  console.log('\n🚀 Enviando pago...\n');
  
  try {
    const sourceKeys = Keypair.fromSecret(cuentaActual.secretKey);
    const sourceAccount = await server.loadAccount(cuentaActual.publicKey);
    
    console.log(`Balance actual: ${sourceAccount.balances[0].balance} XLM\n`);
    
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: destination.trim(),
        asset: Asset.native(),
        amount: amount.trim()
      }))
      .addMemo(memo.trim() ? Memo.text(memo.trim()) : Memo.none())
      .setTimeout(30)
      .build();
    
    transaction.sign(sourceKeys);
    const result = await server.submitTransaction(transaction);
    
    console.log('🎉 ¡PAGO EXITOSO!\n');
    console.log(`💰 Enviaste: ${amount} XLM`);
    console.log(`📧 Destinatario: ${destination.substring(0, 16)}...`);
    console.log(`🔗 Hash: ${result.hash}\n`);
  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

// Función para ver historial (últimas transacciones)
async function verHistorial() {
  if (!cuentaActual) {
    console.log('\n❌ No hay cuenta cargada. Por favor carga una cuenta primero.');
    return;
  }
  
  console.log('\n📜 Consultando historial de transacciones...\n');
  
  try {
    const account = await server.loadAccount(cuentaActual.publicKey);
    
    // Obtener últimas transacciones
    const transactions = await server
      .transactions()
      .forAccount(cuentaActual.publicKey)
      .order('desc')
      .limit(10)
      .call();
    
    console.log('╔═══════════════════════════════════╗');
    console.log('📜 ÚLTIMAS TRANSACCIONES');
    console.log('╚═══════════════════════════════════╝\n');
    
    if (transactions.records.length === 0) {
      console.log('📭 No hay transacciones aún\n');
      return;
    }
    
    transactions.records.forEach((tx, index) => {
      console.log(`${index + 1}. Hash: ${tx.hash.substring(0, 16)}...`);
      console.log(`   Fecha: ${new Date(tx.created_at).toLocaleString()}`);
      console.log(`   Ledger: ${tx.ledger_attr}`);
      console.log(`   Operaciones: ${tx.operation_count}`);
      console.log('');
    });
  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

// Función para mostrar menú
function mostrarMenu() {
  console.log('\n╔═══════════════════════════════════╗');
  console.log('🦈 MI WALLET STELLAR');
  console.log('╚═══════════════════════════════════╝');
  console.log('\n1. Crear nueva cuenta');
  console.log('2. Cargar cuenta existente');
  console.log('3. Ver balance');
  console.log('4. Enviar pago');
  console.log('5. Ver historial');
  console.log('6. Salir\n');
  
  if (cuentaActual) {
    console.log(`✅ Cuenta cargada: ${cuentaActual.publicKey.substring(0, 16)}...\n`);
  }
}

// Función principal del menú
async function menu() {
  // Intentar cargar cuenta guardada al iniciar
  if (!cuentaActual && fs.existsSync(ARCHIVO_CUENTA)) {
    try {
      const data = JSON.parse(fs.readFileSync(ARCHIVO_CUENTA, 'utf8'));
      cuentaActual = data;
      console.log('\n✅ Cuenta cargada automáticamente desde archivo');
    } catch (error) {
      // Ignorar error, continuar sin cuenta cargada
    }
  }
  
  mostrarMenu();
  
  const opcion = await pregunta('Elige opción: ');
  
  switch (opcion.trim()) {
    case '1':
      await crearCuenta();
      await pregunta('\nPresiona Enter para continuar...');
      menu();
      break;
      
    case '2':
      await cargarCuenta();
      await pregunta('\nPresiona Enter para continuar...');
      menu();
      break;
      
    case '3':
      await verBalance();
      await pregunta('\nPresiona Enter para continuar...');
      menu();
      break;
      
    case '4':
      await enviarPago();
      await pregunta('\nPresiona Enter para continuar...');
      menu();
      break;
      
    case '5':
      await verHistorial();
      await pregunta('\nPresiona Enter para continuar...');
      menu();
      break;
      
    case '6':
      console.log('\n👋 ¡Hasta luego!');
      rl.close();
      process.exit(0);
      break;
      
    default:
      console.log('\n❌ Opción inválida');
      await pregunta('\nPresiona Enter para continuar...');
      menu();
  }
}

// Iniciar wallet
console.log('\n🚀 Iniciando Mi Wallet Stellar...');
menu();

