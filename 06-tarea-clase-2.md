# 📝 TAREA CLASE 2 - FUNDAMENTOS DE PROGRAMACIÓN STELLAR

**Tu Camino hacia el Desarrollo Blockchain Continúa**

---

## 🎯 INFORMACIÓN GENERAL DE LA TAREA

**Entrega:** Antes del domingo 12/10 a la medianoche por medio de Chamverse dejar el link de GitHub, la tarea puede ser realizada entre los equipos pero recuerden que cada entrega es individual en Chamverse.

**Tiempo estimado:** 4-5 horas  

**Objetivo:** Consolidar todo lo que aprendiste

---

## 💡 FILOSOFÍA DE ESTA TAREA

### 🎓 Objetivos Principales

- **Consolidar** los conocimientos de JavaScript + Stellar SDK

- **Ganar confianza** trabajando con transacciones blockchain

- **Prepararte** para el desafío de Smart Contracts en Rust

- **Desarrollar muscle memory** con los patrones esenciales de Stellar

### 🛡️ Enfoque Anti-Frustración

**Evitamos:**

- Ejercicios excesivamente complejos

- Configuraciones complicadas sin contexto

- Tareas que requieran más de 4 horas

**Promovemos:**

- Refuerzo progresivo de lo aprendido

- Éxitos tempranos que generen confianza

- Tiempo para descansar y procesar

---

## 📂 EJERCICIOS PRÁCTICOS DETALLADOS

### 💡 Orden Recomendado de Ejecución

**Los ejercicios están diseñados para ejecutarse en secuencia:**

1. **Ejercicio 1** → Crea 5 cuentas y las guarda en `cuentas-tarea-clase2.json`
2. **Ejercicio 2** → Lee automáticamente las cuentas del Ejercicio 1 (usa la primera como fuente, las otras 3 como destinatarios)
3. **Ejercicio 3** → Lee automáticamente todas las cuentas del Ejercicio 1 para monitorear

**También puedes configurar manualmente cada ejercicio si prefieres usar tus propias cuentas.**

---

### 🚀 Ejercicio 1: Creación Masiva de Cuentas

**Archivo:** `ejercicio-1-creacion-masiva.js`  

**Objetivo:** Modificar el script para crear 5 cuentas automáticamente

**Requisitos:**

- Usar un bucle `for` para generar 5 keypairs

- Cada cuenta debe ser fondeada con Friendbot

- Mostrar en consola: public key, secret key y balance inicial de cada una

- Guardar toda la información en un array

**Código de referencia (Página 46):**

```javascript
// Ejemplo de estructura del bucle
for (let i = 1; i <= 5; i++) {
    console.log(`Creando cuenta ${i}...`);
    // Tu código aquí
}
```

**Ejecutar:**

```bash
npm run ejercicio-1-creacion-masiva
```

**💡 IMPORTANTE:** Este ejercicio guarda automáticamente todas las cuentas creadas en `cuentas-tarea-clase2.json`. Este archivo será usado automáticamente por los ejercicios 2 y 3.

**✅ IMPLEMENTADO:** Ver archivo [`ejercicio-1-creacion-masiva.js`](./ejercicio-1-creacion-masiva.js)

**Página de referencia:** Página 53 del PDF de la clase n2, lo pueden ver en Chamverse las Tiburonas de Código Futura

---

### 🚀 Ejercicio 2: Sistema de Pagos Automatizado

**Archivo:** `ejercicio-2-pagos-automatizados.js`  

**Objetivo:** Crear un sistema que envíe pagos a múltiples destinos

**Requisitos:**

- Enviar 2 XLM a 3 cuentas diferentes en una sola ejecución

- Cada pago debe tener un memo único identificando el número de transacción

- Verificar que cada transacción fue exitosa antes de proceder con la siguiente

- Mostrar el hash de cada transacción para seguimiento

**Estructura del array:**

```javascript
const destinatarios = [
    { publicKey: "G...1", memo: "Pago-001" },
    { publicKey: "G...2", memo: "Pago-002" },
    { publicKey: "G...3", memo: "Pago-003" }
];
```

**Ejecutar:**

```bash
npm run ejercicio-2-pagos-automatizados
```

**⚠️ IMPORTANTE:** 

**OPCIÓN AUTOMÁTICA (Recomendada):**
1. Ejecuta primero `npm run ejercicio-1-creacion-masiva` para crear las cuentas
2. El Ejercicio 2 cargará automáticamente las cuentas creadas desde el archivo `cuentas-tarea-clase2.json`

**OPCIÓN MANUAL:**
- Edita el archivo y configura `SECRET_KEY` y `DESTINATARIOS_MANUALES` (líneas 20-30)
- Descomenta las líneas y reemplaza con tus valores reales

**✅ IMPLEMENTADO:** Ver archivo [`ejercicio-2-pagos-automatizados.js`](./ejercicio-2-pagos-automatizados.js)

**Página de referencia:** Página 55-62 del PDF de la clase n2, lo pueden ver en Chamverse las Tiburonas de Código Futura

---

### 🔍 Ejercicio 3: Monitor de Balances

**Archivo:** `ejercicio-3-monitor-balances.js`  

**Objetivo:** Desarrollar un monitor que verifique balances de múltiples cuentas

**Requisitos:**

- Aceptar un array de public keys como entrada

- Mostrar para cada cuenta:

  - Balance de XLM

  - Número de trustlines activos

  - Sequence number actual

- Formatear la salida de manera legible

**Ejemplo de salida esperada:**

```
=== MONITOR DE CUENTAS ===

Cuenta: GBXXX...123
  Balance: 100.50 XLM
  Trustlines: 2
  Sequence: 123456789

Cuenta: GBYYY...456
  Balance: 25.00 XLM  
  Trustlines: 0
  Sequence: 987654321
```

**Ejecutar:**

```bash
npm run ejercicio-3-monitor-balances
```

**⚠️ IMPORTANTE:** 

**OPCIÓN AUTOMÁTICA (Recomendada):**
1. Ejecuta primero `npm run ejercicio-1-creacion-masiva` para crear las cuentas
2. El Ejercicio 3 cargará automáticamente todas las cuentas creadas desde el archivo `cuentas-tarea-clase2.json`

**OPCIÓN MANUAL:**
- Edita el archivo y configura `PUBLIC_KEYS_MANUALES` (líneas 22-28)
- Descomenta las líneas y reemplaza con tus public keys reales

**✅ IMPLEMENTADO:** Ver archivo [`ejercicio-3-monitor-balances.js`](./ejercicio-3-monitor-balances.js)

**Página de referencia:** Página 71-72 del PDF de la clase n2, lo pueden ver en Chamverse las Tiburonas de Código Futura

---

## 🆘 CUÁNDO PEDIR AYUDA

### Siempre está bien pedir ayuda si:

- Estás atascada más de 30 minutos

- El error no tiene sentido

- No sabes por dónde empezar

### Dónde pedir ayuda:

- **Discord del curso**

- **Stellar Discord:** https://discord.gg/stellardev (#soroban-dev)

- **Stack Overflow:** Tag [stellar]

- **A tus compañeras**

**La comunidad blockchain es muy colaborativa. ¡Úsala!**

---

## 🐛 DEBUGGING TIPS

### Si algo no funciona:

1. **Lee el error completo** (no solo la primera línea)

2. **Google el error exacto** (entre comillas)

3. **Verifica las versiones** (Node, SDK, CLI)

4. **Revisa la documentación oficial**

5. **Compara con el código de clase**

6. **Pide ayuda después de 30 min**

### Errores comunes y soluciones:

**Error: "Cannot use import"**
- **Solución:** Verifica que `package.json` tenga `"type": "module"`

**Error: "Account not found"**
- **Solución:** Verifica que la cuenta esté fondeada con Friendbot

**Error: "Insufficient balance"**
- **Solución:** Verifica tu balance y los reserves bloqueados

**Error: "tx_bad_seq"**
- **Solución:** Recarga la cuenta con `server.loadAccount()` antes de cada transacción

**Más ayuda:** Ver [`recursos/troubleshooting.md`](./recursos/troubleshooting.md)

---

## 📚 RECURSOS ADICIONALES

### Documentación técnica

- **Stellar SDK JS:** https://stellar.github.io/js-stellar-sdk/

- **Stellar CLI:** https://developers.stellar.org/docs/tools/cli/stellar-cli

- **Soroban Docs:** https://developers.stellar.org/docs/build/smart-contracts

- **Horizon API:** https://developers.stellar.org/api/horizon

---

### Herramientas útiles

- **Laboratory:** https://laboratory.stellar.org

- **StellarExpert:** https://stellar.expert/explorer/testnet

- **Friendbot:** https://friendbot.stellar.org

---

## 🎯 OBJETIVOS DE APRENDIZAJE

### Al completar esta tarea, habrás:

✅ **Consolidado JavaScript + Stellar**

- Transacciones multi-operación

- Programación asíncrona

- Manejo de errores

- Streams en tiempo real

✅ **Dominado Stellar CLI**

- Automatización con bash

- Gestión de identidades

- Deploy de contratos

- Documentación técnica

✅ **Desarrollado pensamiento crítico**

- Investigación de proyectos

- Identificación de problemas

- Propuesta de soluciones

- Escritura técnica

✅ **Ganado confianza**

- En tu habilidad de aprender

- En tu capacidad de construir

- En tu futuro como developer

---

## 💬 REFLEXIÓN PERSONAL

**Antes de empezar la tarea, tómate 5 minutos:**

1. **¿Qué fue lo más difícil de la Clase 2?**

2. **¿Qué fue lo más emocionante?**

3. **¿Qué quieres dominar mejor?**

4. **¿Cómo te sientes con tu progreso?**

**Escribe tus respuestas.** Te ayudará a enfocar tu energía.

---

## 🦈 PALABRAS FINALES

### Un mensaje de Tiburona a Tiburona

> "Esta tarea no es un examen.  
> Es tu oportunidad de practicar.  
> De experimentar.  
> De romper cosas y aprender.  
>  
> No busques perfección.  
> Busca progreso.  
>  
> Cada línea de código que escribas  
> te acerca más a tus metas.  
>  
> No estás sola en esto.  
> Toda la comunidad está aquí para ayudarte.  
>  
> Pregunta.  
> Experimenta.  
> Construye.  
>  
> Porque así son las Tiburonas:  
> Persistentes.  
> Valientes.  
> Imparables."

---

### Tu progreso hasta ahora

**Semana 1 - Clase 1:**

- ✅ Creaste tu primera cuenta Stellar

- ✅ Enviaste tu primera transacción

- ✅ Entendiste los conceptos básicos

**Semana 1 - Clase 2:**

- ✅ Escribiste código JavaScript real

- ✅ Usaste la terminal como pro

- ✅ Deployaste un smart contract

**Próxima semana:**

- 🎯 Aprenderás Rust

- 🎯 Escribirás tu primer contrato

- 🎯 Construirás algo único

**¿Ves el patrón?**  

Cada clase te lleva más lejos.  

Cada día eres más capaz.  

Cada línea de código suma.

---

## 🌟 CIERRE

**Has llegado hasta aquí.**

Eso dice mucho de ti.

No solo leíste la clase.  

No solo asististe.  

Estás aquí, leyendo hasta el final, lista para hacer la tarea.

**Esa determinación es lo que separa a las que aprenden de las que construyen.**

**Tú estás construyendo.**

Sigue así, Tiburona. El futuro que estás creando para ti es increíble.

**Nos vemos el martes. Con Rust. Con más poder. Con más conocimiento.**

**Sigue nadando. Sigue construyendo.** 🦈⚡

---

## 📋 CHECKLIST DE ENTREGA

Antes de entregar, verifica que tengas:

- [ ] ✅ Ejercicio 1 completado (`ejercicio-1-creacion-masiva.js`)
- [ ] ✅ Ejercicio 2 completado (`ejercicio-2-pagos-automatizados.js`)
- [ ] ✅ Ejercicio 3 completado (`ejercicio-3-monitor-balances.js`)
- [ ] ✅ Todos los scripts ejecutados exitosamente
- [ ] ✅ Código comentado explicando decisiones
- [ ] ✅ Link de GitHub listo para compartir
- [ ] ✅ Reflexión personal escrita

---

**Documento anterior:** [05-tarea-y-proximos-pasos.md](./05-tarea-y-proximos-pasos.md)  

**Volver al índice:** [README.md](./README.md)

---

**Creado con ❤️ para las Tiburonas Builders**

