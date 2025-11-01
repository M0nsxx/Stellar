# 💻 JavaScript y Stellar SDK

**Clase 2 - Tus Scripts Explicados**

---

## 🎯 QUÉ VAS A ENCONTRAR AQUÍ

En esta clase escribiste **3 scripts en JavaScript**. Aquí te explicamos:

- ✅ Cómo funciona cada uno
- ✅ Por qué tomamos cada decisión
- ✅ Cómo modificarlos
- ✅ Qué más puedes hacer

---

## 📦 CONFIGURACIÓN INICIAL

### ¿Qué instalaste?

**Node.js** - El cerebro que ejecuta JavaScript fuera del navegador

**Stellar SDK** - La caja de herramientas para Stellar

```bash
npm install @stellar/stellar-sdk
```

### Configuración importante

**En tu `package.json` agregaste:**

```json
{
  "type": "module"
}
```

**¿Por qué?**  

Esto le dice a Node.js: "Voy a usar `import` en vez de `require`". Es el estilo moderno de JavaScript (ESM).

**⚠️ IMPORTANTE:** El SDK v14.3.0 requiere importar `Server` de forma diferente:

```javascript
import { Horizon } from '@stellar/stellar-sdk';
const Server = Horizon.Server;
```

---

## 🔧 SCRIPT 1: CREAR CUENTA

### El código completo

Ver archivo: [`crear-cuenta.js`](./crear-cuenta.js)

### 🎮 CÓMO EJECUTARLO

```bash
npm run crear-cuenta
# o
node crear-cuenta.js
```

### 🔄 MODIFICACIONES QUE PUEDES HACER

Ver archivo: [`crear-cuenta-mejorado.js`](./crear-cuenta-mejorado.js)

**1. Crear múltiples cuentas**

```bash
node crear-cuenta-mejorado.js --multiples 5
```

**2. Guardar en archivo**

```bash
node crear-cuenta-mejorado.js --guardar mi-cuenta.json
```

**3. Validar formato de llaves**

Ya implementado en `crear-cuenta-mejorado.js`:
- `esPublicKeyValida()` - Valida que empiece con 'G' y tenga 56 caracteres
- `esSecretKeyValida()` - Valida que empiece con 'S' y tenga 56 caracteres

---

## 💸 SCRIPT 2: ENVIAR PAGO

### El código completo

Ver archivo: [`enviar-pago.js`](./enviar-pago.js)

### 🎮 CÓMO EJECUTARLO

**1. Reemplaza tus llaves en el archivo:**

```javascript
const SECRET_KEY = 'TU_SECRET_KEY_AQUI';
const DESTINATION = 'PUBLIC_KEY_DESTINO';
```

**2. Ejecuta:**

```bash
npm run enviar-pago
```

### 🔄 MODIFICACIONES POSIBLES

Ver archivo: [`enviar-pago-mejorado.js`](./enviar-pago-mejorado.js)

**1. Enviar múltiples pagos**

```bash
node enviar-pago-mejorado.js --multiples --cuentas GBXXX... GBYYY... --amount 10
```

**2. Validar balance antes de enviar**

Ya implementado: Verifica balance disponible (considerando reserves)

**3. Múltiples operaciones en una transacción**

```bash
node enviar-pago-mejorado.js --una-tx GBXXX... GBYYY... GBZZZ... 10
```

**Ventaja:** Múltiples pagos en UNA transacción = 1 solo fee.

---

## 📊 SCRIPT 3: CONSULTAR BALANCE

### El código completo

Ver archivo: [`consultar-balance.js`](./consultar-balance.js)

### 🎮 CÓMO EJECUTARLO

**1. Reemplaza la PUBLIC_KEY en el archivo:**

```javascript
const PUBLIC_KEY = 'GBXXX...'; // Cuenta a consultar
```

**2. Ejecuta:**

```bash
npm run consultar-balance
```

---

## 🎯 RESUMEN: LO QUE APRENDISTE

### Patrones que usaste

**1. Patrón async/await**

```javascript
async function miFunc() {
  const resultado = await operacionLenta();
  // Continuar después de que termine
}
```

**¿Cuándo usar?** Cuando necesitas esperar respuestas (API, blockchain, archivos).

**2. Patrón try/catch**

```javascript
try {
  const resultado = await operacionRiesgosa();
} catch (error) {
  console.error('Algo salió mal:', error);
}
```

**¿Cuándo usar?** SIEMPRE que hables con cosas externas.

**3. Patrón Builder**

```javascript
const transaction = new TransactionBuilder(account, config)
  .addOperation(op1)
  .addOperation(op2)
  .setTimeout(30)
  .build();
```

**¿Por qué es útil?** Construyes cosas complejas paso a paso, de forma clara.

### Funciones del SDK que dominaste

| Función | Para qué sirve |
|---------|----------------|
| `Keypair.random()` | Generar nuevas llaves |
| `Keypair.fromSecret()` | Cargar llaves existentes |
| `server.loadAccount()` | Obtener datos de cuenta |
| `TransactionBuilder` | Construir transacciones |
| `Operation.payment()` | Crear operación de pago |
| `transaction.sign()` | Firmar transacción |
| `server.submitTransaction()` | Enviar a blockchain |

---

## 💡 TIPS Y BUENAS PRÁCTICAS

### 1. Manejo de Secret Keys

**✅ MEJOR:** Usa variables de entorno

Ver archivo: [`enviar-pago-dotenv.js`](./enviar-pago-dotenv.js)

```javascript
import dotenv from 'dotenv';
dotenv.config();

const SECRET_KEY = process.env.SECRET_KEY;
```

**Crear archivo `.env`:**

```
SECRET_KEY=SBXXX...
DESTINATION=GBYYY...
```

### 2. Validación de inputs

Ya implementado en `enviar-pago-mejorado.js`:

```javascript
function validarAmount(amount) {
  const num = parseFloat(amount);
  if (isNaN(num) || num <= 0) {
    throw new Error('Amount debe ser un número positivo');
  }
  if (num > 1000000) {
    throw new Error('Amount demasiado grande');
  }
  return num;
}
```

### 3. Logs informativos

✅ Todos los scripts usan emojis y formato claro:
- 🚀 Iniciando...
- ✅ Éxito!
- ⚠️ Advertencia
- ❌ Error

---

## 🔄 EJERCICIOS PARA PRACTICAR

### 1. Script de Airdrop

**✅ COMPLETADO:** Ver archivo [`airdrop.js`](./airdrop.js)

Envía XLM a múltiples cuentas diferentes.

```bash
# Edita el archivo con tus llaves y ejecuta:
npm run airdrop
```

### 2. Monitor de Balance

**✅ COMPLETADO:** Ver archivo [`monitor-balance.js`](./monitor-balance.js)

Script que revisa tu balance cada N segundos.

```bash
# Edita el archivo con tu public key y ejecuta:
npm run monitor
# Con intervalo personalizado (en segundos):
node monitor-balance.js 5
```

### 3. Calculadora de Fees

**✅ COMPLETADO:** Ver archivo [`calculadora-fees.js`](./calculadora-fees.js)

Calcula cuánto costaría enviar N transacciones.

```bash
npm run calculadora-fees
# Ejemplo: 100 transacciones con 1 operación cada una
node calculadora-fees.js 100 1
# Tabla comparativa
node calculadora-fees.js --tabla 10 5
```

---

## 🚀 PROYECTO MINI: TU PRIMERA WALLET

**✅ COMPLETADO:** Ver archivo [`mi-wallet.js`](./mi-wallet.js)

**Funcionalidades:**

1. ✅ Crear nueva cuenta
2. ✅ Cargar cuenta existente
3. ✅ Ver balance
4. ✅ Enviar pago
5. ✅ Ver historial de transacciones

```bash
npm run wallet
```

**Este es un proyecto real.** Si lo completas, tienes una wallet funcional.

---

## 🐛 ERRORES COMUNES

### Error: "Cannot use import"

**Causa:** Falta `"type": "module"` en `package.json`

**Solución:** Ya está configurado en tu `package.json`

### Error: "Server is not a constructor"

**Causa:** SDK v14 requiere importar `Horizon.Server`

**Solución:** Ya corregido en todos los scripts:

```javascript
import { Horizon } from '@stellar/stellar-sdk';
const Server = Horizon.Server;
```

### Error: "Account not found"

**Causa:** La cuenta no existe en blockchain

**Solución:**
1. Verifica que la public key sea correcta
2. Fondea con Friendbot primero
3. Verifica que estás en Testnet

### Error: "Insufficient balance"

**Causa:** No tienes suficiente XLM

**Solución:**
- Verifica tu balance primero
- Recuerda los reserves bloqueados
- Fondea más con Friendbot

---

## 📚 RECURSOS ADICIONALES

- [Stellar SDK Docs](https://stellar.github.io/js-stellar-sdk/)
- [Horizon API](https://developers.stellar.org/api/horizon)
- [Stellar Expert](https://stellar.expert/explorer/testnet)
- [Stellar Laboratory](https://laboratory.stellar.org)

---

**Siguiente:** [03-terminal-y-cli.md](./03-terminal-y-cli.md)

---

**Creado con ❤️ para las Tiburonas Builders**

