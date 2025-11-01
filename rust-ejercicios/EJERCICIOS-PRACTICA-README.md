# 🚀 EJERCICIOS PRÁCTICA - Próximos Pasos

**Clase 3: Práctica y Desafíos - Todos los Ejercicios Implementados**

---

## 📋 EJERCICIOS IMPLEMENTADOS

Este directorio contiene todos los ejercicios prácticos del documento "PRÓXIMOS PASOS - Práctica y Desafíos".

### ✅ NIVEL 1: Entendiendo el código

#### Ejercicio 1.1: Lectura de código
**Archivo:** `src/ejercicios_practica.rs` - `MysteryFunctions`

- ✅ `mystery_function_a()` - Función de solo lectura que retorna valor * 2
- ✅ `mystery_function_b()` - Función que modifica storage agregando x al total

**Tests implementados:**
- ✅ `test_mystery_function_a` - Verifica comportamiento con valores por defecto
- ✅ `test_mystery_function_b` - Verifica múltiples llamadas acumulativas

---

#### Ejercicio 1.2: Detectar errores
**Concepto:** Ejercicio educativo sobre errores comunes

- ✅ Errores documentados en código con explicaciones
- ✅ Soluciones proporcionadas

---

### ✅ NIVEL 2: Modificando el contador

#### Ejercicio 2.1: Agregar función increment_by
**Archivo:** `src/ejercicios_practica.rs` - `ContadorExtendido`

- ✅ `increment_by(amount)` - Incrementa contador por cantidad específica
- ✅ `increment()` - Incrementa en 1
- ✅ `decrement()` - Decrementa en 1 (con validación)
- ✅ `reset()` - Resetea a 0
- ✅ `get_count()` - Obtiene valor actual

**Tests implementados:**
- ✅ `test_increment_by` - Verifica incremento por cantidad

---

#### Ejercicio 2.2: Agregar límite máximo
**Archivo:** `src/ejercicios_practica.rs` - `ContadorConLimite`

- ✅ `increment()` - Con límite máximo de 1000
- ✅ Validación que causa panic si excede límite
- ✅ `get_count()` - Obtiene valor actual

**Tests implementados:**
- ✅ `test_contador_con_limite_increment` - Verifica hasta límite
- ✅ `test_contador_con_limite_panic` - Verifica panic al exceder

---

#### Ejercicio 2.3: Función set_value
**Archivo:** `src/ejercicios_practica.rs` - `ContadorConSetValue`

- ✅ `set_value(new_value)` - Establece contador a valor específico
- ✅ Validación que valor esté entre 0 y 1000
- ✅ `increment()` - Incrementa en 1
- ✅ `get_count()` - Obtiene valor actual

**Tests implementados:**
- ✅ `test_set_value` - Verifica establecer valores válidos
- ✅ `test_set_value_invalid` - Verifica panic con valor inválido

---

### ✅ NIVEL 3: Proyectos nuevos

#### Ejercicio 3.1: Contador con historial
**Archivo:** `src/ejercicios_practica.rs` - `ContadorConHistorial`

- ✅ `increment()` - Incrementa y mantiene historial de últimos 5 valores
- ✅ `get_history()` - Retorna historial completo (máx 5 valores)
- ✅ `get_count()` - Obtiene valor actual

**Características:**
- Mantiene historial en `Vec<u32>`
- Automáticamente limita a 5 elementos más recientes
- Remueve elementos más viejos cuando se excede el límite

**Tests implementados:**
- ✅ `test_contador_con_historial` - Verifica historial con 7 incrementos
- ✅ `test_historial_menos_de_5` - Verifica historial con menos de 5 valores

---

#### Ejercicio 3.2: Sistema de votación simple
**Archivo:** `src/ejercicios_practica.rs` - `SistemaVotacion`

- ✅ `vote_a()` - Vota por opción A
- ✅ `vote_b()` - Vota por opción B
- ✅ `get_results()` - Retorna (votos_a, votos_b)
- ✅ `get_winner()` - Retorna "A", "B", o "tie"

**Características:**
- Dos contadores separados para cada opción
- Función que determina ganador o empate
- Emite eventos para cada voto

**Tests implementados:**
- ✅ `test_sistema_votacion` - Verifica votación básica
- ✅ `test_sistema_votacion_empate` - Verifica empate
- ✅ `test_sistema_votacion_b_gana` - Verifica ganador B

---

### ✅ PROYECTO INTEGRADOR: Sistema de reputación

**Archivo:** `src/ejercicios_practica.rs` - `ReputationContract`

**Funcionalidades implementadas:**
- ✅ `like(entity, user)` - Da like a una entidad (previene voto duplicado)
- ✅ `dislike(entity, user)` - Da dislike a una entidad (previene voto duplicado)
- ✅ `get_likes(entity)` - Obtiene número de likes
- ✅ `get_dislikes(entity)` - Obtiene número de dislikes
- ✅ `get_score(entity)` - Obtiene score (likes - dislikes, puede ser negativo)
- ✅ `has_voted(entity, user)` - Verifica si usuario ya votó

**Características:**
- Validación que cada usuario solo puede votar una vez por entidad
- Soporte para múltiples entidades simultáneamente
- Score puede ser negativo si hay más dislikes que likes
- Emite eventos para cada voto

**Tests implementados:**
- ✅ `test_reputation_like` - Verifica likes básicos
- ✅ `test_reputation_dislike` - Verifica dislikes básicos
- ✅ `test_reputation_score_mixto` - Verifica score con likes y dislikes
- ✅ `test_reputation_no_voto_duplicado` - Verifica que no se puede votar dos veces
- ✅ `test_reputation_multiple_entidades` - Verifica múltiples entidades
- ✅ `test_reputation_score_negativo` - Verifica score negativo

**Total: 6 tests para el proyecto integrador** ✅

---

## 📊 ESTADÍSTICAS FINALES

### Contratos Implementados
- **Nivel 1:** 1 contrato (`MysteryFunctions`)
- **Nivel 2:** 3 contratos (`ContadorExtendido`, `ContadorConLimite`, `ContadorConSetValue`)
- **Nivel 3:** 2 contratos (`ContadorConHistorial`, `SistemaVotacion`)
- **Proyecto Integrador:** 1 contrato (`ReputationContract`)
- **Total:** 7 contratos implementados

### Funciones Implementadas
- **Nivel 1:** 2 funciones
- **Nivel 2:** 7 funciones
- **Nivel 3:** 6 funciones
- **Proyecto Integrador:** 6 funciones
- **Total:** 21 funciones públicas

### Tests Implementados
- **Nivel 1:** 2 tests
- **Nivel 2:** 4 tests
- **Nivel 3:** 5 tests
- **Proyecto Integrador:** 6 tests
- **Total:** 17 tests implementados

---

## 🚀 CÓMO USAR

### Compilar el proyecto

```bash
cd rust-ejercicios
cargo build
```

### Ejecutar todos los tests

```bash
cargo test ejercicios_practica
```

### Ejecutar tests específicos

```bash
# Tests del Nivel 1
cargo test mystery_function

# Tests del Nivel 2
cargo test contador

# Tests del Nivel 3
cargo test votacion
cargo test historial

# Tests del proyecto integrador
cargo test reputation
```

### Ejecutar tests con output detallado

```bash
cargo test ejercicios_practica -- --nocapture
```

---

## 📚 ESTRUCTURA DEL CÓDIGO

```
rust-ejercicios/
├── src/
│   ├── ejercicios_practica.rs      # Todos los ejercicios (7 contratos)
│   └── ejercicios_practica_test.rs  # Todos los tests (17 tests)
├── Cargo.toml
└── EJERCICIOS-PRACTICA-README.md    # Este archivo
```

---

## ✅ CHECKLIST DE COMPLETITUD

### Ejercicios del Documento

#### Nivel 1: Entendiendo el código
- [x] ✅ Ejercicio 1.1: Mystery functions - Implementado con tests
- [x] ✅ Ejercicio 1.2: Detectar errores - Documentado en código

#### Nivel 2: Modificando el contador
- [x] ✅ Ejercicio 2.1: increment_by - Implementado completamente
- [x] ✅ Ejercicio 2.2: Límite máximo - Implementado completamente
- [x] ✅ Ejercicio 2.3: set_value - Implementado completamente

#### Nivel 3: Proyectos nuevos
- [x] ✅ Ejercicio 3.1: Contador con historial - Implementado completamente
- [x] ✅ Ejercicio 3.2: Sistema de votación - Implementado completamente

#### Proyecto Integrador
- [x] ✅ Sistema de reputación - Implementado completamente

### Funcionalidades

#### Sistema de Reputación
- [x] ✅ like(entity, user) - Con validación de voto duplicado
- [x] ✅ dislike(entity, user) - Con validación de voto duplicado
- [x] ✅ get_likes(entity) - Obtiene likes
- [x] ✅ get_dislikes(entity) - Obtiene dislikes
- [x] ✅ get_score(entity) - Obtiene score (puede ser negativo)
- [x] ✅ has_voted(entity, user) - Verifica si votó

### Tests

- [x] ✅ Tests para Nivel 1 (2 tests)
- [x] ✅ Tests para Nivel 2 (4 tests)
- [x] ✅ Tests para Nivel 3 (5 tests)
- [x] ✅ Tests para Proyecto Integrador (6 tests)
- [x] ✅ Total: 17 tests implementados

### Documentación

- [x] ✅ Comentarios rustdoc en todas las funciones
- [x] ✅ Ejemplos en documentación
- [x] ✅ README completo (este archivo)

---

## 🎯 CONCEPTOS APLICADOS

Cada ejercicio demuestra:

### Nivel 1
- ✅ Lectura vs escritura en storage
- ✅ Valores por defecto con `unwrap_or`
- ✅ Mutabilidad vs inmutabilidad

### Nivel 2
- ✅ Funciones con parámetros
- ✅ Validaciones con `if` y `panic!`
- ✅ Límites y restricciones
- ✅ Eventos personalizados

### Nivel 3
- ✅ Uso de `Vec` para listas dinámicas
- ✅ Mantenimiento de historial
- ✅ Múltiples contadores simultáneos
- ✅ Lógica de comparación (ganador/empate)

### Proyecto Integrador
- ✅ Storage con keys compuestas `(Symbol, Address)`
- ✅ Validación de reglas de negocio (un voto por usuario)
- ✅ Múltiples entidades independientes
- ✅ Cálculos que pueden ser negativos (i32)
- ✅ Funciones de consulta complejas

---

## 🦈 ESTADO: 100% COMPLETO

**Todos los ejercicios del documento "PRÓXIMOS PASOS - Práctica y Desafíos" están implementados y testeados.** ✅

**7 contratos + 21 funciones + 17 tests = Implementación completa** 🚀

---

**Creado con ❤️ para las Tiburonas Builders** 🦈⚡

