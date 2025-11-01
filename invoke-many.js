#!/usr/bin/env node

/**
 * 🚀 Invocación Masiva desde JavaScript
 * Ejercicio 2: Invocar el contrato múltiples veces
 */

import { Horizon, Keypair, Networks, Contract } from '@stellar/stellar-sdk';
import { scValToNative, nativeToScVal } from '@stellar/stellar-sdk/contract';
import fs from 'fs';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// ⚠️ CONFIGURACIÓN
const SECRET_KEY = 'SBXXX...';

// Leer Contract ID de archivo
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
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

// Array de nombres
const nombres = ['Ana', 'Luis', 'Sofia', 'Carlos', 'Elena', 'Maria', 'Juan', 'Pedro'];

async function invocarMasivo() {
  log('\n╔═══════════════════════════════════════════╗', 'cyan');
  log('🚀 INVOCACIÓN MASIVA - SMART CONTRACT', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  
  if (SECRET_KEY === 'SBXXX...') {
    log('\n❌ ERROR: Configura tu SECRET_KEY', 'red');
    process.exit(1);
  }
  
  log(`\n📋 Invocando para ${nombres.length} nombres diferentes...`, 'blue');
  log(`🔑 Contract ID: ${CONTRACT_ID.substring(0, 16)}...\n`, 'blue');
  
  // Cargar cuenta
  const sourceKeys = Keypair.fromSecret(SECRET_KEY);
  const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
  const contract = new Contract(CONTRACT_ID);
  
  let exitosos = 0;
  let fallidos = 0;
  const resultados = [];
  
  for (let i = 0; i < nombres.length; i++) {
    const nombre = nombres[i];
    log(`[${i + 1}/${nombres.length}] 🚀 Invocando para ${nombre}...`, 'cyan');
    
    try {
      // Recargar cuenta para actualizar sequence number
      const account = await server.loadAccount(sourceKeys.publicKey());
      
      const result = await contract.invoke({
        method: 'hello',
        args: [nativeToScVal(nombre, { type: 'string' })],
        networkPassphrase: networkPassphrase,
        source: account,
        signers: [sourceKeys]
      });
      
      log(`✅ [${i + 1}/${nombres.length}] Exitoso para ${nombre}`, 'green');
      exitosos++;
      resultados.push({ nombre, exito: true, resultado: result });
      
    } catch (error) {
      log(`❌ [${i + 1}/${nombres.length}] Fallido para ${nombre}: ${error.message}`, 'red');
      fallidos++;
      resultados.push({ nombre, exito: false, error: error.message });
    }
    
    // Esperar entre invocaciones
    if (i < nombres.length - 1) {
      await new Promise(resolve => setTimeout(resolve, 2000));
    }
    
    console.log('');
  }
  
  // Resumen
  log('\n╔═══════════════════════════════════════════╗', 'cyan');
  log('📊 RESUMEN', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  log(`\n✅ Exitosos: ${exitosos}`, 'green');
  log(`❌ Fallidos: ${fallidos}`, 'red');
  log(`📦 Total: ${nombres.length}`, 'blue');
  log(`\n🔗 Ver en StellarExpert:`, 'yellow');
  log(`   https://stellar.expert/explorer/testnet/contract/${CONTRACT_ID}`, 'blue');
}

invocarMasivo().catch(error => {
  log(`\n❌ Error fatal: ${error.message}`, 'red');
  process.exit(1);
});

