import { Keypair } from '@stellar/stellar-sdk';
import fs from 'fs';

// Función para validar formato de public key
function esPublicKeyValida(key) {
  return key.startsWith('G') && key.length === 56;
}

// Función para validar formato de secret key
function esSecretKeyValida(key) {
  return key.startsWith('S') && key.length === 56;
}

// Función para crear una cuenta y guardarla en archivo
async function crearYGuardarCuenta(filename = 'mi-cuenta.json') {
  console.log('🔐 Generando tu nuevo par de llaves...\n');
  
  const pair = Keypair.random();
  
  const data = {
    publicKey: pair.publicKey(),
    secretKey: pair.secret(),
    creado: new Date().toISOString()
  };
  
  // Validar antes de guardar
  if (!esPublicKeyValida(data.publicKey)) {
    throw new Error('Public key inválida');
  }
  
  if (!esSecretKeyValida(data.secretKey)) {
    throw new Error('Secret key inválida');
  }
  
  fs.writeFileSync(filename, JSON.stringify(data, null, 2));
  
  console.log('✅ ¡Cuenta creada y guardada!\n');
  console.log('📧 PUBLIC KEY (puedes compartir):');
  console.log(data.publicKey);
  console.log('\n🔑 SECRET KEY (NUNCA COMPARTIR):');
  console.log(data.secretKey);
  console.log(`\n💾 Guardado en: ${filename}\n`);
  
  // Fondear con Friendbot
  console.log('💰 Fondeando con Friendbot...');
  
  try {
    const response = await fetch(
      `https://friendbot.stellar.org/?addr=${pair.publicKey()}`
    );
    
    const result = await response.json();
    
    if (result.successful || response.ok) {
      console.log('✅ ¡Cuenta fondeada con 10,000 XLM!\n');
      console.log('🔗 Transaction hash:', result.hash);
    }
  } catch (error) {
    console.error('❌ Error al fondear:', error.message);
  }
  
  console.log('\n⚠️  IMPORTANTE: Guarda estas llaves en un lugar seguro\n');
  
  return data;
}

// Función para crear múltiples cuentas
async function crearMultiplesCuentas(cantidad = 5) {
  console.log(`🔐 Generando ${cantidad} cuentas...\n`);
  
  const cuentas = [];
  
  for (let i = 0; i < cantidad; i++) {
    const pair = Keypair.random();
    const publicKey = pair.publicKey();
    
    // Validar formato
    if (esPublicKeyValida(publicKey)) {
      cuentas.push({
        numero: i + 1,
        publicKey: publicKey,
        secretKey: pair.secret()
      });
      
      console.log(`✅ Cuenta ${i + 1}: ${publicKey}`);
    } else {
      console.log(`❌ Cuenta ${i + 1}: Error al generar`);
    }
  }
  
  // Guardar todas en archivo
  const filename = `cuentas-multiples-${Date.now()}.json`;
  fs.writeFileSync(filename, JSON.stringify(cuentas, null, 2));
  
  console.log(`\n💾 ${cantidad} cuentas guardadas en: ${filename}\n`);
  console.log('⚠️  IMPORTANTE: Guarda este archivo en un lugar seguro\n');
  
  return cuentas;
}

// Función principal mejorada (versión original con validación)
async function crearCuenta() {
  console.log('🔐 Generando tu nuevo par de llaves...\n');
  
  const pair = Keypair.random();
  
  const publicKey = pair.publicKey();
  const secretKey = pair.secret();
  
  // Validar formato
  if (!esPublicKeyValida(publicKey)) {
    throw new Error('Public key generada no es válida');
  }
  
  if (!esSecretKeyValida(secretKey)) {
    throw new Error('Secret key generada no es válida');
  }
  
  console.log('✅ ¡Cuenta creada!\n');
  console.log('📧 PUBLIC KEY (puedes compartir):');
  console.log(publicKey);
  console.log('\n🔑 SECRET KEY (NUNCA COMPARTIR):');
  console.log(secretKey);
  
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
    }
  } catch (error) {
    console.error('❌ Error al fondear:', error.message);
  }
  
  console.log('\n⚠️  IMPORTANTE: Guarda estas llaves en un lugar seguro\n');
  
  return { publicKey, secretKey };
}

// Ejecutar función según argumentos
const args = process.argv.slice(2);

if (args.includes('--multiples')) {
  const cantidad = parseInt(args[args.indexOf('--multiples') + 1]) || 5;
  crearMultiplesCuentas(cantidad);
} else if (args.includes('--guardar')) {
  const filename = args[args.indexOf('--guardar') + 1] || 'mi-cuenta.json';
  crearYGuardarCuenta(filename);
} else {
  crearCuenta();
}

