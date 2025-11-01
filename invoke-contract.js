#!/usr/bin/env node

/**
 * 📞 Invocar Smart Contract desde JavaScript
 * Clase 2 - Integración de Smart Contracts con JavaScript
 */

import { Horizon, Keypair, Networks, Contract } from '@stellar/stellar-sdk';
import { scValToNative, nativeToScVal } from '@stellar/stellar-sdk/contract';
import fs from 'fs';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// ⚠️ CONFIGURACIÓN: Reemplaza con tus valores
const CONTRACT_ID = 'CBQHNQXVZHKFGPZKDV5YXGPFVQTE6EXNIXKYFKBMJBQTBUKQRX7FE2OV'; // Tu Contract ID
const SECRET_KEY = 'SBXXX...'; // Tu secret key

// Colores para consola
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

// Función para leer Contract ID de archivo
function leerContractID() {
  try {
    if (fs.existsSync('.contract-id')) {
      const contractId = fs.readFileSync('.contract-id', 'utf8').trim();
      return contractId;
    }
  } catch (error) {
    // Ignorar error
  }
  return null;
}

// Función principal para invocar contrato
async function invocarContrato(nombre = 'Tiburona') {
  try {
    log('\n╔═══════════════════════════════════════════╗', 'cyan');
    log('📞 INVOCAR SMART CONTRACT DESDE JAVASCRIPT', 'cyan');
    log('╚═══════════════════════════════════════════╝', 'cyan');
    
    // Obtener Contract ID
    let contractId = CONTRACT_ID;
    const fileContractId = leerContractID();
    
    if (fileContractId) {
      contractId = fileContractId;
      log(`✅ Contract ID leído de archivo: ${contractId.substring(0, 16)}...`, 'green');
    } else if (CONTRACT_ID === 'CBQHNQXVZHKFGPZKDV5YXGPFVQTE6EXNIXKYFKBMJBQTBUKQRX7FE2OV') {
      log('⚠️  Usando Contract ID de ejemplo', 'yellow');
      log('   Edita el archivo y reemplaza CONTRACT_ID con tu Contract ID real', 'yellow');
    }
    
    // Verificar Secret Key
    if (SECRET_KEY === 'SBXXX...') {
      log('\n❌ ERROR: Configura tu SECRET_KEY en el archivo', 'red');
      log('   Edita el archivo y reemplaza SECRET_KEY con tu secret key', 'yellow');
      process.exit(1);
    }
    
    log(`\n📧 Nombre a saludar: ${nombre}`, 'blue');
    log('🚀 Invocando contrato...', 'cyan');
    
    // Cargar cuenta
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    log(`✅ Cuenta cargada: ${sourceKeys.publicKey().substring(0, 16)}...`, 'green');
    
    // Crear instancia del contrato
    const contract = new Contract(contractId);
    
    // Invocar función hello
    const result = await contract.invoke({
      method: 'hello',
      args: [
        nativeToScVal(nombre, { type: 'string' })
      ],
      networkPassphrase: networkPassphrase,
      source: sourceAccount,
      signers: [sourceKeys]
    });
    
    log('\n╔═══════════════════════════════════════════╗', 'green');
    log('✅ INVOCACIÓN EXITOSA', 'green');
    log('╚═══════════════════════════════════════════╝', 'green');
    
    log('\n📊 Resultado:', 'cyan');
    
    // Convertir resultado
    if (result) {
      try {
        const resultadoNativo = scValToNative(result);
        log(`   ${JSON.stringify(resultadoNativo, null, 2)}`, 'blue');
      } catch (e) {
        log(`   ${result}`, 'blue');
      }
    }
    
    log('\n✅ Función ejecutada correctamente', 'green');
    log(`\n🔗 Ver en StellarExpert:`, 'yellow');
    log(`   https://stellar.expert/explorer/testnet/contract/${contractId}`, 'blue');
    
    return result;
    
  } catch (error) {
    log('\n❌ Error invocando contrato:', 'red');
    log(`   ${error.message}`, 'yellow');
    
    if (error.message.includes('ContractNotFound')) {
      log('\n💡 Posibles soluciones:', 'yellow');
      log('   1. Verifica que el Contract ID sea correcto', 'yellow');
      log('   2. Asegúrate de haber deployado el contrato primero', 'yellow');
      log('   3. Verifica que estás usando Testnet', 'yellow');
    }
    
    throw error;
  }
}

// Ejecutar según argumentos
const args = process.argv.slice(2);
const nombre = args[0] || 'Tiburona';

invocarContrato(nombre).catch(error => {
  console.error(error);
  process.exit(1);
});

