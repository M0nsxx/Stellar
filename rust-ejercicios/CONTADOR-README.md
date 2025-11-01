# 📊 CONTADOR COMPLETO EN SOROBAN

**Clase 3: Código Explicado - Contador en Soroban**

---

## 🎯 OBJETIVO

Este es el contador completo que demuestra **TODOS** los conceptos fundamentales de Rust para Soroban:

- ✅ Tipos optimizados (u32, Symbol)
- ✅ Storage persistente en blockchain
- ✅ Borrowing en las funciones
- ✅ Option en lectura de storage
- ✅ Pattern matching en validaciones
- ✅ Eventos para transparencia

---

## 📋 FUNCIONES IMPLEMENTADAS

### Funciones Básicas

#### 1. `increment()` - Incrementar contador

Incrementa el contador en 1.

**Flujo:**
1. Leer del storage (Option -> unwrap_or(0))
2. Incrementar el valor
3. Guardar en storage
4. Emitir evento
5. Retornar nuevo valor

**Ejemplo:**
```rust
let client = ContadorContractClient::new(&env, &contract_id);
assert_eq!(client.increment(), 1);
assert_eq!(client.increment(), 2);
```

---

#### 2. `decrement()` - Decrementar contador

Decrementa el contador en 1 con validación de underflow.

**Validación:**
- Si contador == 0: panic! (no se puede decrementar)
- Previene underflow (u32 no puede ser negativo)

**Ejemplo:**
```rust
client.increment();
client.increment();
assert_eq!(client.decrement(), 1);
```

---

#### 3. `get_count()` - Obtener valor actual

Función de solo lectura para consultar el contador.

**Características:**
- No modifica el estado
- No necesita `mut`
- Más barata en gas (solo lectura)

**Ejemplo:**
```rust
assert_eq!(client.get_count(), 0);
client.increment();
assert_eq!(client.get_count(), 1);
```

---

#### 4. `reset()` - Resetear a 0

Resetea el contador a 0 y emite evento.

**Ejemplo:**
```rust
client.increment();
client.increment();
client.reset();
assert_eq!(client.get_count(), 0);
```

---

### Ejercicios Guiados Adicionales

#### 5. `increment_by(amount)` - Incrementar por cantidad

Incrementa el contador por una cantidad específica.

**Validación:**
- Usa `checked_add` para prevenir overflow
- Si hay overflow, panic con mensaje descriptivo

**Ejemplo:**
```rust
assert_eq!(client.increment_by(5), 5);
assert_eq!(client.increment_by(3), 8);
```

---

#### 6. `increment_con_limite()` - Incrementar con límite máximo

Versión modificada de `increment()` que incluye límite máximo de 1000.

**Validación:**
- Si contador >= 1000: panic!
- Previene que el contador exceda el límite

**Ejemplo:**
```rust
// Incrementar hasta 999
for _ in 0..999 {
    client.increment_con_limite();
}
assert_eq!(client.get_count(), 999);
```

---

#### 7. `decrement_by(amount)` - Decrementar por cantidad

Decrementa el contador por una cantidad específica.

**Validación:**
- Verifica que contador >= amount antes de restar
- Usa `checked_sub` para prevenir underflow

**Ejemplo:**
```rust
client.increment_by(10);
assert_eq!(client.decrement_by(3), 7);
assert_eq!(client.decrement_by(2), 5);
```

---

## 🚀 CÓMO USAR

### Compilar el Contador

```bash
cd rust-ejercicios
cargo build
```

### Ejecutar Tests

```bash
# Todos los tests del contador
cargo test contador

# Test específico
cargo test test_increment

# Con output detallado
cargo test contador -- --nocapture
```

---

## 📊 TESTS IMPLEMENTADOS

### Tests Básicos

- ✅ `test_increment` - Verifica incrementar funciona
- ✅ `test_decrement` - Verifica decrementar funciona
- ✅ `test_decrement_panic` - Verifica panic al decrementar desde 0
- ✅ `test_reset` - Verifica reset funciona
- ✅ `test_get_count` - Verifica lectura funciona

### Tests de Ejercicios Adicionales

- ✅ `test_increment_by` - Verifica increment_by funciona
- ✅ `test_increment_by_overflow` - Verifica panic en overflow
- ✅ `test_increment_con_limite` - Verifica límite funciona
- ✅ `test_increment_con_limite_panic` - Verifica panic al exceder límite
- ✅ `test_decrement_by` - Verifica decrement_by funciona
- ✅ `test_decrement_by_insuficiente` - Verifica panic si no hay suficiente

### Tests de Integración

- ✅ `test_flujo_completo` - Flujo completo de uso
- ✅ `test_multiple_increments` - Múltiples incrementos seguidos
- ✅ `test_alternar_increment_decrement` - Alternar increment/decrement

**Total: 14 tests implementados** ✅

---

## 💡 CONCEPTOS DEMOSTRADOS

### 1. Tipos de Datos
- **u32**: Tipo eficiente para contadores
- **Symbol**: Identificadores optimizados para keys y eventos

### 2. Mutabilidad
- **`mut`**: Variables que se modifican
- **Inmutabilidad por defecto**: Funciones de solo lectura

### 3. Borrowing
- **Referencias (`&`)**: En storage operations
- **No copiamos valores innecesariamente**

### 4. Option
- **`get()` retorna `Option<u32>`**
- **`unwrap_or(0)`**: Valor por defecto

### 5. Pattern Matching
- **En `unwrap_or()`**: Maneja Some/None
- **En validaciones con `if`**

### 6. Storage Persistente
- **`env.storage().instance()`**: Datos persistentes
- **`get()`**: Leer, `set()`: Escribir

### 7. Eventos
- **`env.events().publish()`**: Transparencia
- **Topics con Symbol, data con valores**

### 8. Operaciones Seguras
- **`checked_add()`**: Prevenir overflow
- **`checked_sub()`**: Prevenir underflow

### 9. Tests
- **Estructura AAA**: Arrange, Act, Assert
- **Tests positivos y negativos**
- **`should_panic`**: Para validaciones

---

## 🔍 ESTRUCTURA DEL CÓDIGO

```
rust-ejercicios/
├── src/
│   ├── contador.rs          # Contador completo (7 funciones)
│   └── contador_test.rs     # Tests completos (14 tests)
└── Cargo.toml
```

---

## 📚 PATRONES APLICADOS

### Patrón 1: Leer-Modificar-Guardar

```rust
let mut valor = storage.get(key).unwrap_or(default);  // Leer
valor += 1;                                            // Modificar
storage.set(key, &valor);                              // Guardar
```

**Usado en:** `increment()`, `decrement()`, `increment_by()`, `decrement_by()`

---

### Patrón 2: Validar-Ejecutar-Emitir

```rust
if !es_valido { panic!("Error"); }  // Validar
ejecutar_logica();                   // Ejecutar
env.events().publish(...);           // Emitir
```

**Usado en:** `decrement()`, `increment_by()`, `decrement_by()`, `increment_con_limite()`

---

### Patrón 3: Solo Lectura sin mut

```rust
pub fn get_valor(env: Env) -> u32 {
    storage.get(key).unwrap_or(0)  // Solo lectura
}
```

**Usado en:** `get_count()`

---

## ✅ CHECKLIST DE COMPLETITUD

### Funciones Básicas
- [x] ✅ `increment()` - Implementada con todos los pasos
- [x] ✅ `decrement()` - Implementada con validación
- [x] ✅ `get_count()` - Implementada (solo lectura)
- [x] ✅ `reset()` - Implementada

### Ejercicios Guiados
- [x] ✅ `increment_by()` - Implementada con validación overflow
- [x] ✅ `increment_con_limite()` - Implementada con límite 1000
- [x] ✅ `decrement_by()` - Implementada con validación underflow

### Tests
- [x] ✅ Tests básicos (5 tests)
- [x] ✅ Tests de ejercicios adicionales (6 tests)
- [x] ✅ Tests de integración (3 tests)
- [x] ✅ Total: 14 tests implementados

### Documentación
- [x] ✅ Comentarios rustdoc en todas las funciones
- [x] ✅ Ejemplos en documentación
- [x] ✅ Explicación de patrones
- [x] ✅ README completo

---

## 🎯 PRÓXIMOS PASOS

Después de entender el contador:

1. **Modificá el código:** Cambiá límites, agregá validaciones
2. **Experimentá:** Probá cambiar u32 por u8 y observá las diferencias
3. **Creá variantes:** Contador con múltiples variables, con timestamps, etc.

---

**El contador es la base de TODO en Soroban. Dominalo y dominarás Rust para blockchain.** 🦈⚡

---

**Creado con ❤️ para las Tiburonas Builders**

