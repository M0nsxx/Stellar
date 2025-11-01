# ⚡ Terminal y CLI

**Guía completa para trabajar desde la terminal**

---

## 🎯 QUÉ VAS A APRENDER

- ✅ Comandos básicos de terminal
- ✅ Trabajar con Node.js y npm
- ✅ Stellar CLI (stellar-cli)
- ✅ Trucos y tips útiles
- ✅ Automatización con scripts

---

## 📟 COMANDOS BÁSICOS DE TERMINAL

### Windows (PowerShell)

```powershell
# Navegar entre directorios
cd "C:\ruta\a\directorio"
cd ..                          # Volver un directorio atrás
cd ~                           # Ir al directorio home

# Listar archivos
dir                            # Listar archivos
dir *.js                       # Listar solo archivos .js
Get-ChildItem                  # Alternativa más moderna

# Crear directorios
mkdir nuevo-directorio

# Crear archivos
New-Item archivo.js            # Crear archivo vacío
echo "contenido" > archivo.js  # Crear con contenido

# Limpiar pantalla
cls                            # Clear screen

# Ver ayuda
Get-Help comando              # Ayuda de comandos
```

### Linux/Mac (Bash)

```bash
# Navegar entre directorios
cd /ruta/a/directorio
cd ..                          # Volver un directorio atrás
cd ~                           # Ir al directorio home

# Listar archivos
ls                             # Listar archivos
ls *.js                        # Listar solo archivos .js
ls -la                         # Listar con detalles

# Crear directorios
mkdir nuevo-directorio

# Crear archivos
touch archivo.js               # Crear archivo vacío
echo "contenido" > archivo.js  # Crear con contenido

# Limpiar pantalla
clear                          # Clear screen

# Ver ayuda
man comando                    # Manual de comandos
comando --help                 # Ayuda rápida
```

---

## 📦 COMANDOS NODE.JS Y NPM

### Instalación y Configuración

```bash
# Verificar versión de Node.js
node --version
node -v

# Verificar versión de npm
npm --version
npm -v

# Inicializar proyecto
npm init                      # Interactivo
npm init -y                   # Con valores por defecto

# Instalar dependencias
npm install @stellar/stellar-sdk
npm install                   # Instalar todas las dependencias de package.json

# Instalar como dependencia de desarrollo
npm install --save-dev paquete
npm install -D paquete
```

### Ejecutar Scripts

```bash
# Ejecutar archivo JavaScript
node script.js

# Ejecutar con npm scripts (definidos en package.json)
npm run crear-cuenta
npm run enviar-pago
npm run consultar-balance

# Ejecutar con argumentos
node script.js arg1 arg2 arg3

# Ver todos los scripts disponibles
npm run
```

### Gestión de Paquetes

```bash
# Ver paquetes instalados
npm list                      # Local
npm list -g                   # Globales

# Actualizar paquetes
npm update                    # Actualizar todos
npm update @stellar/stellar-sdk  # Actualizar específico

# Desinstalar paquetes
npm uninstall paquete

# Limpiar cache
npm cache clean --force
```

---

## 🦈 STELLAR CLI

### Instalación

```bash
# Instalar Stellar CLI globalmente
npm install -g @stellar/cli

# Verificar instalación
stellar --version
stellar -v
```

### Comandos Principales

```bash
# Ver ayuda general
stellar --help
stellar -h

# Crear cuenta
stellar account create

# Ver información de cuenta
stellar account info GXXXXXXXXX...

# Enviar pago
stellar payment send \
  --from SBXXXXX... \
  --to GBXXXXX... \
  --amount 10 \
  --asset XLM

# Consultar transacciones
stellar tx list GXXXXXXXXX...

# Ver balance
stellar account balance GXXXXXXXXX...
```

### Configuración

```bash
# Configurar red (Testnet por defecto)
stellar config network testnet
stellar config network mainnet

# Configurar cuenta por defecto
stellar config account set GXXXXXXXXX...

# Ver configuración actual
stellar config show
```

---

## 🎨 TRUCOS Y TIPS

### 1. Variables de Entorno

```bash
# Windows PowerShell
$env:SECRET_KEY="SBXXXXX..."
node script.js

# Linux/Mac
SECRET_KEY="SBXXXXX..." node script.js

# O crear archivo .env
# SECRET_KEY=SBXXXXX...
# DESTINATION=GBXXXXX...
```

### 2. Ejecutar Scripts en Segundo Plano

```bash
# Windows PowerShell
Start-Process node -ArgumentList "monitor-balance.js"

# Linux/Mac
node monitor-balance.js &

# Con nohup (no se detiene al cerrar terminal)
nohup node monitor-balance.js &
```

### 3. Redirigir Output

```bash
# Guardar output en archivo
node script.js > output.txt

# Agregar a archivo existente
node script.js >> output.txt

# Ver y guardar al mismo tiempo (Linux/Mac)
node script.js | tee output.txt
```

### 4. Ejecutar Múltiples Comandos

```bash
# Ejecutar secuencialmente (si falla uno, se detiene)
npm install && node crear-cuenta.js

# Ejecutar todos (aunque fallen)
npm install; node crear-cuenta.js

# Ejecutar en paralelo (Linux/Mac)
npm install & node crear-cuenta.js &
```

### 5. Buscar en Archivos

```bash
# Windows PowerShell
Select-String "SECRET_KEY" *.js

# Linux/Mac
grep -r "SECRET_KEY" *.js
```

---

## 🔧 AUTOMATIZACIÓN CON SCRIPTS

### Script npm personalizado

En `package.json`:

```json
{
  "scripts": {
    "crear-cuenta": "node crear-cuenta.js",
    "enviar-pago": "node enviar-pago.js",
    "consultar-balance": "node consultar-balance.js",
    "test-all": "node crear-cuenta.js && node consultar-balance.js"
  }
}
```

Ejecutar:
```bash
npm run test-all
```

### Scripts de Shell (Linux/Mac)

Crear `deploy.sh`:

```bash
#!/bin/bash

echo "🚀 Deployando..."

# Instalar dependencias
npm install

# Ejecutar tests
npm run test

# Crear cuenta de prueba
node crear-cuenta.js

echo "✅ Deploy completado"
```

Ejecutar:
```bash
chmod +x deploy.sh
./deploy.sh
```

### Scripts de PowerShell (Windows)

Crear `deploy.ps1`:

```powershell
Write-Host "🚀 Deployando..." -ForegroundColor Green

# Instalar dependencias
npm install

# Ejecutar tests
npm run test

# Crear cuenta de prueba
node crear-cuenta.js

Write-Host "✅ Deploy completado" -ForegroundColor Green
```

Ejecutar:
```powershell
.\deploy.ps1
```

---

## 🐛 DEBUGGING DESDE TERMINAL

### Node.js Debugger

```bash
# Activar inspector
node --inspect script.js

# Con puerto específico
node --inspect=9229 script.js

# Luego abre Chrome en: chrome://inspect
```

### Logs Detallados

```bash
# Ver logs de npm
npm install --verbose

# Debug mode en Node.js
NODE_ENV=debug node script.js

# Ver errores detallados
node script.js 2>&1 | tee error.log
```

---

## 📚 EJERCICIOS DE PRÁCTICA

### Ejercicio 1: Crear Script de Setup

Crea un script que:
1. Verifique que Node.js está instalado
2. Instale todas las dependencias
3. Cree una cuenta de prueba
4. Muestre el balance

### Ejercicio 2: Script de Batch Processing

Crea un script que:
1. Lea un archivo JSON con múltiples cuentas
2. Envíe pagos a todas
3. Guarde resultados en un archivo de log

### Ejercicio 3: Script de Monitoreo

Crea un script que:
1. Monitoree el balance de una cuenta
2. Guarde logs con timestamps
3. Envíe alerta si el balance baja de cierto nivel

---

## 🔗 RECURSOS ADICIONALES

- [Node.js Docs](https://nodejs.org/docs/)
- [npm Docs](https://docs.npmjs.com/)
- [PowerShell Docs](https://learn.microsoft.com/en-us/powershell/)
- [Bash Guide](https://www.gnu.org/software/bash/manual/)
- [Stellar CLI Docs](https://github.com/stellar/cli)

---

**Siguiente:** [04-smart-contracts.md](./04-smart-contracts.md)

---

**Creado con ❤️ para las Tiburonas Builders**

