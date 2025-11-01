# 🚀 Smart Contracts con Stellar (Soroban)

**Tu primer contrato inteligente en Stellar**

---

## 🎯 QUÉ VAS A APRENDER

- ✅ Qué son los Smart Contracts en Stellar (Soroban)
- ✅ Cómo crear tu primer contrato
- ✅ Cómo deployarlo
- ✅ Cómo invocarlo desde JavaScript
- ✅ Casos de uso prácticos

---

## 📚 ¿QUÉ ES SOROBAN?

**Soroban** es la plataforma de Smart Contracts de Stellar. Te permite:

- ✅ Escribir contratos inteligentes en Rust o AssemblyScript
- ✅ Deployar contratos en la red Stellar
- ✅ Invocar funciones desde JavaScript/TypeScript
- ✅ Crear dApps (aplicaciones descentralizadas)

---

## 🔧 CONFIGURACIÓN INICIAL

### Instalación de Herramientas

```bash
# Instalar Stellar CLI (incluye herramientas de Soroban)
npm install -g @stellar/cli

# Verificar instalación
stellar --version
soroban --version
```

### Crear Proyecto

```bash
# Crear nuevo proyecto de contrato
stellar contract new mi-primer-contrato

# O usar Soroban CLI directamente
soroban contract new mi-primer-contrato
```

---

## 📝 TU PRIMER CONTRATO (Rust)

### Contrato Simple

Crear archivo `src/lib.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, symbol_short};

#[contract]
pub struct MiContrato;

#[contractimpl]
impl MiContrato {
    pub fn hola(env: Env) -> symbol_short {
        symbol_short!("Hola Stellar!")
    }
    
    pub fn sumar(env: Env, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

### Compilar

```bash
# Desde el directorio del contrato
soroban contract build
```

---

## 📝 TU PRIMER CONTRATO (JavaScript/AssemblyScript)

### Contrato Simple

Crear archivo `index.ts`:

```typescript
import { symbolStrToVal, Val } from '@stellar/stellar-sdk/contract';

export function hello(): Val {
    return symbolStrToVal('Hola Stellar!');
}

export function add(a: number, b: number): number {
    return a + b;
}
```

### Compilar

```bash
# Instalar dependencias
npm install @stellar/stellar-sdk

# Compilar
# (El proceso depende del framework usado)
```

---

## 🚀 DEPLOYAR CONTRATO

### Deploy a Testnet

**✅ IMPLEMENTADO:** Ver scripts [`deploy-contract.sh`](./deploy-contract.sh) y [`deploy-contract.ps1`](./deploy-contract.ps1)

```bash
# Windows PowerShell
npm run deploy-contract

# Linux/Mac Bash
npm run deploy-contract

# O directamente
bash deploy-contract.sh
# O en Windows
.\deploy-contract.ps1
```

**El script automáticamente:**
1. Descarga el contrato hello.wasm
2. Verifica/crea identity
3. Deploya a Testnet
4. Guarda el Contract ID en `.contract-id`
5. Muestra la URL de StellarExpert

### Verificar Deploy

```bash
# Ver información del contrato
npm run invoke-contract-js "Ana"

# O usar CLI
npm run invoke-contract
```

---

## 💻 INVOCAR DESDE JAVASCRIPT

### Instalación

```bash
npm install @stellar/stellar-sdk
```

### Código de Invocación

**✅ IMPLEMENTADO:** Ver archivo [`invoke-contract.js`](./invoke-contract.js)

**Uso:**

```bash
npm run invoke-contract-js "Ana"
```

**O edita el archivo directamente:**

```javascript
import { Horizon, Keypair, Networks, nativeToScVal, scValToNative } from '@stellar/stellar-sdk';
import { Contract } from '@stellar/stellar-sdk/contract';

const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;

// ID del contrato deployado
const CONTRACT_ID = 'CONTRACT_ID_AQUI';
const SECRET_KEY = 'SBXXXXX...'; // Tu secret key

async function invocarContrato() {
  try {
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
    
    // Crear instancia del contrato
    const contract = new Contract(CONTRACT_ID);
    
    // Invocar función "hola"
    const result = await contract.invoke({
      method: 'hola',
      networkPassphrase: networkPassphrase,
      source: sourceAccount,
      signers: [sourceKeys]
    });
    
    console.log('✅ Resultado:', scValToNative(result));
    
    // Invocar función "sumar"
    const result2 = await contract.invoke({
      method: 'sumar',
      args: [
        nativeToScVal(10, { type: 'i32' }),
        nativeToScVal(20, { type: 'i32' })
      ],
      networkPassphrase: networkPassphrase,
      source: sourceAccount,
      signers: [sourceKeys]
    });
    
    console.log('✅ Suma:', scValToNative(result2));
    
  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

invocarContrato();
```

---

## 🎯 CASOS DE USO PRÁCTICOS

### 1. Contrato de Token Simple

```rust
pub fn mint(env: Env, to: Address, amount: i128) {
    // Lógica de minting
}

pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    // Lógica de transferencia
}

pub fn balance(env: Env, owner: Address) -> i128 {
    // Retornar balance
}
```

### 2. Contrato de Votación

```rust
pub fn votar(env: Env, votante: Address, opcion: u32) {
    // Registrar voto
}

pub fn resultados(env: Env) -> Map<u32, i128> {
    // Retornar resultados
}
```

### 3. Contrato de Escrow (Depósito en Garantía)

```rust
pub fn depositar(env: Env, depositor: Address, cantidad: i128) {
    // Guardar depósito
}

pub fn liberar(env: Env, receptor: Address) {
    // Liberar fondos
}
```

---

## 🔍 DEBUGGING Y TESTING

### Testing de Contratos

```bash
# Ejecutar tests
cargo test

# Tests con cobertura
cargo test -- --nocapture
```

### Debug Local

```bash
# Ejecutar contrato localmente
soroban contract invoke \
  --id CONTRACT_ID \
  --wasm target/wasm32-unknown-unknown/release/mi_contrato.wasm \
  -- hello
```

---

## 📚 EJERCICIOS DE PRÁCTICA

### Ejercicio 1: Contrato de Contador

Crea un contrato que:
1. Mantenga un contador
2. Permita incrementar
3. Permita decrementar
4. Permita leer el valor

### Ejercicio 2: Contrato de Mensajería

Crea un contrato que:
1. Permita enviar mensajes
2. Permita leer mensajes por ID
3. Mantenga un registro de todos los mensajes

### Ejercicio 3: Contrato de Wallet Multisig

Crea un contrato que:
1. Permita múltiples signers
2. Requiera N de M firmas para ejecutar operaciones
3. Permita agregar/remover signers

---

## 🔗 RECURSOS ADICIONALES

- [Soroban Docs](https://soroban.stellar.org/docs)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Stellar SDK Contract API](https://stellar.github.io/js-stellar-sdk/modules/contract.html)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

---

## ⚠️ NOTAS IMPORTANTES

1. **Gas Fees:** Los contratos consumen gas. Asegúrate de tener suficiente XLM.
2. **Limitaciones:** Cada función tiene límites de tiempo y memoria.
3. **Testing:** Siempre prueba en Testnet antes de Mainnet.
4. **Seguridad:** Revisa tu código cuidadosamente antes de deployar.

---

## 🚀 SCRIPTS DISPONIBLES

### Deployar Contrato

```bash
npm run deploy-contract
```

### Invocar Contrato

```bash
# Desde CLI
npm run invoke-contract

# Desde JavaScript
npm run invoke-contract-js "Ana"
```

### Invocación Masiva

```bash
npm run invoke-many
# O desde JavaScript
npm run invoke-many-js
```

### Medir Tiempo

```bash
npm run medir-tiempo
```

---

## 📖 DOCUMENTO COMPLETO

Para una explicación detallada paso a paso de todo el proceso, ver:

**📄 [04-smart-contracts-completo.md](./04-smart-contracts-completo.md)**

Este documento incluye:
- ✅ Explicación detallada de cada paso
- ✅ Desglose línea por línea del código Rust
- ✅ Conceptos clave explicados
- ✅ Comparaciones con JavaScript
- ✅ Casos de uso reales
- ✅ Reflexiones y ejercicios

---

**Siguiente:** [05-tarea-y-proximos-pasos.md](./05-tarea-y-proximos-pasos.md)

---

**Creado con ❤️ para las Tiburonas Builders**

