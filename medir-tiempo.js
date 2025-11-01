#!/usr/bin/env node

/**
 * ⏱️ Medir Tiempo de Ejecución
 * Ejercicio 3: Medir cuánto tarda una invocación
 */

import { Horizon, Keypair, Networks, Contract } from '@stellar/stellar-sdk';
import { nativeToScVal } from '@stellar/stellar-sdk/contract';
import fs from 'fs';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// ⚠️ CONFIGURACIÓN
const SECRET_KEY = 'SBXXX...';
const NOMBRE = process.argv[2] || 'Test';

// Leer Contract ID
function leerContractID() {
  try {
    if (fs.existsSync('.contract-id')) {
      return fs.readFileSync('.contract-id', 'utf8').trim();
    }
  } catch (error) {
    return null;
  }
  return null;
}

const CONTRACT_ID = leerContractID() || 'CBQHNQXVZHKFGPZKDV5YXGPFVQTE6EXNIXKYFKBMJBQTBUKQRX7FE2OV';

const colors = {
  reset: '\x1b[0m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

async function medirTiempo() {
  log('\n╔═══════════════════════════════════════════╗', 'cyan');
  log('⏱️  MEDIR TIEMPO DE EJECUCIÓN', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  
  if (SECRET_KEY === 'SBXXX...') {
    log('\n❌ ERROR: Configura tu SECRET_KEY', 'red');
    process.exit(1);
  }
  
  log(`\n📞 Invocando función 'hello' con parámetro: ${NOMBRE}`, 'blue');
  log('⏱️  Midiendo tiempo de ejecución...', 'yellow');
  
  // Cargar cuenta
  const sourceKeys = Keypair.fromSecret(SECRET_KEY);
  const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
  const contract = new Contract(CONTRACT_ID);
  
  // Medir tiempo
  const startTime = Date.now();
  
  try {
    await contract.invoke({
      method: 'hello',
      args: [nativeToScVal(NOMBRE, { type: 'string' })],
      networkPassphrase: networkPassphrase,
      source: sourceAccount,
      signers: [sourceKeys]
    });
    
    const endTime = Date.now();
    const elapsed = ((endTime - startTime) / 1000).toFixed(3);
    
    log('\n╔═══════════════════════════════════════════╗', 'cyan');
    log('⏱️  RESULTADOS', 'cyan');
    log('╚═══════════════════════════════════════════╝', 'cyan');
    log(`\n⏱️  Tiempo de ejecución: ${elapsed} segundos`, 'blue');
    log('\n💡 Este tiempo incluye:', 'yellow');
    log('   • Construcción de transacción');
    log('   • Firma criptográfica');
    log('   • Envío a la red');
    log('   • Propagación por blockchain');
    log('   • Ejecución del contrato');
    log('   • Confirmación en ledger');
    
  } catch (error) {
    log(`\n❌ Error: ${error.message}`, 'red');
    throw error;
  }
}

medirTiempo().catch(error => {
  console.error(error);
  process.exit(1);
});

