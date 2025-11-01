import { Horizon } from '@stellar/stellar-sdk';
import fs from 'fs';

const Server = Horizon.Server;

const server = new Server('https://horizon-testnet.stellar.org');

const ARCHIVO_CUENTAS = 'cuentas-tarea-clase2.json';

/**
 * 🔍 EJERCICIO 3: MONITOR DE BALANCES
 * 
 * Objetivo: Desarrollar un monitor que verifique balances de múltiples cuentas
 * - Aceptar un array de public keys como entrada
 * - Mostrar para cada cuenta:
 *   - Balance de XLM
 *   - Número de trustlines activos
 *   - Sequence number actual
 * - Formatear la salida de manera legible
 */

// ⚠️ CONFIGURACIÓN: Puedes configurar manualmente O usar las cuentas del Ejercicio 1
// Opción 1: Configurar manualmente (descomenta y reemplaza)
// const PUBLIC_KEYS_MANUALES = [
//   'GBXXX...',
//   'GBYYY...',
//   'GBZZZ...'
// ];

/**
 * Función para cargar configuración desde archivo o usar configuración manual
 */
function cargarConfiguracion() {
  let publicKeysFromFile = null;
  
  // Intentar cargar desde archivo del Ejercicio 1
  try {
    if (fs.existsSync(ARCHIVO_CUENTAS)) {
      const datos = JSON.parse(fs.readFileSync(ARCHIVO_CUENTAS, 'utf8'));
      
      if (datos.cuentas && datos.cuentas.length > 0) {
        // Usar todas las cuentas creadas para monitorear
        publicKeysFromFile = datos.cuentas.map(cuenta => cuenta.publicKey);
        
        console.log(`✅ Configuración cargada desde: ${ARCHIVO_CUENTAS}`);
        console.log(`   📋 Cuentas a monitorear: ${publicKeysFromFile.length}\n`);
      }
    }
  } catch (error) {
    console.log(`⚠️  No se pudo cargar configuración desde archivo: ${error.message}`);
  }
  
  // Si no se cargó desde archivo, usar configuración manual (si está definida)
  // Descomenta y configura abajo si prefieres configurar manualmente:
  
  // const PUBLIC_KEYS_MANUALES = [
  //   'GBXXX...',
  //   'GBYYY...',
  //   'GBZZZ...'
  // ];
  
  return publicKeysFromFile;
}

// Cargar configuración al inicio
let publicKeys = cargarConfiguracion();

// Si no se cargó desde archivo, usar configuración manual o valores por defecto
if (!publicKeys || publicKeys.length === 0) {
  publicKeys = [
    'GBXXX...', // Reemplaza con tus public keys reales
    'GBYYY...',
    'GBZZZ...'
  ];
}

/**
 * Función para consultar información de una cuenta
 */
async function consultarCuenta(publicKey) {
  try {
    const account = await server.loadAccount(publicKey);
    
    // Obtener balance de XLM
    const balanceXLM = account.balances.find(b => b.asset_type === 'native');
    const balance = balanceXLM ? balanceXLM.balance : '0';
    
    // Obtener número de trustlines
    // Los trustlines son balances que no son native (XLM)
    const trustlines = account.balances.filter(b => b.asset_type !== 'native').length;
    
    // Obtener sequence number
    const sequence = account.sequenceNumber();
    
    return {
      publicKey: publicKey,
      balance: balance,
      trustlines: trustlines,
      sequence: sequence,
      exito: true
    };
    
  } catch (error) {
    if (error.response && error.response.status === 404) {
      return {
        publicKey: publicKey,
        balance: 'N/A',
        trustlines: 'N/A',
        sequence: 'N/A',
        exito: false,
        error: 'Cuenta no encontrada'
      };
    } else {
      return {
        publicKey: publicKey,
        balance: 'N/A',
        trustlines: 'N/A',
        sequence: 'N/A',
        exito: false,
        error: error.message
      };
    }
  }
}

/**
 * Función principal: Monitor de balances
 */
async function monitorBalances() {
  console.log('╔═══════════════════════════════════════════╗');
  console.log('🔍 EJERCICIO 3: MONITOR DE BALANCES');
  console.log('╚═══════════════════════════════════════════╝');
  
  // Verificar que las public keys estén configuradas
  if (!publicKeys || publicKeys.length === 0 || publicKeys[0] === 'GBXXX...' || publicKeys[0].startsWith('GBXXX') || publicKeys[0] === undefined) {
    console.error('\n❌ ERROR: Debes configurar las public keys');
    console.log('\n💡 OPCIONES:');
    console.log('   1. Ejecuta primero el Ejercicio 1 para crear cuentas automáticamente');
    console.log(`   2. Edita el archivo y configura PUBLIC_KEYS_MANUALES (línea ~20)`);
    console.log('   3. Descomenta y configura PUBLIC_KEYS_MANUALES en la función cargarConfiguracion()\n');
    process.exit(1);
  }
  
  console.log(`\n📋 Consultando ${publicKeys.length} cuentas...\n`);
  
  const resultados = [];
  
  // Consultar cada cuenta
  for (let i = 0; i < publicKeys.length; i++) {
    const publicKey = publicKeys[i];
    
    console.log(`🔍 Consultando cuenta ${i + 1}/${publicKeys.length}: ${publicKey.substring(0, 8)}...`);
    
    const resultado = await consultarCuenta(publicKey);
    resultados.push(resultado);
    
    // Pequeña pausa entre consultas
    if (i < publicKeys.length - 1) {
      await new Promise(resolve => setTimeout(resolve, 500));
    }
  }
  
  // Mostrar resultados formateados
  console.log('\n╔═══════════════════════════════════════════╗');
  console.log('📊 RESULTADOS DEL MONITOR');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  resultados.forEach((resultado, index) => {
    console.log(`━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
    
    if (resultado.exito) {
      console.log(`✅ Cuenta ${index + 1}:`);
      console.log(`   📧 Public Key: ${resultado.publicKey}`);
      console.log(`   💵 Balance: ${resultado.balance} XLM`);
      console.log(`   🔗 Trustlines: ${resultado.trustlines}`);
      console.log(`   🔢 Sequence: ${resultado.sequence}`);
    } else {
      console.log(`❌ Cuenta ${index + 1}:`);
      console.log(`   📧 Public Key: ${resultado.publicKey}`);
      console.log(`   ⚠️  Error: ${resultado.error}`);
    }
    
    console.log('');
  });
  
  // Resumen estadístico
  const exitosos = resultados.filter(r => r.exito).length;
  const fallidos = resultados.filter(r => !r.exito).length;
  
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log(`📊 Total consultadas: ${resultados.length}`);
  console.log(`✅ Exitosas: ${exitosos}`);
  console.log(`❌ Fallidas: ${fallidos}`);
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  
  // Mostrar balances totales (solo de cuentas exitosas)
  const balancesExitosos = resultados
    .filter(r => r.exito && r.balance !== 'N/A')
    .map(r => parseFloat(r.balance))
    .reduce((sum, balance) => sum + balance, 0);
  
  if (exitosos > 0) {
    console.log(`💰 Balance total (XLM): ${balancesExitosos.toFixed(7)} XLM`);
    console.log(`🔗 Trustlines totales: ${resultados
      .filter(r => r.exito && r.trustlines !== 'N/A')
      .map(r => r.trustlines)
      .reduce((sum, tl) => sum + tl, 0)}`);
    console.log('');
  }
  
  return resultados;
}

// Ejecutar ejercicio
monitorBalances().catch(error => {
  console.error('❌ Error fatal:', error.message);
  process.exit(1);
});

