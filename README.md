# 🚀 Stellar Codigo Futura - Proyecto Semana 2

Este proyecto contiene scripts de JavaScript para trabajar con Stellar SDK en la red Testnet. Todos los ejercicios están diseñados para aprender y practicar con la blockchain de Stellar.

## 📋 Ejercicios

### Ejercicio 1: Creación Masiva de Cuentas

- Crea 5 cuentas automáticamente
- Todas fondeadas con Friendbot (10,000 XLM cada una)
- Ejecutar: `node crear-multiples-cuentas.js`

O con npm:

```bash
npm run crear-multiples
```

### Ejercicio 2: Sistema de Pagos

- Envía 2 XLM a 3 cuentas diferentes
- Cada pago con memo único
- Ejecutar: `node ejercicio-2-pagos.js`

O con npm:

```bash
npm run ejercicio-2-pagos
```

### Ejercicio 3: Monitor de Balances

- Verifica balances de 4 cuentas
- Muestra balance, trustlines, sequence
- Ejecutar: `node ejercicio-3-monitor.js`

O con npm:

```bash
npm run ejercicio-3-monitor
```

### Ejercicio 4: Configuración de Proyecto

- Muestra los pasos para configurar un proyecto Stellar desde cero
- Verifica la configuración actual del proyecto
- Ejecutar: `node ejercicio-4-setup-proyecto.js`

O con npm:

```bash
npm run ejercicio-4-setup
```

## 📊 Cuentas Principales Utilizadas

### Cuenta #1 (Principal - Envía pagos)

**Public Key**: `GAY7MEJJMSXRQZKEWYFWGWBIZRONXK5BPBW2SZFFQINFSXUREVQN25PF`

**Secret Key**: `SCHKVPBYYFG4KBG2JIKVXSNTPIQZYH4CQTSZIJ3L6KVEKG4SL22W7HLS` ⚠️ NUNCA COMPARTIR

### Cuentas Destinatarias

- **Cuenta #2**: `GAL5VGY5Z4KGVTQ5UVOCCOWSYZPQV34EM3I676GJ3BRSFDMMN3K36MZI`
- **Cuenta #3**: `GBKCFHUMWDK5RENETIM264U357ERDYXPPMFTYVW4X4LTE6SJ4JX4GXSQ`
- **Cuenta #4**: `GANMMUWU6NNTXXYRPS2FA7HQIGP3QAJA3HMMLACHJS5D7AWXD7KKOH7V`
- **Cuenta #5**: `GB7CRYRZLAIEJZ6LZDGIT26QYU7Z3VB5KYZQ4COOC37YBKBXKPZMKH6M`

⚠️ **IMPORTANTE**: Estas son cuentas de **TESTNET** y no tienen valor monetario real.

## 🚀 Setup

### Requisitos Previos

- Node.js (v14 o superior)
- npm (v6 o superior)

### Instalación

1. **Clonar o descargar el proyecto**

```bash
cd StellarCodigoFutura
```

2. **Instalar dependencias**

```bash
npm install
```

Esto instalará automáticamente:
- `@stellar/stellar-sdk` - SDK oficial de Stellar
- `dotenv` - Para variables de entorno (opcional)

3. **Verificar configuración**

El proyecto ya está configurado con:
- `"type": "module"` en `package.json` para usar sintaxis ES6
- Dependencias instaladas

### Configuración Inicial (Si creas un proyecto nuevo)

Si quieres crear un proyecto desde cero:

```bash
# 1. Crear directorio del proyecto
mkdir stellar-tarea2
cd stellar-tarea2

# 2. Inicializar proyecto Node.js
npm init -y

# 3. Instalar Stellar SDK
npm install @stellar/stellar-sdk

# 4. Configurar package.json
# Agregar "type": "module" en package.json
```

Ejemplo de `package.json`:

```json
{
  "name": "stellar-tarea2",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@stellar/stellar-sdk": "^latest"
  }
}
```

## 📝 Scripts Disponibles

Puedes ejecutar los ejercicios usando npm:

```bash
# Ejercicio 1: Crear múltiples cuentas
npm run crear-multiples

# Ejercicio 2: Sistema de pagos
npm run ejercicio-2-pagos

# Ejercicio 3: Monitor de balances
npm run ejercicio-3-monitor

# Ejercicio 4: Verificar configuración
npm run ejercicio-4-setup
```

O ejecutar directamente con Node.js:

```bash
node crear-multiples-cuentas.js
node ejercicio-2-pagos.js
node ejercicio-3-monitor.js
node ejercicio-4-setup-proyecto.js
```

## 📚 Documentación

Para más detalles sobre cada ejercicio, consulta el archivo:

- **[SEMANA-2.md](./SEMANA-2.md)** - Documentación completa de todos los ejercicios con código, resultados y explicaciones

## 🔗 Enlaces Útiles

- **Horizon Testnet**: https://horizon-testnet.stellar.org/
- **Friendbot**: https://friendbot.stellar.org/
- **Explorador Testnet**: https://stellar.expert/explorer/testnet
- **Documentación Stellar**: https://developers.stellar.org/
- **SDK de Stellar**: https://stellar.github.io/js-stellar-sdk/

## ⚠️ Advertencias Importantes

- 🔒 **NUNCA** compartas tus Secret Keys públicamente
- 💾 Guarda todas las llaves en un lugar seguro
- ⚠️ Estas son cuentas de **TESTNET** y no tienen valor monetario real
- 🧪 Todos los scripts están configurados para usar **Stellar Testnet**

## 📄 Archivos Importantes

- `SEMANA-2.md` - Documentación completa de ejercicios
- `cuentas-stellar-testnet.md` - Información detallada de las cuentas creadas
- `package.json` - Configuración del proyecto y scripts

## 🛠️ Tecnologías Utilizadas

- **Node.js** - Entorno de ejecución JavaScript
- **@stellar/stellar-sdk** - SDK oficial de Stellar
- **ES6 Modules** - Sintaxis moderna de JavaScript

## 📝 Licencia

MIT

---

**Desarrollado para el curso de Stellar - Semana 2**
