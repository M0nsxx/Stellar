#!/usr/bin/env node

/**
 * Ejercicio 2 de Terminal y CLI
 * Script de Batch Processing
 * 
 * Este script:
 * 1. Lee un archivo JSON con múltiples cuentas
 * 2. Envía pagos a todas
 * 3. Guarda resultados en un archivo de log
 */

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
import path from 'path';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// ⚠️ CONFIGURACIÓN: Reemplaza con tus valores
const SECRET_KEY = 'SBXXX...'; // Tu secret key
const ARCHIVO_DESTINATARIOS = 'destinatarios.json'; // Archivo con destinatarios
const ARCHIVO_LOG = 'batch-pagos.log'; // Archivo de log

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
  console.log(mensaje);
}

// Leer archivo de destinatarios
function leerDestinatarios() {
  try {
    if (!fs.existsSync(ARCHIVO_DESTINATARIOS)) {
      // Crear archivo de ejemplo si no existe
      const ejemplo = {
        destinatarios: [
          {
            publicKey: 'GBXXX...',
            cantidad: '10',
            memo: 'Pago batch #1'
          },
          {
            publicKey: 'GBYYY...',
            cantidad: '10',
            memo: 'Pago batch #2'
          }
        ]
      };
      
      fs.writeFileSync(ARCHIVO_DESTINATARIOS, JSON.stringify(ejemplo, null, 2));
      log(`⚠️  Archivo ${ARCHIVO_DESTINATARIOS} no existe. Se creó un archivo de ejemplo.`, 'yellow');
      log(`   Por favor edita el archivo y agrega tus destinatarios reales.`, 'yellow');
      return null;
    }
    
    const contenido = fs.readFileSync(ARCHIVO_DESTINATARIOS, 'utf8');
    const data = JSON.parse(contenido);
    
    if (!data.destinatarios || !Array.isArray(data.destinatarios)) {
      throw new Error('El archivo debe contener un array "destinatarios"');
    }
    
    return data.destinatarios;
  } catch (error) {
    log(`❌ Error leyendo ${ARCHIVO_DESTINATARIOS}:`, 'red');
    log(`   ${error.message}`, 'yellow');
    return null;
  }
}

// Enviar pago a un destinatario
async function enviarPago(destinatario, index, total) {
  try {
    const { publicKey, cantidad, memo } = destinatario;
    
    escribirLog(`📧 [${index + 1}/${total}] Enviando ${cantidad} XLM a ${publicKey.substring(0, 12)}...`);
    
    // Validar datos
    if (!publicKey || !cantidad) {
      throw new Error('Faltan datos: publicKey y cantidad son requeridos');
    }
    
    // Cargar cuenta origen
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    // Validar balance
    const balance = parseFloat(sourceAccount.balances[0].balance);
    const amount = parseFloat(cantidad);
    const baseReserve = 0.5;
    const subentryReserve = sourceAccount.subentry_count * 0.5;
    const totalReserve = baseReserve + subentryReserve;
    const disponible = balance - totalReserve;
    
    if (amount > disponible) {
      throw new Error(`Balance insuficiente. Disponible: ${disponible.toFixed(7)} XLM`);
    }
    
    // Construir transacción
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: publicKey,
        asset: Asset.native(),
        amount: cantidad.toString()
      }))
      .addMemo(memo ? Memo.text(memo) : Memo.text(`Pago batch #${index + 1}`))
      .setTimeout(30)
      .build();
    
    // Firmar y enviar
    transaction.sign(sourceKeys);
    const result = await server.submitTransaction(transaction);
    
    escribirLog(`✅ [${index + 1}/${total}] Pago exitoso | Hash: ${result.hash}`);
    
    return {
      exito: true,
      destinatario: publicKey,
      cantidad,
      hash: result.hash
    };
    
  } catch (error) {
    escribirLog(`❌ [${index + 1}/${total}] Error: ${error.message}`);
    
    return {
      exito: false,
      destinatario: destinatario.publicKey || 'Desconocido',
      cantidad: destinatario.cantidad || '0',
      error: error.message
    };
  }
}

// Función principal
async function procesarBatch() {
  log('╔═══════════════════════════════════════════╗', 'cyan');
  log('🚀 BATCH PROCESSING - PAGOS MÚLTIPLES', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  
  // Verificar configuración
  if (SECRET_KEY === 'SBXXX...') {
    log('\n❌ ERROR: Configura tu SECRET_KEY en el archivo', 'red');
    log('   Edita el archivo y reemplaza SECRET_KEY con tu secret key', 'yellow');
    process.exit(1);
  }
  
  // Inicializar log
  const timestamp = new Date().toISOString();
  fs.writeFileSync(ARCHIVO_LOG, `=== BATCH PROCESSING INICIADO ===\nFecha: ${timestamp}\n\n`);
  
  // Leer destinatarios
  log('\n📖 Leyendo archivo de destinatarios...', 'cyan');
  const destinatarios = leerDestinatarios();
  
  if (!destinatarios || destinatarios.length === 0) {
    log('❌ No hay destinatarios para procesar', 'red');
    process.exit(1);
  }
  
  log(`✅ ${destinatarios.length} destinatarios encontrados`, 'green');
  
  // Procesar cada destinatario
  log('\n🚀 Iniciando procesamiento batch...\n', 'cyan');
  
  const resultados = [];
  let exitosos = 0;
  let fallidos = 0;
  
  for (let i = 0; i < destinatarios.length; i++) {
    const resultado = await enviarPago(destinatarios[i], i, destinatarios.length);
    resultados.push(resultado);
    
    if (resultado.exito) {
      exitosos++;
    } else {
      fallidos++;
    }
    
    // Esperar entre pagos para evitar problemas de sequence number
    if (i < destinatarios.length - 1) {
      await new Promise(resolve => setTimeout(resolve, 2000));
    }
  }
  
  // Resumen final
  log('\n╔═══════════════════════════════════════════╗', 'cyan');
  log('📊 RESUMEN DEL PROCESAMIENTO BATCH', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  
  log(`\n✅ Exitosos: ${exitosos}`, 'green');
  log(`❌ Fallidos: ${fallidos}`, 'red');
  log(`📦 Total procesados: ${destinatarios.length}`, 'blue');
  
  const totalEnviado = resultados
    .filter(r => r.exito)
    .reduce((sum, r) => sum + parseFloat(r.cantidad), 0);
  
  log(`💰 Total enviado: ${totalEnviado.toFixed(7)} XLM`, 'green');
  
  // Guardar resumen en log
  escribirLog('\n=== RESUMEN FINAL ===');
  escribirLog(`Exitosos: ${exitosos}`);
  escribirLog(`Fallidos: ${fallidos}`);
  escribirLog(`Total enviado: ${totalEnviado.toFixed(7)} XLM`);
  
  log(`\n📝 Log guardado en: ${ARCHIVO_LOG}`, 'blue');
  log('\n✅ Procesamiento batch completado', 'green');
}

// Ejecutar
procesarBatch().catch(error => {
  log(`\n❌ Error fatal: ${error.message}`, 'red');
  escribirLog(`ERROR FATAL: ${error.message}`);
  process.exit(1);
});

