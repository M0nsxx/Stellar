# 💻 CÓDIGO EXPLICADO - Contador en Soroban

**Clase 3: Explicación Completa del Contador**

---

## 🎯 OBJETIVO

Este documento explica el contador completo bloque por bloque, línea por línea, para que entiendas cada decisión y te sientas segura modificándolo.

**Este contador demuestra TODOS los conceptos que aprendiste:**
- Tipos optimizados (u32, Symbol)
- Storage persistente en blockchain
- Borrowing en las funciones
- Option en lectura de storage
- Pattern matching en validaciones
- Eventos para transparencia

---

## 📚 ESTRUCTURA DEL ARCHIVO

El contador completo está en: **`src/contador.rs`**

**Archivo de tests:** Los tests están integrados en `src/contador.rs` con `#[cfg(test)]`

---

## 🔧 BLOQUE 1: SETUP INICIAL

```rust
#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    Env,
    symbol_short,
};
```

### Explicación línea por línea

**`#![no_std]`**
- Dice "no usar la biblioteca estándar de Rust"
- La std incluye threads, filesystem, networking
- Nada de eso existe en blockchain
- Soroban provee todo lo necesario

**`use soroban_sdk::{...}`**
- `contract`: Macro para marcar structs como contratos
- `contractimpl`: Macro para exponer métodos como funciones públicas
- `Env`: Ambiente - tu interfaz con la blockchain
- `symbol_short`: Para identificadores eficientes

---

## 🗂️ BLOQUE 2: DEFINICIÓN DEL CONTRATO

```rust
#[contract]
pub struct ContadorContract;
```

**`#[contract]`**: Marca este struct como un smart contract

**`pub struct ContadorContract;`**: El struct está vacío, solo sirve como contenedor

---

## ⚡ BLOQUE 3: FUNCIÓN INCREMENT

```rust
pub fn increment(env: Env) -> u32 {
    // PASO 1: Leer del storage
    let mut contador: u32 = env.storage()
        .instance()
        .get(&symbol_short!("COUNTER"))
        .unwrap_or(0);
    
    // PASO 2: Incrementar
    contador += 1;
    
    // PASO 3: Guardar en storage
    env.storage().instance().set(
        &symbol_short!("COUNTER"),
        &contador
    );
    
    // PASO 4: Emitir evento
    env.events().publish(
        (symbol_short!("increment"),),
        contador
    );
    
    // PASO 5: Retornar
    contador
}
```

### Explicación paso a paso

#### PASO 1: Leer del storage

```rust
let mut contador: u32 = env.storage()
    .instance()
    .get(&symbol_short!("COUNTER"))
    .unwrap_or(0);
```

**Desglose:**
1. `env.storage()` → Acceder al storage del contrato
2. `.instance()` → Usar storage de tipo "instance"
3. `.get(&symbol_short!("COUNTER"))` → Buscar la key "COUNTER"
   - Retorna `Option<u32>`
   - `Some(valor)` si existe
   - `None` si no existe
4. `.unwrap_or(0)` → Si es `None`, usar 0

**¿Por qué `mut`?** Necesitamos modificar el contador en el siguiente paso.

**¿Por qué `&symbol_short!("COUNTER")`?** El `&` crea una referencia (borrowing).

#### PASO 2: Incrementar

```rust
contador += 1;
```

Sintaxis de Rust para `contador = contador + 1`

#### PASO 3: Guardar en storage

```rust
env.storage().instance().set(
    &symbol_short!("COUNTER"),
    &contador
);
```

**¿Por qué referencias?** `set()` toma referencias para no consumir los valores.

#### PASO 4: Emitir evento

```rust
env.events().publish(
    (symbol_short!("increment"),),
    contador
);
```

**Topics vs Data:**
- Primer argumento: Topics (tupla de Symbols) para filtrar
- Segundo argumento: Data del evento (el nuevo valor)

#### PASO 5: Retornar

```rust
contador
```

Sin punto y coma = retorno implícito

---

## ⬇️ BLOQUE 4: FUNCIÓN DECREMENT

```rust
pub fn decrement(env: Env) -> u32 {
    let mut contador: u32 = env.storage()
        .instance()
        .get(&symbol_short!("COUNTER"))
        .unwrap_or(0);
    
    // VALIDACIÓN CRÍTICA
    if contador == 0 {
        panic!("No se puede decrementar: contador ya está en 0");
    }
    
    contador -= 1;
    
    env.storage().instance().set(
        &symbol_short!("COUNTER"),
        &contador
    );
    
    env.events().publish(
        (symbol_short!("decrement"),),
        contador
    );
    
    contador
}
```

### La validación

```rust
if contador == 0 {
    panic!("No se puede decrementar: contador ya está en 0");
}
```

**¿Por qué esta validación?**
1. **Prevenir underflow:** u32 no puede ser negativo
2. **`panic!` en smart contracts:** Detiene ejecución, revierte transacción
3. **Validar ANTES de modificar:** Principio "fail fast"

---

## 👁️ BLOQUE 5: FUNCIÓN GET_COUNT

```rust
pub fn get_count(env: Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("COUNTER"))
        .unwrap_or(0)
}
```

**Características:**
- No necesita `mut` (solo lectura)
- Más barata en gas (solo lectura)
- Retorno directo sin variable intermedia

---

## 🔄 BLOQUE 6: FUNCIÓN RESET

```rust
pub fn reset(env: Env) {
    env.storage().instance().set(
        &symbol_short!("COUNTER"),
        &0u32
    );
    
    env.events().publish(
        (symbol_short!("reset"),),
        0u32
    );
}
```

**Sin tipo de retorno:** Operación de efecto lateral, no retorna valor

**`&0u32`:** Referencia directa al literal 0

---

## 🧪 BLOQUE 7: TESTS

Los tests están en el mismo archivo con `#[cfg(test)]`:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{...};
    
    #[test]
    fn test_increment() {
        // Setup
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);
        let client = ContadorContractClient::new(&env, &contract_id);
        
        // Verificaciones
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.get_count(), 2);
    }
    
    // ... más tests ...
}
```

**14 tests implementados** cubriendo todas las funciones y casos edge.

---

## 📊 FUNCIONES IMPLEMENTADAS

### Funciones Básicas (4)
1. ✅ `increment()` - Incrementa en 1
2. ✅ `decrement()` - Decrementa en 1 (con validación)
3. ✅ `get_count()` - Obtiene valor actual
4. ✅ `reset()` - Resetea a 0

### Ejercicios Guiados (3)
5. ✅ `increment_by(amount)` - Incrementa por cantidad
6. ✅ `increment_con_limite()` - Incrementa con límite máximo 1000
7. ✅ `decrement_by(amount)` - Decrementa por cantidad

**Total: 7 funciones públicas** ✅

---

## 🚀 CÓMO COMPILAR Y EJECUTAR

```bash
cd rust-ejercicios

# Compilar
cargo build

# Ejecutar todos los tests
cargo test contador

# Test específico
cargo test test_increment

# Con output detallado
cargo test contador -- --nocapture
```

---

## 💡 CONCEPTOS APLICADOS

Cada función demuestra:
- ✅ **Mutabilidad:** `mut` cuando se modifica
- ✅ **Tipos:** u32 para contador, Symbol para keys
- ✅ **Borrowing:** Referencias `&` en storage
- ✅ **Option:** `unwrap_or(0)` para valores por defecto
- ✅ **Validaciones:** `if` y `panic!` antes de modificar
- ✅ **Storage:** `get()` y `set()` para persistencia
- ✅ **Eventos:** `publish()` para transparencia
- ✅ **Operaciones seguras:** `checked_add` y `checked_sub`

---

## 📚 PATRONES UNIVERSALES

### Patrón 1: Leer-Modificar-Guardar

```
Leer → Modificar → Guardar → Emitir → Retornar
```

**Usado en:** `increment()`, `decrement()`, `increment_by()`, `decrement_by()`

### Patrón 2: Validar-Ejecutar-Emitir

```
Validar → Ejecutar → Emitir
```

**Usado en:** Todas las funciones que modifican estado

### Patrón 3: Solo Lectura

```
Leer → Retornar
```

**Usado en:** `get_count()`

---

## 🎯 EJERCICIOS GUIADOS IMPLEMENTADOS

### Ejercicio 1: `increment_by(amount)`
✅ Implementado con validación de overflow usando `checked_add`

### Ejercicio 2: `increment_con_limite()`
✅ Implementado con límite máximo de 1000

### Ejercicio 4: `decrement_by(amount)`
✅ Implementado con validación de underflow usando `checked_sub`

---

## ✅ CHECKLIST DE COMPLETITUD

### Código del Contador
- [x] ✅ Setup inicial (`#![no_std]`, imports)
- [x] ✅ Definición del contrato (`#[contract]`, `struct`)
- [x] ✅ Función `increment()` con todos los pasos
- [x] ✅ Función `decrement()` con validación
- [x] ✅ Función `get_count()` (solo lectura)
- [x] ✅ Función `reset()`
- [x] ✅ Ejercicios guiados implementados (3 funciones)
- [x] ✅ Todos los comentarios explicativos
- [x] ✅ Documentación rustdoc completa

### Tests
- [x] ✅ Test básico de increment
- [x] ✅ Test básico de decrement
- [x] ✅ Test de panic en decrement
- [x] ✅ Test de reset
- [x] ✅ Test de get_count
- [x] ✅ Test de increment_by
- [x] ✅ Test de overflow en increment_by
- [x] ✅ Test de increment_con_limite
- [x] ✅ Test de panic en límite
- [x] ✅ Test de decrement_by
- [x] ✅ Test de underflow en decrement_by
- [x] ✅ Test de flujo completo
- [x] ✅ Test de múltiples incrementos
- [x] ✅ Test de alternar increment/decrement

**Total: 14 tests implementados** ✅

### Documentación
- [x] ✅ Comentarios explicando cada bloque
- [x] ✅ Documentación rustdoc para cada función
- [x] ✅ Ejemplos de uso en documentación
- [x] ✅ README del contador (CONTADOR-README.md)
- [x] ✅ Este documento (CODIGO-EXPLICADO.md)

---

## 🎓 VERIFICACIÓN FINAL

**Todas las funciones del documento están implementadas:**
- ✅ `increment()` - Completo con todos los pasos
- ✅ `decrement()` - Completo con validación
- ✅ `get_count()` - Completo (solo lectura)
- ✅ `reset()` - Completo

**Todos los ejercicios guiados están implementados:**
- ✅ `increment_by()` - Con validación overflow
- ✅ `increment_con_limite()` - Con límite 1000
- ✅ `decrement_by()` - Con validación underflow

**Todos los conceptos están demostrados:**
- ✅ Tipos optimizados
- ✅ Storage persistente
- ✅ Borrowing
- ✅ Option
- ✅ Pattern matching
- ✅ Eventos
- ✅ Validaciones
- ✅ Operaciones seguras

---

## 🦈 ESTADO: 100% COMPLETO

**El contador está completamente implementado según el documento "CÓDIGO EXPLICADO - Contador en Soroban".**

**7 funciones + 14 tests + documentación completa = Listo para usar** ✅

---

**Creado con ❤️ para las Tiburonas Builders** 🦈⚡

