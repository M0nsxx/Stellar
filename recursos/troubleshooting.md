# 🔧 Troubleshooting

**Solución a problemas comunes**

---

## 🐛 ERRORES COMUNES Y SOLUCIONES

### Error: "Cannot use import"

**Síntomas:**
```
SyntaxError: Cannot use import statement outside a module
```

**Causa:** Falta `"type": "module"` en `package.json`

**Solución:**
1. Abre `package.json`
2. Agrega `"type": "module"`:
   ```json
   {
     "type": "module",
     "dependencies": { ... }
   }
   ```
3. Guarda el archivo
4. Vuelve a ejecutar el script

---

### Error: "Cannot find module '@stellar/stellar-sdk'"

**Síntomas:**
```
Error: Cannot find module '@stellar/stellar-sdk'
```

**Causa:** No instalaste las dependencias

**Solución:**
```bash
npm install
```

O si no tienes `package.json`:
```bash
npm install @stellar/stellar-sdk
```

---

### Error: "Server is not a constructor"

**Síntomas:**
```
TypeError: Server is not a constructor
```

**Causa:** SDK v14 requiere importar `Horizon.Server`

**Solución:**
Cambia la importación:

```javascript
// ❌ No funciona en SDK v14
import { Server } from '@stellar/stellar-sdk';

// ✅ Funciona correctamente
import { Horizon } from '@stellar/stellar-sdk';
const Server = Horizon.Server;
```

---

### Error: "Account not found"

**Síntomas:**
```
Error: Account not found
```

**Causa:** La cuenta no existe en blockchain

**Solución:**
1. Verifica que la public key sea correcta (debe empezar con 'G' y tener 56 caracteres)
2. Fondea la cuenta con Friendbot:
   ```javascript
   const response = await fetch(
     `https://friendbot.stellar.org/?addr=${publicKey}`
   );
   ```
3. Verifica que estás usando Testnet (no Mainnet)

---

### Error: "Insufficient balance"

**Síntomas:**
```
Error: Insufficient balance
```

**Causa:** No tienes suficiente XLM

**Solución:**
1. Consulta tu balance:
   ```bash
   node consultar-balance.js
   ```
2. Recuerda que Stellar bloquea reserves:
   - Base reserve: 0.5 XLM
   - Subentry reserve: 0.5 XLM por trustline/offer
3. Fondea más con Friendbot (si es Testnet)

---

### Error: "tx_bad_seq"

**Síntomas:**
```
Error: tx_bad_seq
```

**Causa:** Sequence number incorrecto

**Solución:**
1. Siempre carga la cuenta antes de crear una transacción:
   ```javascript
   const sourceAccount = await server.loadAccount(sourceKeys.publicKey());
   ```
2. No reutilices la misma transacción
3. Si envías múltiples transacciones, espera entre cada una:
   ```javascript
   await new Promise(resolve => setTimeout(resolve, 2000));
   ```

---

### Error: "tx_bad_auth"

**Síntomas:**
```
Error: tx_bad_auth
```

**Causa:** La transacción no está firmada correctamente

**Solución:**
1. Asegúrate de firmar la transacción:
   ```javascript
   transaction.sign(sourceKeys);
   ```
2. Verifica que la secret key sea correcta
3. Verifica que estés usando la misma red (Testnet/Mainnet)

---

### Error: "Network mismatch"

**Síntomas:**
```
Error: Network mismatch
```

**Causa:** Transacción firmada para una red (Testnet) pero enviada a otra (Mainnet)

**Solución:**
1. Verifica que `networkPassphrase` coincida con la red:
   ```javascript
   const networkPassphrase = Networks.TESTNET; // Para Testnet
   // o
   const networkPassphrase = Networks.PUBLIC; // Para Mainnet
   ```
2. Verifica que el `Server` apunte a la misma red:
   ```javascript
   // Testnet
   const server = new Server('https://horizon-testnet.stellar.org');
   
   // Mainnet
   const server = new Server('https://horizon.stellar.org');
   ```

---

## 🔍 DEBUGGING

### Ver Logs Detallados

```javascript
// Agregar logs de debugging
console.log('🔍 Debug:', {
  publicKey: sourceKeys.publicKey(),
  balance: sourceAccount.balances[0].balance,
  sequenceNumber: sourceAccount.sequenceNumber()
});
```

### Ver Errores Completos

```javascript
try {
  // Tu código
} catch (error) {
  console.error('❌ Error completo:', {
    message: error.message,
    stack: error.stack,
    response: error.response?.data
  });
}
```

### Verificar Conectividad

```javascript
// Verificar que puedes conectarte a Horizon
try {
  const server = new Server('https://horizon-testnet.stellar.org');
  const account = await server.loadAccount('GDXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX');
} catch (error) {
  console.error('❌ No puedes conectarte a Horizon:', error.message);
}
```

---

## 📚 RECURSOS DE AYUDA

### Documentación

- [Stellar SDK Docs](https://stellar.github.io/js-stellar-sdk/)
- [Horizon API](https://developers.stellar.org/api/horizon)
- [Stellar Developer Docs](https://developers.stellar.org/)

### Comunidad

- **Stellar Discord:** https://discord.gg/stellardev
- **Stack Overflow:** Busca etiquetas `stellar`, `stellar-sdk`

### Herramientas

- **Stellar Laboratory:** https://laboratory.stellar.org
- **Stellar Expert:** https://stellar.expert/explorer/testnet

---

## 💡 TIPS DE PREVENCIÓN

### 1. Siempre Valida Inputs

```javascript
function validarPublicKey(key) {
  if (!key.startsWith('G') || key.length !== 56) {
    throw new Error('Public key inválida');
  }
  return key;
}
```

### 2. Siempre Maneja Errores

```javascript
try {
  // Operación riesgosa
} catch (error) {
  // Manejo apropiado
  console.error('Error:', error.message);
  throw error; // O manejar según el caso
}
```

### 3. Siempre Verifica Balance

```javascript
const balance = parseFloat(sourceAccount.balances[0].balance);
const amount = parseFloat(amountString);

if (balance < amount + 0.5) { // 0.5 para reserves
  throw new Error('Balance insuficiente');
}
```

---

## 🆘 ¿NO ENCUENTRAS LA SOLUCIÓN?

1. **Revisa la documentación oficial**
2. **Busca en Discord de Stellar**
3. **Pregunta en Stack Overflow**
4. **Revisa los logs detallados**
5. **Contacta al instructor del curso**

---

**Creado con ❤️ para las Tiburonas Builders**

