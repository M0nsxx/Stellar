# 💡 Conceptos Clave

**Glosario de términos importantes de Stellar**

---

## 🔑 LLAVES Y CUENTAS

### Public Key (Llave Pública)

**¿Qué es?** Dirección pública de tu cuenta en Stellar.

**Características:**
- Empieza con 'G'
- Tiene 56 caracteres
- Puedes compartirla públicamente
- Ejemplo: `GBXM7...`

**Uso:** Para recibir pagos, identificar tu cuenta.

---

### Secret Key (Llave Secreta)

**¿Qué es?** Llave privada que controla tu cuenta.

**Características:**
- Empieza con 'S'
- Tiene 56 caracteres
- **NUNCA** la compartas
- Ejemplo: `SBXM7...`

**Uso:** Para firmar transacciones, controlar tu cuenta.

---

### Account (Cuenta)

**¿Qué es?** Una cuenta en la red Stellar.

**Componentes:**
- Public Key (dirección)
- Secret Key (control)
- Balances (activos)
- Sequence Number (contador de transacciones)

---

## 💰 BALANCES Y RESERVES

### Balance

**¿Qué es?** Cantidad de activos en tu cuenta.

**Tipos:**
- **Native (XLM):** La moneda nativa de Stellar
- **Assets:** Tokens personalizados (USDC, etc.)

---

### Reserves (Bloqueados)

**¿Qué es?** XLM bloqueado en tu cuenta para prevenir spam.

**Tipos:**
- **Base Reserve:** 0.5 XLM (por existir)
- **Subentry Reserve:** 0.5 XLM por cada:
  - Trustline
  - Offer
  - Data entry

**Ejemplo:**
- Si tienes 10 XLM y 1 trustline:
  - Base Reserve: 0.5 XLM
  - Subentry Reserve: 0.5 XLM (1 trustline)
  - Total Reserves: 1.0 XLM
  - Disponible: 9.0 XLM

---

### Available Balance (Balance Disponible)

**¿Qué es?** Balance que puedes usar (Total - Reserves).

**Cálculo:**
```javascript
const available = balance - baseReserve - subentryReserve;
```

---

## 🔗 TRANSACCIONES

### Transaction (Transacción)

**¿Qué es?** Operación o conjunto de operaciones firmadas y listas para enviar.

**Componentes:**
- Source Account (cuenta origen)
- Operations (operaciones a ejecutar)
- Fee (costo)
- Sequence Number (número único)
- Signature (firma)

---

### Operation (Operación)

**¿Qué es?** Acción específica en una transacción.

**Tipos comunes:**
- **Payment:** Enviar XLM/assets
- **Create Account:** Crear nueva cuenta
- **Trust:** Crear trustline
- **Change Trust:** Modificar trustline
- **Manage Data:** Guardar datos en cuenta

---

### Sequence Number

**¿Qué es?** Número único incremental para cada transacción.

**Características:**
- Empieza en 0 (cuenta nueva)
- Incrementa con cada transacción
- Previene replay attacks
- Debe ser mayor que el último usado

**Uso:**
```javascript
const account = await server.loadAccount(publicKey);
const seqNum = account.sequenceNumber(); // Obtiene el siguiente número
```

---

### Fee (Costo)

**¿Qué es?** Costo en stroops por operación.

**Estándar:**
- **BASE_FEE:** 100 stroops por operación
- **1 XLM = 10,000,000 stroops**
- **BASE_FEE = 0.00001 XLM**

**Ejemplo:**
- Transacción con 1 operación: 100 stroops (0.00001 XLM)
- Transacción con 3 operaciones: 300 stroops (0.00003 XLM)

---

### Memo

**¿Qué es?** Campo opcional para agregar información a una transacción.

**Tipos:**
- **Text:** Texto (hasta 28 bytes)
- **ID:** Número
- **Hash:** Hash de 32 bytes
- **Return:** Hash para retorno

**Uso:**
```javascript
.addMemo(Memo.text('Pago #123'))
```

---

## 🌐 REDES

### Testnet

**¿Qué es?** Red de prueba de Stellar (sin dinero real).

**Características:**
- Gratis y público
- Dinero de prueba (XLM fake)
- Para desarrollo y testing
- Friendbot para fondear

**URLs:**
- Horizon: `https://horizon-testnet.stellar.org`
- Friendbot: `https://friendbot.stellar.org`

**Network Passphrase:** `Test SDF Network ; September 2015`

---

### Mainnet

**¿Qué es?** Red principal de Stellar (dinero real).

**Características:**
- Dinero real (XLM real)
- Producción
- Requiere XLM real
- No hay Friendbot

**URLs:**
- Horizon: `https://horizon.stellar.org`

**Network Passphrase:** `Public Global Stellar Network ; September 2015`

---

### Network Passphrase

**¿Qué es?** String que identifica la red.

**Uso:** Para firmar transacciones solo para esa red.

**Ejemplo:**
```javascript
const networkPassphrase = Networks.TESTNET;
// Transacciones firmadas solo funcionan en Testnet
```

---

## 🔐 SEGURIDAD

### Signature (Firma)

**¿Qué es?** Prueba criptográfica de que autorizaste la transacción.

**Cómo funciona:**
- Usa tu Secret Key
- Prueba que tú eres el dueño
- Inmutable una vez firmada

**Uso:**
```javascript
transaction.sign(sourceKeys);
```

---

### Multi-signature (Multifirma)

**¿Qué es?** Sistema que requiere múltiples firmas para ejecutar.

**Casos de uso:**
- Cuentas corporativas
- Seguridad adicional
- Control compartido

---

## 📡 HORIZON

### Horizon API

**¿Qué es?** API REST para interactuar con Stellar.

**Funciones:**
- Consultar cuentas
- Consultar transacciones
- Enviar transacciones
- Consultar balances

**URLs:**
- Testnet: `https://horizon-testnet.stellar.org`
- Mainnet: `https://horizon.stellar.org`

---

### Server

**¿Qué es?** Cliente en JavaScript para conectarse a Horizon.

**Uso:**
```javascript
import { Horizon } from '@stellar/stellar-sdk';
const Server = Horizon.Server;
const server = new Server('https://horizon-testnet.stellar.org');
```

---

## 🚀 SMART CONTRACTS (SOROBAN)

### Smart Contract

**¿Qué es?** Código que se ejecuta en la blockchain.

**Lenguajes:**
- Rust (principal)
- AssemblyScript (en desarrollo)

**Uso:** Automatizar lógica compleja en la blockchain.

---

### Deploy

**¿Qué es?** Publicar un contrato en la red.

**Proceso:**
1. Escribir código
2. Compilar a WASM
3. Subir a la red
4. Obtener Contract ID

---

### Invoke

**¿Qué es?** Llamar una función del contrato.

**Uso:**
```javascript
const result = await contract.invoke({
  method: 'miFuncion',
  args: [...],
  ...
});
```

---

## 📊 TÉRMINOS TÉCNICOS

### Ledger (Libro Mayor)

**¿Qué es?** Un "bloque" en Stellar que contiene transacciones.

**Características:**
- Se cierra cada 3-5 segundos
- Contiene múltiples transacciones
- Inmutable una vez cerrado

---

### Stellar Consensus Protocol (SCP)

**¿Qué es?** Algoritmo de consenso de Stellar.

**Ventajas:**
- Rápido (3-5 segundos)
- Eficiente energéticamente
- Seguro

---

### Stroops

**¿Qué es?** Unidad más pequeña de XLM.

**Conversión:**
- 1 XLM = 10,000,000 stroops
- 1 stroop = 0.0000001 XLM

**Uso:** Para calcular fees precisamente.

---

## 💡 CONCEPTOS AVANZADOS

### Trustline

**¿Qué es?** Conexión entre tu cuenta y un asset.

**Uso:** Para recibir tokens personalizados (USDC, etc.)

**Costo:** 0.5 XLM de reserve por trustline.

---

### Offer (Oferta)

**¿Qué es?** Oferta de compra/venta en el order book.

**Uso:** Para trading de assets.

---

### Anchor

**¿Qué es?** Institución que emite assets y los conecta con el mundo real.

**Ejemplos:**
- Circle (USDC)
- Tempo (EURT)

---

## 📚 RECURSOS ADICIONALES

Para más información:
- [Stellar Developer Docs](https://developers.stellar.org/)
- [Stellar Protocol](https://stellar.org/protocol/)
- [Troubleshooting](./troubleshooting.md)
- [Links Útiles](./links-utiles.md)

---

**Creado con ❤️ para las Tiburonas Builders**

