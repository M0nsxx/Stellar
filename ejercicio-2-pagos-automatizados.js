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
import fs from 'fs';

const Server = Horizon.Server;

const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

const ARCHIVO_CUENTAS = 'cuentas-tarea-clase2.json';

// ⚠️ CONFIGURACIÓN: Puedes configurar manualmente O usar las cuentas del Ejercicio 1
// Opción 1: Configurar manualmente (descomenta y reemplaza)
// const SECRET_KEY = 'SBXXX...'; // Tu secret key (cuenta fuente que envía)
// const DESTINATARIOS_MANUALES = [
//   { publicKey: 'G...1', memo: 'Pago-001' },
//   { publicKey: 'G...2', memo: 'Pago-002' },
//   { publicKey: 'G...3', memo: 'Pago-003' }
// ];

// Opción 2: Leer del archivo del Ejercicio 1 (automático)
let SECRET_KEY = null;
let DESTINATARIOS_MANUALES = null;

/**
 * 🚀 EJERCICIO 2: SISTEMA DE PAGOS AUTOMATIZADO
 * 
 * Objetivo: Crear un sistema que envíe pagos a múltiples destinos
 * - Enviar 2 XLM a 3 cuentas diferentes en una sola ejecución
 * - Cada pago debe tener un memo único identificando el número de transacción
 * - Verificar que cada transacción fue exitosa antes de proceder con la siguiente
 * - Mostrar el hash de cada transacción para seguimiento
 */

const MONTO_XLM = '2'; // 2 XLM como se especifica en el ejercicio

/**
 * Función para cargar configuración desde archivo o usar configuración manual
 */
function cargarConfiguracion() {
  // Intentar cargar desde archivo del Ejercicio 1
  try {
    if (fs.existsSync(ARCHIVO_CUENTAS)) {
      const datos = JSON.parse(fs.readFileSync(ARCHIVO_CUENTAS, 'utf8'));
      
      if (datos.cuentas && datos.cuentas.length > 0) {
        // Usar la primera cuenta como fuente (SECRET_KEY)
        SECRET_KEY = datos.cuentas[0].secretKey;
        
        // Usar las siguientes 3 cuentas como destinatarios
        const cuentasDisponibles = datos.cuentas.slice(1); // Excluir la primera (es la fuente)
        
        if (cuentasDisponibles.length >= 3) {
          DESTINATARIOS_MANUALES = [
            { publicKey: cuentasDisponibles[0].publicKey, memo: 'Pago-001' },
            { publicKey: cuentasDisponibles[1].publicKey, memo: 'Pago-002' },
            { publicKey: cuentasDisponibles[2].publicKey, memo: 'Pago-003' }
          ];
          
          console.log(`✅ Configuración cargada desde: ${ARCHIVO_CUENTAS}`);
          console.log(`   📧 Cuenta fuente: ${datos.cuentas[0].publicKey.substring(0, 16)}...`);
          console.log(`   📋 Destinatarios: ${cuentasDisponibles.length} cuentas\n`);
        } else {
          console.log(`⚠️  Archivo encontrado pero necesitas al menos 4 cuentas (1 fuente + 3 destinatarios)`);
          console.log(`   Ejecuta primero el Ejercicio 1 para crear más cuentas\n`);
        }
      }
    }
  } catch (error) {
    console.log(`⚠️  No se pudo cargar configuración desde archivo: ${error.message}`);
  }
  
  // Si no se cargó desde archivo, usar configuración manual (si está definida)
  // Descomenta y configura abajo si prefieres configurar manualmente:
  
  // SECRET_KEY = 'SBXXX...'; // Descomenta y reemplaza con tu secret key
  // DESTINATARIOS_MANUALES = [
  //   { publicKey: 'G...1', memo: 'Pago-001' },
  //   { publicKey: 'G...2', memo: 'Pago-002' },
  //   { publicKey: 'G...3', memo: 'Pago-003' }
  // ];
}

// Cargar configuración al inicio
cargarConfiguracion();

// Array de destinatarios (se llena desde archivo o manual)
const destinatarios = DESTINATARIOS_MANUALES || [
  { publicKey: 'G...1', memo: 'Pago-001' },
  { publicKey: 'G...2', memo: 'Pago-002' },
  { publicKey: 'G...3', memo: 'Pago-003' }
];

/**
 * Función para enviar un pago individual
 */
async function enviarPago(destinatario, monto, memo) {
  try {
    console.log(`📤 Enviando ${monto} XLM a ${destinatario.substring(0, 8)}...`);
    console.log(`   📝 Memo: ${memo}`);
    
    // Paso 1: Cargar cuenta fuente (siempre recargar para obtener sequence actualizado)
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    // Paso 2: Construir transacción
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: destinatario,
        asset: Asset.native(),
        amount: monto.toString()
      }))
      .addMemo(Memo.text(memo))
      .setTimeout(30)
      .build();
    
    // Paso 3: Firmar
    transaction.sign(sourceKeys);
    
    // Paso 4: Enviar
    const result = await server.submitTransaction(transaction);
    
    console.log(`✅ ¡Pago exitoso!`);
    console.log(`   🔗 Hash: ${result.hash}\n`);
    
    return {
      destinatario: destinatario,
      memo: memo,
      monto: monto,
      hash: result.hash,
      exito: true
    };
    
  } catch (error) {
    console.error(`❌ Error enviando pago a ${destinatario.substring(0, 8)}...`);
    console.error(`   Error: ${error.message}\n`);
    
    return {
      destinatario: destinatario,
      memo: memo,
      monto: monto,
      hash: null,
      exito: false,
      error: error.message
    };
  }
}

/**
 * Función principal: Enviar pagos a múltiples destinos
 */
async function enviarPagosAutomatizados() {
  console.log('╔═══════════════════════════════════════════╗');
  console.log('🚀 EJERCICIO 2: SISTEMA DE PAGOS AUTOMATIZADO');
  console.log('╚═══════════════════════════════════════════╝');
  console.log(`\n📋 Enviando ${MONTO_XLM} XLM a ${destinatarios.length} destinatarios...\n`);
  
  // Verificar que la SECRET_KEY esté configurada
  if (!SECRET_KEY || SECRET_KEY === 'SBXXX...' || SECRET_KEY.startsWith('SBXXX')) {
    console.error('❌ ERROR: Debes configurar tu SECRET_KEY');
    console.log('\n💡 OPCIONES:');
    console.log('   1. Ejecuta primero el Ejercicio 1 para crear cuentas automáticamente');
    console.log(`   2. Edita el archivo y configura SECRET_KEY manualmente (línea ~20)`);
    console.log('   3. Descomenta y configura SECRET_KEY en la función cargarConfiguracion()\n');
    process.exit(1);
  }
  
  // Verificar que los destinatarios estén configurados
  if (!destinatarios || destinatarios.length === 0 || destinatarios[0].publicKey === 'G...1' || destinatarios[0].publicKey.startsWith('G...')) {
    console.error('❌ ERROR: Debes configurar los destinatarios');
    console.log('\n💡 OPCIONES:');
    console.log('   1. Ejecuta primero el Ejercicio 1 para crear cuentas automáticamente');
    console.log(`   2. Edita el archivo y configura DESTINATARIOS_MANUALES (línea ~20)`);
    console.log('   3. Descomenta y configura DESTINATARIOS_MANUALES en la función cargarConfiguracion()\n');
    process.exit(1);
  }
  
  const resultados = [];
  
  // Enviar pagos uno por uno (verificando éxito antes de continuar)
  for (let i = 0; i < destinatarios.length; i++) {
    const destinatario = destinatarios[i];
    
    console.log(`\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
    console.log(`   PAGO ${i + 1} DE ${destinatarios.length}`);
    console.log(`━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
    
    const resultado = await enviarPago(
      destinatario.publicKey,
      MONTO_XLM,
      destinatario.memo
    );
    
    resultados.push(resultado);
    
    // Verificar éxito antes de continuar con el siguiente
    if (!resultado.exito) {
      console.log(`⚠️  Transacción ${i + 1} falló. Deteniendo envío de pagos adicionales.`);
      console.log(`   Puedes revisar el error y continuar manualmente si es necesario.\n`);
      break;
    }
    
    // Pequeña pausa entre transacciones
    if (i < destinatarios.length - 1) {
      await new Promise(resolve => setTimeout(resolve, 2000));
    }
  }
  
  // Resumen final
  console.log('\n╔═══════════════════════════════════════════╗');
  console.log('📊 RESUMEN DE TRANSMISIONES');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  let exitosos = 0;
  let fallidos = 0;
  
  resultados.forEach((resultado, index) => {
    if (resultado.exito) {
      exitosos++;
      console.log(`✅ Pago ${index + 1} (${resultado.memo}):`);
      console.log(`   Destinatario: ${resultado.destinatario.substring(0, 16)}...`);
      console.log(`   Monto: ${resultado.monto} XLM`);
      console.log(`   Hash: ${resultado.hash}`);
      console.log('');
    } else {
      fallidos++;
      console.log(`❌ Pago ${index + 1} (${resultado.memo}): FALLIDO`);
      console.log(`   Error: ${resultado.error}`);
      console.log('');
    }
  });
  
  console.log(`📊 Total exitosos: ${exitosos}`);
  console.log(`📊 Total fallidos: ${fallidos}`);
  console.log(`📊 Total intentados: ${resultados.length}\n`);
  
  return resultados;
}

// Ejecutar ejercicio
enviarPagosAutomatizados().catch(error => {
  console.error('❌ Error fatal:', error.message);
  process.exit(1);
});

