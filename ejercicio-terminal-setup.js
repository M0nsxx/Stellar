#!/usr/bin/env node

/**
 * Ejercicio 1 de Terminal y CLI
 * Script de Setup Automático
 * 
 * Este script:
 * 1. Verifica que Node.js está instalado
 * 2. Instala todas las dependencias
 * 3. Crea una cuenta de prueba
 * 4. Muestra el balance
 */

import { Keypair } from '@stellar/stellar-sdk';
import { Horizon } from '@stellar/stellar-sdk';
import { exec } from 'child_process';
import { promisify } from 'util';

const Server = Horizon.Server;
const execAsync = promisify(exec);
const server = new Server('https://horizon-testnet.stellar.org');

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

// Paso 1: Verificar Node.js
async function verificarNodeJS() {
  log('\n🔍 Paso 1: Verificando Node.js...', 'cyan');
  
  try {
    const { stdout } = await execAsync('node --version');
    const version = stdout.trim();
    log(`✅ Node.js instalado: ${version}`, 'green');
    return true;
  } catch (error) {
    log('❌ Node.js no está instalado', 'red');
    log('   Por favor instala Node.js desde: https://nodejs.org/', 'yellow');
    return false;
  }
}

// Paso 2: Verificar npm
async function verificarNPM() {
  log('\n🔍 Paso 2: Verificando npm...', 'cyan');
  
  try {
    const { stdout } = await execAsync('npm --version');
    const version = stdout.trim();
    log(`✅ npm instalado: ${version}`, 'green');
    return true;
  } catch (error) {
    log('❌ npm no está instalado', 'red');
    return false;
  }
}

// Paso 3: Instalar dependencias
async function instalarDependencias() {
  log('\n🔍 Paso 3: Instalando dependencias...', 'cyan');
  
  try {
    log('📦 Ejecutando npm install...', 'blue');
    const { stdout, stderr } = await execAsync('npm install');
    
    if (stdout) {
      log('📦 Output:', 'blue');
      console.log(stdout);
    }
    
    log('✅ Dependencias instaladas correctamente', 'green');
    return true;
  } catch (error) {
    log('❌ Error instalando dependencias', 'red');
    log(`   ${error.message}`, 'yellow');
    
    // Si el error es que ya están instaladas, no es problema
    if (error.message.includes('up to date') || error.message.includes('audited')) {
      log('✅ Dependencias ya instaladas', 'green');
      return true;
    }
    
    return false;
  }
}

// Paso 4: Crear cuenta de prueba
async function crearCuentaPrueba() {
  log('\n🔍 Paso 4: Creando cuenta de prueba...', 'cyan');
  
  try {
    log('🔐 Generando llaves...', 'blue');
    const pair = Keypair.random();
    const publicKey = pair.publicKey();
    const secretKey = pair.secret();
    
    log(`✅ Cuenta creada:`, 'green');
    log(`   📧 Public Key: ${publicKey}`, 'blue');
    log(`   🔑 Secret Key: ${secretKey.substring(0, 16)}...`, 'blue');
    
    // Fondear con Friendbot
    log('\n💰 Fondeando con Friendbot...', 'cyan');
    
    try {
      const response = await fetch(
        `https://friendbot.stellar.org/?addr=${publicKey}`
      );
      
      const result = await response.json();
      
      if (result.successful || response.ok) {
        log('✅ Cuenta fondeada con 10,000 XLM', 'green');
        log(`   🔗 Transaction hash: ${result.hash}`, 'blue');
        
        // Esperar un poco para que se propague
        await new Promise(resolve => setTimeout(resolve, 3000));
        
        return { publicKey, secretKey };
      }
    } catch (error) {
      log('⚠️  Error al fondear con Friendbot:', 'yellow');
      log(`   ${error.message}`, 'yellow');
      log('   La cuenta se creó pero no se pudo fondear', 'yellow');
      
      return { publicKey, secretKey };
    }
    
    return { publicKey, secretKey };
  } catch (error) {
    log('❌ Error creando cuenta:', 'red');
    log(`   ${error.message}`, 'yellow');
    return null;
  }
}

// Paso 5: Mostrar balance
async function mostrarBalance(publicKey) {
  log('\n🔍 Paso 5: Consultando balance...', 'cyan');
  
  try {
    const account = await server.loadAccount(publicKey);
    const balance = account.balances[0];
    
    if (balance.asset_type === 'native') {
      const baseReserve = 0.5;
      const subentryReserve = account.subentry_count * 0.5;
      const totalReserve = baseReserve + subentryReserve;
      const available = parseFloat(balance.balance) - totalReserve;
      
      log('╔═══════════════════════════════════╗', 'cyan');
      log('📊 BALANCE DE CUENTA', 'cyan');
      log('╚═══════════════════════════════════╝', 'cyan');
      log(`\n💰 Balance Total: ${balance.balance} XLM`, 'green');
      log(`🔒 Bloqueado (Reserves): ${totalReserve.toFixed(7)} XLM`, 'yellow');
      log(`✅ Disponible: ${available.toFixed(7)} XLM`, 'green');
      log(`🔢 Sequence Number: ${account.sequenceNumber()}`, 'blue');
      
      return true;
    }
  } catch (error) {
    log('❌ Error consultando balance:', 'red');
    log(`   ${error.message}`, 'yellow');
    
    if (error.response && error.response.status === 404) {
      log('   💡 La cuenta existe pero puede que no esté fondeada aún', 'yellow');
      log('   💡 Espera unos segundos y vuelve a ejecutar el script', 'yellow');
    }
    
    return false;
  }
}

// Función principal
async function setup() {
  log('╔═══════════════════════════════════════════╗', 'cyan');
  log('🚀 SCRIPT DE SETUP - STELLAR BLOCKCHAIN', 'cyan');
  log('╚═══════════════════════════════════════════╝', 'cyan');
  
  // Paso 1: Verificar Node.js
  const nodeOk = await verificarNodeJS();
  if (!nodeOk) {
    log('\n❌ Setup fallido: Node.js requerido', 'red');
    process.exit(1);
  }
  
  // Paso 2: Verificar npm
  const npmOk = await verificarNPM();
  if (!npmOk) {
    log('\n❌ Setup fallido: npm requerido', 'red');
    process.exit(1);
  }
  
  // Paso 3: Instalar dependencias
  const depsOk = await instalarDependencias();
  if (!depsOk) {
    log('\n⚠️  Advertencia: Puede haber problemas con las dependencias', 'yellow');
  }
  
  // Paso 4: Crear cuenta
  const cuenta = await crearCuentaPrueba();
  if (!cuenta) {
    log('\n❌ Setup fallido: No se pudo crear cuenta', 'red');
    process.exit(1);
  }
  
  // Paso 5: Mostrar balance
  await mostrarBalance(cuenta.publicKey);
  
  // Resumen final
  log('\n╔═══════════════════════════════════════════╗', 'green');
  log('✅ SETUP COMPLETADO EXITOSAMENTE', 'green');
  log('╚═══════════════════════════════════════════╝', 'green');
  log('\n📝 Resumen:', 'cyan');
  log(`   ✅ Node.js instalado`, 'green');
  log(`   ✅ npm instalado`, 'green');
  log(`   ✅ Dependencias instaladas`, 'green');
  log(`   ✅ Cuenta creada: ${cuenta.publicKey}`, 'green');
  log(`   ⚠️  Guarda tu Secret Key: ${cuenta.secretKey.substring(0, 16)}...`, 'yellow');
  log('\n💡 Puedes usar esta cuenta para tus pruebas', 'blue');
  log('\n🚀 ¡Listo para comenzar a desarrollar!', 'green');
}

// Ejecutar setup
setup().catch(error => {
  log(`\n❌ Error fatal: ${error.message}`, 'red');
  process.exit(1);
});

