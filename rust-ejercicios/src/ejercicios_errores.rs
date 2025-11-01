#![no_std]

/// Ejercicio 1.2: Detectar errores
/// 
/// Este archivo contiene ejemplos de código con errores comunes
/// para que puedas identificar y corregir problemas.
/// 
/// NOTA: Este código NO compila intencionalmente.
/// Es solo para fines educativos.

// ============================================================
// ERROR 1: Falta `mut` en variable que se modifica
// ============================================================

/*
// ❌ CÓDIGO CON ERROR:
pub fn bad_function_1(env: Env) {
    let contador = 0;  // ❌ Falta `mut`
    contador += 1;     // ❌ No puede modificar variable inmutable
    env.storage().instance().set(&symbol_short!("COUNT"), &contador);
}

// ✅ SOLUCIÓN CORRECTA:
pub fn good_function_1(env: Env) {
    let mut contador = 0;  // ✅ Con `mut`
    contador += 1;         // ✅ Ahora puede modificar
    env.storage().instance().set(&symbol_short!("COUNT"), &contador);
}
*/

// ============================================================
// ERROR 2: Usar variable después de move
// ============================================================

/*
// ❌ CÓDIGO CON ERROR:
pub fn bad_function_2(env: Env) -> String {
    let s = String::from_str(&env, "hola");
    let s2 = s;  // ❌ MOVE: s ya no es válido
    s            // ❌ ERROR: intentando usar s después del move
}

// ✅ SOLUCIÓN CORRECTA:
// Opción 1: Retornar s2
pub fn good_function_2a(env: Env) -> SorobanString {
    let s = SorobanString::from_str(&env, "hola");
    let s2 = s;  // MOVE: s ya no es válido
    s2           // ✅ Retornar s2 en lugar de s
}

// Opción 2: Usar borrowing (si solo necesitas leer)
pub fn good_function_2b(env: Env) -> usize {
    let s = SorobanString::from_str(&env, "hola");
    let len = s.len();  // ✅ Solo lectura, no necesita move
    len
}
*/

// ============================================================
// ERROR 3: Múltiples referencias mutables
// ============================================================

/*
// ❌ CÓDIGO CON ERROR:
pub fn bad_function_3(env: Env) {
    let mut x = SorobanString::from_str(&env, "test");
    let r1 = &mut x;
    let r2 = &mut x;  // ❌ ERROR: segunda referencia mutable
    // Rust no permite esto para prevenir data races
}

// ✅ SOLUCIÓN CORRECTA:
pub fn good_function_3(env: Env) {
    let mut x = SorobanString::from_str(&env, "test");
    {
        let r1 = &mut x;
        // Usar r1 aquí
    }  // r1 sale de scope
    let r2 = &mut x;  // ✅ Ahora OK, r1 ya no existe
}
*/

// ============================================================
// EXPLICACIONES DE LOS ERRORES
// ============================================================

/// ERROR 1: Falta `mut`
/// 
/// **Problema:**
/// En Rust, las variables son inmutables por defecto. Si intentas
/// modificar una variable sin `mut`, el compilador te dará un error.
/// 
/// **Solución:**
/// Agrega `mut` cuando necesites modificar la variable:
/// ```rust
/// let mut contador = 0;  // Con mut
/// contador += 1;          // Ahora funciona
/// ```
/// 
/// **Cuándo usar `mut`:**
/// - Cuando necesites modificar el valor de una variable
/// - Cuando uses operaciones como `+=`, `-=`, `push_back()`, etc.
/// 
/// **Cuándo NO usar `mut`:**
/// - Cuando solo leas el valor
/// - Cuando el valor no cambie durante la función

/// ERROR 2: Usar variable después de move
/// 
/// **Problema:**
/// Cuando asignas un valor que se "mueve" (como String, Vec) a otra
/// variable, el valor original ya no es válido.
/// 
/// **Solución:**
/// - Opción 1: Usa la nueva variable en lugar de la original
/// - Opción 2: Usa borrowing (`&`) si solo necesitas leer
/// - Opción 3: Clona si realmente necesitas una copia independiente
/// 
/// **Tipos que se mueven:**
/// - String, Vec, structs complejos
/// 
/// **Tipos que se copian:**
/// - Números (u32, i32, etc.)
/// - Booleanos
/// - Caracteres

/// ERROR 3: Múltiples referencias mutables
/// 
/// **Problema:**
/// Rust solo permite UNA referencia mutable a la vez para prevenir
/// data races (modificación simultánea del mismo dato).
/// 
/// **Solución:**
/// - Usa la primera referencia y déjala salir de scope antes de crear otra
/// - Usa referencias inmutables (`&`) si solo necesitas leer (puedes tener múltiples)
/// - Diseña tu código para no necesitar múltiples referencias mutables simultáneas
/// 
/// **Reglas de borrowing:**
/// - Puedes tener INFINITAS referencias inmutables (`&T`) simultáneas
/// - Solo puedes tener UNA referencia mutable (`&mut T`) a la vez
/// - No puedes mezclar referencias inmutables y mutables simultáneamente

