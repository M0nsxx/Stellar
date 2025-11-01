#!/usr/bin/env node

/**
 * Ejercicio 3 de Terminal y CLI
 * Script de Monitoreo Avanzado
 * 
 * Este script:
 * 1. Monitorea el balance de una cuenta
 * 2. Guarda logs con timestamps
 * 3. Envía alerta si el balance baja de cierto nivel
 */

import { Horizon } from '@stellar/stellar-sdk';
import fs from 'fs';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');

// ⚠️ CONFIGURACIÓN: Reemplaza con tus valores
const PUBLIC_KEY = 'GBXXX...'; // Cuenta a monitorear
const INTERVALO_MS = 10000; // Intervalo en milisegundos (10 segundos)
const BALANCE_MINIMO = 1000; // Alerta si balance baja de este nivel (XLM)
const ARCHIVO_LOG = 'monitoreo.log'; // Archivo de log

// Colores para la consola
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

// Función para escribir en log
function escribirLog(mensaje) {
  const timestamp = new Date().toISOString();
  const logEntry = `[${timestamp}] ${mensaje}\n`;
  fs.appendFileSync(ARCHIVO_LOG, logEntry);
}

// Consultar balance
async function consultarBalance(publicKey) {
  try {
    const account = await server.loadAccount(publicKey);
    const balance = account.balances[0];
    
    if (balance.asset_type === 'native') {
      const baseReserve = 0.5;
      const subentryReserve = account.subentry_count * 0.5;
      const totalReserve = baseReserve + subentryReserve;
      const available = parseFloat(balance.balance) - totalReserve;
      
      return {
        total: parseFloat(balance.balance),
        bloqueado: totalReserve,
        disponible: available,
        sequenceNumber: account.sequenceNumber(),
        cuenta: publicKey
      };
    }
    
    return null;
  } catch (error) {
    log(`❌ Error consultando balance: ${error.message}`, 'red');
    escribirLog(`ERROR: ${error.message}`);
    return null;
  }
}

// Enviar alerta
function enviarAlerta(balanceInfo) {
  const mensaje = `⚠️  ALERTA: Balance bajo de ${BALANCE_MINIMO} XLM!\n` +
    `   Balance actual: ${balanceInfo.total.toFixed(7)} XLM\n` +
    `   Disponible: ${balanceInfo.disponible.toFixed(7)} XLM`;
  
  log(`\n${mensaje}`, 'yellow');
  escribirLog(`ALERTA: ${mensaje}`);
}

// Función principal de monitoreo
async function monitorear() {
  // Verificar configuración
  if (PUBLIC_KEY === 'GBXXX...') {
    log('\n❌ ERROR: Configura tu PUBLIC_KEY en el archivo', 'red');
    log('   Edita el archivo y reemplaza PUBLIC_KEY con la cuenta a monitorear', 'yellow');
    process.exit(1);
  }
  
  log('╔═══════════════════════════════════════════╗', 'cyan');
  log('🔍 MONITOREO DE BALANCE - STELLAR', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  
  log(`\n📧 Cuenta: ${PUBLIC_KEY.substring(0, 16)}...`, 'blue');
  log(`⏱️  Intervalo: ${INTERVALO_MS / 1000} segundos`, 'blue');
  log(`⚠️  Alerta si balance < ${BALANCE_MINIMO} XLM`, 'yellow');
  log(`📝 Log guardado en: ${ARCHIVO_LOG}`, 'blue');
  log('💡 Presiona Ctrl+C para detener\n', 'yellow');
  
  // Inicializar log
  const timestamp = new Date().toISOString();
  fs.writeFileSync(ARCHIVO_LOG, `=== MONITOREO INICIADO ===\nFecha: ${timestamp}\nCuenta: ${PUBLIC_KEY}\nIntervalo: ${INTERVALO_MS}ms\nBalance mínimo alerta: ${BALANCE_MINIMO} XLM\n\n`);
  
  let consultaNum = 0;
  let balanceAnterior = null;
  let alertasEnviadas = 0;
  
  const intervaloId = setInterval(async () => {
    consultaNum++;
    const timestamp = new Date().toLocaleTimeString();
    
    const balanceInfo = await consultarBalance(PUBLIC_KEY);
    
    if (balanceInfo) {
      log(`\n[${timestamp}] Consulta #${consultaNum}`, 'cyan');
      log(`   💰 Balance Total: ${balanceInfo.total.toFixed(7)} XLM`, 'green');
      log(`   🔒 Bloqueado: ${balanceInfo.bloqueado.toFixed(7)} XLM`, 'yellow');
      log(`   ✅ Disponible: ${balanceInfo.disponible.toFixed(7)} XLM`, 'green');
      log(`   🔢 Sequence: ${balanceInfo.sequenceNumber}`, 'blue');
      
      // Detectar cambios
      if (balanceAnterior !== null) {
        const diferencia = balanceInfo.total - balanceAnterior;
        if (diferencia > 0) {
          log(`   📈 Cambio: +${diferencia.toFixed(7)} XLM ⬆️`, 'green');
          escribirLog(`Cambio: +${diferencia.toFixed(7)} XLM`);
        } else if (diferencia < 0) {
          log(`   📉 Cambio: ${diferencia.toFixed(7)} XLM ⬇️`, 'red');
          escribirLog(`Cambio: ${diferencia.toFixed(7)} XLM`);
        } else {
          log(`   ➡️  Sin cambios`, 'blue');
        }
      }
      
      // Verificar alerta de balance mínimo
      if (balanceInfo.disponible < BALANCE_MINIMO && alertasEnviadas < 5) {
        enviarAlerta(balanceInfo);
        alertasEnviadas++;
      }
      
      balanceAnterior = balanceInfo.total;
      
      // Guardar en log
      escribirLog(`Consulta #${consultaNum} - Balance: ${balanceInfo.total.toFixed(7)} XLM - Disponible: ${balanceInfo.disponible.toFixed(7)} XLM`);
      
    } else {
      log(`[${timestamp}] Consulta #${consultaNum}`, 'cyan');
      log(`   ❌ No se pudo obtener el balance`, 'red');
    }
    
    log('─'.repeat(50), 'blue');
  }, INTERVALO_MS);
  
  // Manejar Ctrl+C
  process.on('SIGINT', () => {
    log('\n\n🛑 Deteniendo monitoreo...', 'yellow');
    clearInterval(intervaloId);
    
    escribirLog(`=== MONITOREO DETENIDO ===\nTotal consultas: ${consultaNum}\nFecha: ${new Date().toISOString()}`);
    
    log(`✅ Monitoreo detenido`, 'green');
    log(`📝 Log guardado en: ${ARCHIVO_LOG}`, 'blue');
    log(`📊 Total consultas realizadas: ${consultaNum}`, 'cyan');
    
    process.exit(0);
  });
}

// Ejecutar
monitorear().catch(error => {
  log(`\n❌ Error fatal: ${error.message}`, 'red');
  escribirLog(`ERROR FATAL: ${error.message}`);
  process.exit(1);
});

