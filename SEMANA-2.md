# 📚 SEMANA 2 - Ejercicios Stellar

Este documento contiene todos los ejercicios de la Semana 2 del curso de Stellar.

---

## 📋 Índice de Ejercicios

1. [Ejercicio 1: Creación Masiva de Cuentas](#ejercicio-1-creación-masiva-de-cuentas)
2. [Ejercicio 2: Sistema de Pagos Automatizado](#ejercicio-2-sistema-de-pagos-automatizado)
3. [Ejercicio 3: Monitor de Cuentas](#ejercicio-3-monitor-de-cuentas)
4. [Ejercicio 4: Configuración de Proyecto](#ejercicio-4-configuración-de-proyecto)

---

## 🔐 Ejercicio 1: Creación Masiva de Cuentas

### 📝 Descripción

Crear múltiples cuentas de Stellar Testnet automáticamente y fondearlas con Friendbot. Cada cuenta será fondeada con 10,000 XLM de testnet.

### 🎯 Objetivos

- Generar múltiples keypairs de forma automática
- Fondear cada cuenta con Friendbot
- Mostrar información de cada cuenta creada
- Generar un resumen final con todas las cuentas

### 📄 Archivo

`crear-multiples-cuentas.js`

### 💻 Código

```javascript
import { Keypair } from '@stellar/stellar-sdk';

async function crearMultiplasCuentas(cantidad) {
  const cuentas = [];

  for (let i = 1; i <= cantidad; i++) {
    console.log(`🔐 Creando cuenta ${i}...\n`);

    const pair = Keypair.random();

    try {
      const response = await fetch(
        `https://friendbot.stellar.org/?addr=${pair.publicKey()}`
      );

      const result = await response.json();

      cuentas.push({
        numero: i,
        publicKey: pair.publicKey(),
        secretKey: pair.secret(),
        balance: '10,000 XLM',
        hash: result.hash
      });

      console.log(`✅ Cuenta ${i} fondeada`);
      console.log(`   Public Key: ${pair.publicKey()}`);
      console.log(`   Balance: 10,000 XLM\n`);

    } catch (error) {
      console.error(`❌ Error en cuenta ${i}:`, error.message);
    }
  }

  return cuentas;
}

const misCuentas = await crearMultiplasCuentas(5);
console.log('\n📊 RESUMEN FINAL:');
console.log(JSON.stringify(misCuentas, null, 2));
```

### 🚀 Ejecución

```bash
npm run crear-multiples
```

O directamente:

```bash
node crear-multiples-cuentas.js
```

### ✅ Resultados

El script crea 5 cuentas automáticamente, cada una fondeada con 10,000 XLM de testnet. Al finalizar, muestra un resumen con todas las cuentas creadas incluyendo:

- Número de cuenta
- Public Key (para recibir pagos)
- Secret Key (para enviar pagos - ⚠️ NUNCA COMPARTIR)
- Balance inicial
- Hash de la transacción de fondeo

### 📊 Ejemplo de Salida (Resultados Reales)

```
🔐 Creando cuenta 1...

✅ Cuenta 1 fondeada
   Public Key: GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF
   Balance: 10,000 XLM

🔐 Creando cuenta 2...

✅ Cuenta 2 fondeada
   Public Key: GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI
   Balance: 10,000 XLM

🔐 Creando cuenta 3...

✅ Cuenta 3 fondeada
   Public Key: GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ
   Balance: 10,000 XLM

🔐 Creando cuenta 4...

✅ Cuenta 4 fondeada
   Public Key: GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V
   Balance: 10,000 XLM

🔐 Creando cuenta 5...

✅ Cuenta 5 fondeada
   Public Key: GB7CRYRZLAIEJZ6LZDGIT26QYU7Z3VB5KYZQ4COOC37YBKBXKPZMKH6M
   Balance: 10,000 XLM

📊 RESUMEN FINAL:
[
  {
    "numero": 1,
    "publicKey": "GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF",
    "secretKey": "SCHKVPBYYFG4KBG2JIKVXSNTPIQZYH4CQTSZIJ3L6KVEKG4SL22W7HLS",
    "balance": "10,000 XLM",
    "hash": "97f5044c6ac10822bb1b3a838b71ebd59ecc0ef1ffb4e6a69335efb53e2726ac"
  },
  {
    "numero": 2,
    "publicKey": "GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI",
    "secretKey": "SBESMJVZWBAMWCOADLRI4K34G7A65CTBED4GEEYRKEJSUALC2QWSZWLF",
    "balance": "10,000 XLM",
    "hash": "0c5cc89b585d31416bb07b669bf4a4e6526d22571b264e0d3fe1b074450c3cdb"
  },
  {
    "numero": 3,
    "publicKey": "GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ",
    "secretKey": "SDNQW2NLEACVZT5JCF62BZ37P36W5MPTU4DFE7U7BB7DG4JAAR4VXGIN",
    "balance": "10,000 XLM",
    "hash": "1a2b75b3acad6236eda46b4e4fc38516de1ba53f475ef6b35849fd625f431e02"
  },
  {
    "numero": 4,
    "publicKey": "GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V",
    "secretKey": "SCDKHT2HOY72RLGSIZZR52TSZ4EAFNPQUUPNJ3GKYMRQHMAZ5QIZXCE6",
    "balance": "10,000 XLM",
    "hash": "223d6f70a073eb2dee9ae7579088d67f78052852bd46007cbc343d239092ea35"
  },
  {
    "numero": 5,
    "publicKey": "GB7CRYRZLAIEJZ6LZDGIT26QYU7Z3VB5KYZQ4COOC37YBKBXKPZMKH6M",
    "secretKey": "SCJ35A3IE7MZWLSVP3KSBVAX6LKLSUN2SF4HWY3UQDWGVWXENZQDR7Y6",
    "balance": "10,000 XLM",
    "hash": "692cfbe12f267f56fc8112640c2321fc4f332b0e92a2c0f9a226ca2dcbbd8ca4"
  }
]
```

### 📝 Notas

- ⚠️ Las cuentas creadas son de **TESTNET** y no tienen valor monetario real
- 🔒 Guarda las **Secret Keys** de forma segura
- 💰 Cada cuenta viene fondeada con 10,000 XLM de testnet automáticamente
- 📁 Las cuentas también se documentan en `cuentas-stellar-testnet.md`

---

## 💸 Ejercicio 2: Sistema de Pagos Automatizado

### 📝 Descripción

Crear un sistema automatizado que envíe pagos a múltiples destinatarios en la red Stellar Testnet. Cada pago incluye un memo único para identificarlo.

### 🎯 Objetivos

- Enviar pagos a múltiples destinatarios en una sola ejecución
- Cada pago debe tener un memo único identificando la transacción
- Verificar que cada transacción fue exitosa antes de proceder con la siguiente
- Mostrar el hash de cada transacción para seguimiento
- Generar un resumen final con todas las transacciones

### 📄 Archivo

`ejercicio-2-pagos.js`

### 💻 Código

```javascript
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

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// Cuenta #1 (fuente - envía pagos)
const SECRET_KEY = 'SCHKVPBYYFG4KBG2JIKVXSNTPIQZYH4CQTSZIJ3L6KVEKG4SL22W7HLS';
const CUENTA_1 = 'GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF';

// Destinatarios: Cuentas #2, #3, #4
const destinatarios = [
  { numero: 1, publicKey: 'GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI', memo: 'Pago-001' },
  { numero: 2, publicKey: 'GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ', memo: 'Pago-002' },
  { numero: 3, publicKey: 'GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V', memo: 'Pago-003' }
];

async function enviarPago(sourceKey, destination, amount, memo) {
  try {
    const sourceKeys = Keypair.fromSecret(sourceKey);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: destination,
        asset: Asset.native(),
        amount: amount.toString()
      }))
      .addMemo(Memo.text(memo))
      .setTimeout(30)
      .build();
    
    transaction.sign(sourceKeys);
    
    const result = await server.submitTransaction(transaction);
    return result;
    
  } catch (error) {
    console.error('Error:', error.message);
    throw error;
  }
}

async function sistemaPagos(destinos, amount) {
  console.log('╔════════════════════════════════════════╗');
  console.log('💸 SISTEMA DE PAGOS AUTOMATIZADO');
  console.log('╚════════════════════════════════════════╝\n');
  
  const resultados = [];
  
  for (const destino of destinos) {
    try {
      console.log(`💰 [${destino.numero}/3] Enviando ${amount} XLM...`);
      console.log(`   Destino: ${destino.publicKey.substring(0, 8)}...`);
      console.log(`   Memo: ${destino.memo}`);
      
      const result = await enviarPago(SECRET_KEY, destino.publicKey, amount, destino.memo);
      
      resultados.push({
        numero: destino.numero,
        destino: destino.publicKey,
        monto: amount,
        memo: destino.memo,
        hash: result.hash,
        estado: '✅ EXITOSO'
      });
      
      console.log(`   ✅ Hash: ${result.hash}\n`);
      
    } catch (error) {
      console.error(`   ❌ Error: ${error.message}\n`);
      
      resultados.push({
        numero: destino.numero,
        destino: destino.publicKey,
        monto: amount,
        memo: destino.memo,
        estado: '❌ FALLIDO'
      });
    }
  }
  
  console.log('╔════════════════════════════════════════╗');
  console.log('📊 RESUMEN DE TRANSACCIONES');
  console.log('╚════════════════════════════════════════╝\n');
  
  resultados.forEach(r => {
    console.log(`${r.estado} - Pago ${r.numero}`);
    console.log(`   Monto: ${r.monto} XLM`);
    console.log(`   Memo: ${r.memo}`);
    if (r.hash) console.log(`   Hash: ${r.hash}`);
    console.log('');
  });
  
  return resultados;
}

const transacciones = await sistemaPagos(destinatarios, '2');
```

### 🚀 Ejecución

```bash
npm run ejercicio-2-pagos
```

O directamente:

```bash
node ejercicio-2-pagos.js
```

### ✅ Resultados

El script envía 2 XLM a cada uno de los 3 destinatarios configurados. Cada transacción incluye un memo único (Pago-001, Pago-002, Pago-003) y muestra el hash de la transacción para seguimiento.

### 📊 Ejemplo de Salida (Resultados Reales)

```
╔════════════════════════════════════════╗
💸 SISTEMA DE PAGOS AUTOMATIZADO
╚════════════════════════════════════════╝

💰 [1/3] Enviando 2 XLM...
   Destino: GAL5VGY5...
   Memo: Pago-001
   ✅ Hash: 429a95668d72bbe36ce31b4d329b54055cec231c8be59ce67f96279d1d9fa3a5

💰 [2/3] Enviando 2 XLM...
   Destino: GBKCFHUM...
   Memo: Pago-002
   ✅ Hash: 5f3bf23f27ef90d0a6d9326b230685ae8d2f9a20819e81cb93d9e329527454c2

💰 [3/3] Enviando 2 XLM...
   Destino: GANMMUWU...
   Memo: Pago-003
   ✅ Hash: 07de0d5128a392e82995ef22fe3c8be3d7c073a5d18be75b728aaca75db954df

╔════════════════════════════════════════╗
📊 RESUMEN DE TRANSACCIONES
╚════════════════════════════════════════╝

✅ EXITOSO - Pago 1
   Monto: 2 XLM
   Memo: Pago-001
   Hash: 429a95668d72bbe36ce31b4d329b54055cec231c8be59ce67f96279d1d9fa3a5

✅ EXITOSO - Pago 2
   Monto: 2 XLM
   Memo: Pago-002
   Hash: 5f3bf23f27ef90d0a6d9326b230685ae8d2f9a20819e81cb93d9e329527454c2

✅ EXITOSO - Pago 3
   Monto: 2 XLM
   Memo: Pago-003
   Hash: 07de0d5128a392e82995ef22fe3c8be3d7c073a5d18be75b728aaca75db954df
```

### 🔧 Configuración

**Cuenta Fuente (envía pagos):**
- Secret Key: La cuenta #1 creada en el Ejercicio 1
- Public Key: `GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF`

**Destinatarios:**
- Cuenta #2: `GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI`
- Cuenta #3: `GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ`
- Cuenta #4: `GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V`

### 📝 Notas

- ⚠️ Asegúrate de tener suficiente balance en la cuenta fuente (al menos 6 XLM + fees para 3 pagos de 2 XLM)
- 🔒 La Secret Key debe ser de una cuenta válida y fondeada
- 📝 Cada pago tiene un memo único para identificarlo
- 🔗 Los hashes de transacción permiten verificar las transacciones en el explorador de Stellar Testnet
- ⚙️ El script maneja errores y muestra un resumen de transacciones exitosas y fallidas

### 🛠️ Conceptos Utilizados

- **TransactionBuilder**: Construye transacciones en Stellar
- **Operation.payment**: Operación de pago
- **Asset.native()**: Asset nativo (XLM)
- **Memo.text()**: Agregar memos a las transacciones
- **Keypair.fromSecret()**: Crear keypair desde secret key
- **server.submitTransaction()**: Enviar transacción a la red

---

## 📊 Ejercicio 3: Monitor de Cuentas

### 📝 Descripción

Crear un sistema de monitoreo que consulte y muestre información detallada de múltiples cuentas de Stellar Testnet. El monitor muestra el balance, número de trustlines y el sequence number de cada cuenta.

### 🎯 Objetivos

- Consultar información de múltiples cuentas en una sola ejecución
- Mostrar el balance en XLM de cada cuenta
- Mostrar el número de trustlines (assets personalizados) de cada cuenta
- Mostrar el sequence number de cada cuenta
- Manejar errores si alguna cuenta no existe o no es válida

### 📄 Archivo

`ejercicio-3-monitor.js`

### 💻 Código

```javascript
import { Horizon } from '@stellar/stellar-sdk';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');

// Cuentas a monitorear: Cuentas #1, #2, #3, #4
const cuentasAMonitorear = [
  'GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF', // Cuenta #1
  'GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI', // Cuenta #2
  'GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ', // Cuenta #3
  'GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V'  // Cuenta #4
];

async function monitorearBalances(publicKeys) {
  console.log('╔════════════════════════════════════════╗');
  console.log('📊 MONITOR DE CUENTAS');
  console.log('╚════════════════════════════════════════╝\n');

  for (let i = 0; i < publicKeys.length; i++) {
    const pk = publicKeys[i];

    try {
      const account = await server.loadAccount(pk);

      const xlmBalance = account.balances[0].balance;
      const numTrustlines = account.balances.length - 1;
      const sequenceNumber = account.sequenceNumber();

      console.log(`📍 Cuenta ${i + 1}:`);
      console.log(`   Public Key: ${pk}`);
      console.log(`   💰 Balance: ${xlmBalance} XLM`);
      console.log(`   🔗 Trustlines: ${numTrustlines}`);
      console.log(`   🔢 Sequence: ${sequenceNumber}`);
      console.log('');

    } catch (error) {
      console.error(`❌ Error en ${pk.substring(0, 8)}...: ${error.message}`);
    }
  }
}

await monitorearBalances(cuentasAMonitorear);
```

### 🚀 Ejecución

```bash
npm run ejercicio-3-monitor
```

O directamente:

```bash
node ejercicio-3-monitor.js
```

### ✅ Resultados

El script consulta y muestra información detallada de cada cuenta configurada, incluyendo:

- **Public Key**: La dirección pública de la cuenta
- **Balance**: Balance en XLM (nativo)
- **Trustlines**: Número de assets personalizados (trustlines) que la cuenta tiene
- **Sequence Number**: Número de secuencia de la cuenta (útil para transacciones)

### 📊 Ejemplo de Salida (Resultados Reales)

```
╔════════════════════════════════════════╗
📊 MONITOR DE CUENTAS
╚════════════════════════════════════════╝

📍 Cuenta 1:
   Public Key: GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF
   💰 Balance: 9987.9999400 XLM
   🔗 Trustlines: 0
   🔢 Sequence: 6109453539606534

📍 Cuenta 2:
   Public Key: GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI
   💰 Balance: 10004.0000000 XLM
   🔗 Trustlines: 0
   🔢 Sequence: 6109457834573824

📍 Cuenta 3:
   Public Key: GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ
   💰 Balance: 10004.0000000 XLM
   🔗 Trustlines: 0
   🔢 Sequence: 6109462129541120

📍 Cuenta 4:
   Public Key: GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V
   💰 Balance: 10004.0000000 XLM
   🔗 Trustlines: 0
   🔢 Sequence: 6109466424508416
```

### 🔧 Configuración

Las cuentas a monitorear están configuradas en el array `cuentasAMonitorear`. Por defecto, se monitorean las cuentas #1, #2, #3 y #4 creadas en el Ejercicio 1.

**Cuentas configuradas:**
- Cuenta #1: `GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF`
- Cuenta #2: `GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI`
- Cuenta #3: `GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ`
- Cuenta #4: `GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V`

### 📝 Notas

- ⚠️ El balance mostrado puede variar si se han realizado transacciones después de la creación inicial
- 🔗 **Trustlines**: Representa el número de assets personalizados (no nativos) que la cuenta tiene. Si es 0, la cuenta solo tiene XLM nativo
- 🔢 **Sequence Number**: Se incrementa con cada transacción. Es importante para evitar transacciones duplicadas
- ❌ Si una cuenta no existe o no es válida, se mostrará un error pero el script continuará con las demás cuentas
- 💡 Puedes agregar o quitar cuentas del array `cuentasAMonitorear` para monitorear diferentes cuentas

### 🛠️ Conceptos Utilizados

- **server.loadAccount()**: Carga información de una cuenta desde Horizon
- **account.balances**: Array con todos los balances de la cuenta (XLM nativo + trustlines)
- **account.balances[0]**: Primer balance (siempre es XLM nativo)
- **account.sequenceNumber()**: Obtiene el número de secuencia de la cuenta
- **Manejo de errores**: Try-catch para manejar cuentas inválidas o inexistentes

### 💡 Casos de Uso

- **Monitoreo de balances**: Verificar balances de múltiples cuentas
- **Auditoría**: Revisar el estado de cuentas después de transacciones
- **Detección de cambios**: Comparar balances antes y después de operaciones
- **Verificación de trustlines**: Ver qué assets personalizados tiene una cuenta

---

## 🚀 Ejercicio 4: Configuración de Proyecto

### 📝 Descripción

Aprender a configurar un proyecto Node.js desde cero para trabajar con Stellar SDK. Este ejercicio muestra los pasos necesarios para crear un nuevo proyecto, instalar las dependencias y configurar el entorno de desarrollo.

### 🎯 Objetivos

- Crear un nuevo directorio de proyecto
- Inicializar un proyecto Node.js con npm
- Instalar el SDK de Stellar
- Configurar package.json para usar módulos ES6
- Verificar la configuración del proyecto

### 📄 Archivo

`ejercicio-4-setup-proyecto.js`

### 💻 Código

```javascript
import fs from 'fs';
import { execSync } from 'child_process';

console.log('╔════════════════════════════════════════╗');
console.log('🚀 EJERCICIO 4: CONFIGURACIÓN DE PROYECTO');
console.log('╚════════════════════════════════════════╝\n');

console.log('📋 Pasos para configurar un proyecto Stellar desde cero:\n');

console.log('1️⃣  Crear directorio del proyecto:');
console.log('   mkdir stellar-tarea2');
console.log('   cd stellar-tarea2\n');

console.log('2️⃣  Inicializar proyecto Node.js:');
console.log('   npm init -y\n');

console.log('3️⃣  Instalar Stellar SDK:');
console.log('   npm install @stellar/stellar-sdk\n');

console.log('4️⃣  Configurar package.json con módulos ES6:');
console.log('   Agregar "type": "module" en package.json\n');

// Verificar configuración actual
console.log('╔════════════════════════════════════════╗');
console.log('✅ VERIFICACIÓN DE CONFIGURACIÓN ACTUAL');
console.log('╚════════════════════════════════════════╝\n');

try {
  const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  
  console.log('📦 package.json encontrado:');
  console.log(`   Nombre: ${packageJson.name}`);
  console.log(`   Versión: ${packageJson.version}`);
  
  if (packageJson.type === 'module') {
    console.log('   ✅ "type": "module" configurado correctamente');
  } else {
    console.log('   ⚠️  "type": "module" no está configurado');
  }
  
  if (packageJson.dependencies && packageJson.dependencies['@stellar/stellar-sdk']) {
    const sdkVersion = packageJson.dependencies['@stellar/stellar-sdk'];
    console.log(`   ✅ @stellar/stellar-sdk instalado: ${sdkVersion}`);
  } else {
    console.log('   ⚠️  @stellar/stellar-sdk no está instalado');
  }
  
  console.log('\n📝 Scripts disponibles:');
  if (packageJson.scripts) {
    Object.keys(packageJson.scripts).forEach(script => {
      console.log(`   - npm run ${script}`);
    });
  }
  
  console.log('\n✅ Proyecto configurado correctamente!\n');
  
} catch (error) {
  console.error('❌ Error al leer package.json:', error.message);
}

console.log('╔════════════════════════════════════════╗');
console.log('📄 CONTENIDO RECOMENDADO DE package.json');
console.log('╚════════════════════════════════════════╝\n');

const ejemploPackageJson = {
  "name": "stellar-tarea2",
  "version": "1.0.0",
  "description": "Scripts de JavaScript para trabajar con Stellar SDK",
  "type": "module",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [
    "stellar",
    "blockchain",
    "cryptocurrency"
  ],
  "author": "",
  "license": "MIT",
  "dependencies": {
    "@stellar/stellar-sdk": "^latest"
  }
};

console.log(JSON.stringify(ejemploPackageJson, null, 2));
console.log('\n');
```

### 🚀 Ejecución

```bash
npm run ejercicio-4-setup
```

O directamente:

```bash
node ejercicio-4-setup-proyecto.js
```

### 📋 Pasos Manuales para Configurar un Proyecto Nuevo

Si quieres crear un proyecto desde cero, sigue estos pasos:

```bash
# 1. Crear directorio del proyecto
mkdir stellar-tarea2
cd stellar-tarea2

# 2. Inicializar proyecto Node.js
npm init -y

# 3. Instalar Stellar SDK
npm install @stellar/stellar-sdk
```

### 📄 Configuración de package.json

Después de ejecutar `npm init -y`, edita el `package.json` y agrega:

```json
{
  "name": "stellar-tarea2",
  "version": "1.0.0",
  "description": "Scripts de JavaScript para trabajar con Stellar SDK",
  "type": "module",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [
    "stellar",
    "blockchain",
    "cryptocurrency"
  ],
  "author": "",
  "license": "MIT",
  "dependencies": {
    "@stellar/stellar-sdk": "^latest"
  }
}
```

**⚠️ Importante**: La propiedad `"type": "module"` es esencial para poder usar `import` y `export` en lugar de `require` y `module.exports`.

### ✅ Resultados

El script muestra los pasos necesarios para configurar un proyecto y verifica la configuración actual del proyecto, incluyendo:

- ✅ Verificación de que `package.json` existe
- ✅ Verificación de que `"type": "module"` está configurado
- ✅ Verificación de que `@stellar/stellar-sdk` está instalado
- ✅ Lista de scripts disponibles en el proyecto
- ✅ Ejemplo de `package.json` recomendado

### 📊 Ejemplo de Salida (Resultados Reales)

```
╔════════════════════════════════════════╗
🚀 EJERCICIO 4: CONFIGURACIÓN DE PROYECTO
╚════════════════════════════════════════╝

📋 Pasos para configurar un proyecto Stellar desde cero:

1️⃣  Crear directorio del proyecto:
   mkdir stellar-tarea2
   cd stellar-tarea2

2️⃣  Inicializar proyecto Node.js:
   npm init -y

3️⃣  Instalar Stellar SDK:
   npm install @stellar/stellar-sdk

4️⃣  Configurar package.json con módulos ES6:
   Agregar "type": "module" en package.json

╔════════════════════════════════════════╗
✅ VERIFICACIÓN DE CONFIGURACIÓN ACTUAL
╚════════════════════════════════════════╝

📦 package.json encontrado:
   Nombre: tarea-2-stellar-scripts
   Versión: 1.0.0
   ✅ "type": "module" configurado correctamente
   ✅ @stellar/stellar-sdk instalado: ^14.3.0

📝 Scripts disponibles:
   - npm run crear-cuenta
   - npm run crear-cuenta-mejorado
   - npm run enviar-pago
   ... (más scripts)

✅ Proyecto configurado correctamente!

╔════════════════════════════════════════╗
📄 CONTENIDO RECOMENDADO DE package.json
╚════════════════════════════════════════╝

{
  "name": "stellar-tarea2",
  "version": "1.0.0",
  "description": "Scripts de JavaScript para trabajar con Stellar SDK",
  "type": "module",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [
    "stellar",
    "blockchain",
    "cryptocurrency"
  ],
  "author": "",
  "license": "MIT",
  "dependencies": {
    "@stellar/stellar-sdk": "^latest"
  }
}
```

### 📝 Notas

- ⚠️ **"type": "module"** es esencial para usar sintaxis ES6 (`import`/`export`)
- 📦 El SDK de Stellar se instala con `npm install @stellar/stellar-sdk`
- 🔧 Puedes usar `npm init -y` para crear un `package.json` básico automáticamente
- 📝 El script verifica la configuración actual del proyecto
- 💡 Puedes personalizar el `package.json` según tus necesidades

### 🛠️ Conceptos Utilizados

- **npm init**: Inicializa un nuevo proyecto Node.js
- **npm install**: Instala dependencias del proyecto
- **package.json**: Archivo de configuración del proyecto
- **"type": "module"**: Habilita módulos ES6 en Node.js
- **fs.readFileSync()**: Lee archivos de forma sincrónica
- **JSON.parse()**: Parsea JSON desde un string

### 💡 Casos de Uso

- **Configuración inicial**: Crear un nuevo proyecto Stellar desde cero
- **Verificación**: Comprobar que un proyecto está configurado correctamente
- **Documentación**: Mostrar los pasos necesarios para configurar un proyecto
- **Onboarding**: Guiar a nuevos desarrolladores en la configuración

---

## 📝 Notas Generales

### ⚠️ Importante

- Todos los ejercicios están configurados para usar **Stellar Testnet**
- Las llaves y transacciones son de **PRUEBA** y no tienen valor monetario real
- 🔒 **NUNCA** compartas tus Secret Keys públicamente
- 💾 Guarda todas las llaves en un lugar seguro

### 🔗 Enlaces Útiles

- **Horizon Testnet**: https://horizon-testnet.stellar.org/
- **Friendbot**: https://friendbot.stellar.org/
- **Explorador Testnet**: https://stellar.expert/explorer/testnet

### 📚 Recursos

- Documentación oficial: https://developers.stellar.org/
- SDK de Stellar: https://stellar.github.io/js-stellar-sdk/

---

## 📅 Próximos Ejercicios

Los ejercicios adicionales de la Semana 2 se agregarán aquí conforme se vayan desarrollando.

---

*Última actualización: Semana 2 - Ejercicios 1, 2, 3 y 4 completados*

