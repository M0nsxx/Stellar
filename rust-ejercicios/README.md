# 🦈 EJERCICIOS RUST PARA SOROBAN

**Clase 3: Rust Esencial para Soroban - Ejercicios Prácticos**

---

## 📋 EJERCICIOS IMPLEMENTADOS

Este directorio contiene todos los ejercicios prácticos del documento "PASO A PASO - Rust Esencial para Soroban".

### ✅ Ejercicios Teóricos (1-4)
- Ejercicio 1: mut o no mut
- Ejercicio 2: Predecir Overflow
- Ejercicio 3: String vs Symbol
- Ejercicio 4: Quiz de Ownership

*Las respuestas están documentadas en el archivo principal del documento.*

### ✅ Ejercicios Prácticos (5-8)

#### Ejercicio 5: Función con Vec
**Archivo:** `src/lib.rs` - Función `contar_mayores`
- Cuenta números mayores a 100 en un Vec
- Demuestra iteración y mutabilidad

#### Ejercicio 6: Validar cantidad
**Archivo:** `src/lib.rs` - Función `validar_cantidad`
- Validación con Result<T, E>
- Manejo de errores descriptivos
- Pattern matching con Result

#### Ejercicio 7: Corregir código ineficiente
**Archivo:** `src/lib.rs` - Función `procesar_bien`
- Demuestra borrowing vs cloning
- Optimización de memoria
- Uso correcto de referencias

#### Ejercicio 8: Transferencia de Tokens (DESAFÍO)
**Archivo:** `src/lib.rs` - Función `transferir`
- Implementación completa de transferencia
- Validaciones múltiples
- Uso de `checked_sub` y `checked_add`
- Storage persistente
- Eventos
- Manejo completo de errores

---

## 🚀 CÓMO USAR

### Compilar y Ejecutar

```bash
# Desde el directorio rust-ejercicios
cargo build

# Ejecutar tests
cargo test

# Ejecutar tests con output
cargo test -- --nocapture
```

### Ejecutar Funciones Específicas

Las funciones están implementadas en `src/lib.rs` y pueden ser llamadas desde tests o desde un contrato real de Soroban.

---

## 📚 CONCEPTOS APLICADOS

Cada ejercicio demuestra conceptos clave:

- ✅ Mutabilidad (`mut`)
- ✅ Tipos de datos (u32, u128, Vec, String)
- ✅ Ownership y Borrowing
- ✅ Pattern Matching (match)
- ✅ Option y Result
- ✅ Storage persistente (Soroban)
- ✅ Eventos (Soroban)
- ✅ Operaciones seguras (checked_*)

---

## 🎯 SIGUIENTE PASO

Después de entender estos ejercicios, pasá a:

**📄 `03-CODIGO-EXPLICADO.md`** - Cómo todos estos conceptos se unen en un contador completo.

---

**Creado con ❤️ para las Tiburonas Builders** 🦈⚡

