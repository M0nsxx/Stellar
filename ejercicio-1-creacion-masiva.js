import { Keypair } from '@stellar/stellar-sdk';
import { Horizon } from '@stellar/stellar-sdk';
import fs from 'fs';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');

// Archivo donde se guardarán las cuentas creadas
const ARCHIVO_CUENTAS = 'cuentas-tarea-clase2.json';

/**
 * 🚀 EJERCICIO 1: CREACIÓN MASIVA DE CUENTAS
 * 
 * Objetivo: Crear 5 cuentas automáticamente
 * - Usar bucle for para generar 5 keypairs
 * - Cada cuenta debe ser fondeada con Friendbot
 * - Mostrar en consola: public key, secret key y balance inicial de cada una
 * - Guardar toda la información en un array
 */

async function crearCuentaConFondeo(numero) {
  console.log(`\n🔐 Creando cuenta ${numero}...\n`);
  
  // Generar llaves aleatorias
  const pair = Keypair.random();
  const publicKey = pair.publicKey();
  const secretKey = pair.secret();
  
  console.log(`✅ ¡Cuenta ${numero} creada!\n`);
  console.log('📧 PUBLIC KEY (puedes compartir):');
  console.log(`   ${publicKey}`);
  console.log('\n🔑 SECRET KEY (NUNCA COMPARTIR):');
  console.log(`   ${secretKey}`);
  
  // Fondear con Friendbot
  console.log('\n💰 Fondeando con Friendbot...');
  
  try {
    const response = await fetch(
      `https://friendbot.stellar.org/?addr=${publicKey}`
    );
    
    const result = await response.json();
    
    if (result.successful || response.ok) {
      console.log('✅ ¡Cuenta fondeada con 10,000 XLM!\n');
      console.log('🔗 Transaction hash:', result.hash);
      
      // Esperar un poco para que se propague la transacción
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Consultar balance inicial
      try {
        const account = await server.loadAccount(publicKey);
        const balance = account.balances[0].balance;
        
        console.log(`\n💵 Balance inicial: ${balance} XLM\n`);
        
        return {
          numero: numero,
          publicKey: publicKey,
          secretKey: secretKey,
          balance: balance,
          hash: result.hash,
          creado: new Date().toISOString()
        };
      } catch (error) {
        console.log(`⚠️  Balance inicial: 10000 XLM (no se pudo verificar en este momento)\n`);
        
        return {
          numero: numero,
          publicKey: publicKey,
          secretKey: secretKey,
          balance: '10000',
          hash: result.hash,
          creado: new Date().toISOString()
        };
      }
    } else {
      throw new Error('Friendbot no respondió exitosamente');
    }
  } catch (error) {
    console.error('❌ Error al fondear:', error.message);
    console.log('⚠️  La cuenta se creó pero no se pudo fondear\n');
    
    return {
      numero: numero,
      publicKey: publicKey,
      secretKey: secretKey,
      balance: '0',
      hash: null,
      error: error.message,
      creado: new Date().toISOString()
    };
  }
}

async function crearMultiplesCuentas() {
  console.log('╔═══════════════════════════════════════════╗');
  console.log('🚀 EJERCICIO 1: CREACIÓN MASIVA DE CUENTAS');
  console.log('╚═══════════════════════════════════════════╝');
  console.log('\n📋 Creando 5 cuentas automáticamente...\n');
  
  const cuentas = [];
  
  // Bucle for para crear 5 cuentas
  for (let i = 1; i <= 5; i++) {
    console.log(`\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
    console.log(`   CUENTA ${i} DE 5`);
    console.log(`━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
    
    const cuenta = await crearCuentaConFondeo(i);
    cuentas.push(cuenta);
    
    // Pequeña pausa entre cuentas para evitar rate limiting
    if (i < 5) {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }
  
  // Resumen final
  console.log('\n╔═══════════════════════════════════════════╗');
  console.log('📊 RESUMEN FINAL');
  console.log('╚═══════════════════════════════════════════╝\n');
  
  cuentas.forEach(cuenta => {
    console.log(`Cuenta ${cuenta.numero}:`);
    console.log(`  📧 Public Key: ${cuenta.publicKey}`);
    console.log(`  💵 Balance: ${cuenta.balance} XLM`);
    console.log(`  🔗 Hash: ${cuenta.hash || 'N/A'}`);
    console.log('');
  });
  
  console.log(`✅ Total de cuentas creadas: ${cuentas.length}`);
  console.log(`💾 Información guardada en array con ${cuentas.length} elementos\n`);
  
  // Guardar todas las cuentas en archivo JSON
  try {
    const datos = {
      creado: new Date().toISOString(),
      total: cuentas.length,
      cuentas: cuentas
    };
    
    fs.writeFileSync(ARCHIVO_CUENTAS, JSON.stringify(datos, null, 2));
    console.log(`💾 Cuentas guardadas en: ${ARCHIVO_CUENTAS}`);
    console.log(`   Este archivo será usado automáticamente por los ejercicios 2 y 3\n`);
  } catch (error) {
    console.log(`⚠️  No se pudo guardar en archivo: ${error.message}\n`);
  }
  
  console.log('⚠️  IMPORTANTE: Guarda estas llaves en un lugar seguro\n');
  console.log('💡 TIP: Puedes usar las cuentas creadas en:');
  console.log('   - Ejercicio 2: Usa la SECRET_KEY de la primera cuenta para enviar pagos');
  console.log('   - Ejercicio 2: Usa las PUBLIC_KEY de otras cuentas como destinatarios');
  console.log('   - Ejercicio 3: Usa las PUBLIC_KEY de todas las cuentas para monitorear\n');
  
  return cuentas;
}

// Ejecutar ejercicio
crearMultiplesCuentas().catch(error => {
  console.error('❌ Error fatal:', error.message);
  process.exit(1);
});

