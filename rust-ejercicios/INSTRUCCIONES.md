# 📋 INSTRUCCIONES PARA EJECUTAR LOS EJERCICIOS

## 🔧 PREREQUISITOS

Antes de comenzar, asegurate de tener instalado:

1. **Rust** (1.70.0 o superior)
   ```bash
   rustc --version
   ```

2. **Cargo** (incluido con Rust)
   ```bash
   cargo --version
   ```

3. **Stellar CLI** (para deploy si querés)
   ```bash
   stellar --version
   ```

---

## 🚀 EJECUTAR EJERCICIOS

### 1. Compilar el proyecto

```bash
cd rust-ejercicios
cargo build
```

### 2. Ejecutar todos los tests

```bash
cargo test
```

### 3. Ejecutar tests con output completo

```bash
cargo test -- --nocapture
```

### 4. Ejecutar un test específico

```bash
# Ejemplo: solo el test de transferir
cargo test test_transferir_exitoso
```

---

## 📚 ESTRUCTURA DEL PROYECTO

```
rust-ejercicios/
├── Cargo.toml          # Configuración del proyecto
├── README.md           # Documentación general
├── INSTRUCCIONES.md    # Este archivo
├── src/
│   ├── lib.rs          # Código principal con todos los ejercicios
│   └── test.rs         # Tests para verificar ejercicios
└── target/             # Archivos compilados (generado por cargo)
```

---

## ✅ VERIFICACIÓN DE EJERCICIOS

### Ejercicio 5: contar_mayores

**Test:** `test_contar_mayores`

```bash
cargo test test_contar_mayores
```

**Verifica:**
- ✅ Cuenta correctamente números > 100
- ✅ Retorna 0 cuando no hay números > 100
- ✅ Maneja Vec vacío correctamente

---

### Ejercicio 6: validar_cantidad

**Tests:** `test_validar_cantidad_*`

```bash
cargo test test_validar_cantidad
```

**Verifica:**
- ✅ Acepta cantidades entre 1 y 1000
- ✅ Rechaza cantidad = 0
- ✅ Rechaza cantidad > 1000
- ✅ Retorna Result apropiado

---

### Ejercicio 7: procesar_token_info_eficiente

**Concepto:** Demuestra borrowing vs cloning

**Nota:** Este ejercicio es principalmente educativo. El código optimizado usa referencias en lugar de clones.

---

### Ejercicio 8: transferir

**Tests:** `test_transferir_*`

```bash
cargo test test_transferir
```

**Verifica:**
- ✅ Transfiere tokens correctamente
- ✅ Rechaza transferencias de 0 tokens
- ✅ Rechaza cuando balance insuficiente
- ✅ Actualiza balances correctamente
- ✅ Maneja múltiples transferencias
- ✅ Previene overflow/underflow con checked_*

---

## 🎯 EJERCICIOS TEÓRICOS (1-4)

Los ejercicios 1-4 son teóricos y están documentados en el documento principal:

1. **Ejercicio 1:** mut o no mut - Respuestas en el documento
2. **Ejercicio 2:** Predecir Overflow - Respuestas en el documento
3. **Ejercicio 3:** String vs Symbol - Respuestas en el documento
4. **Ejercicio 4:** Quiz de Ownership - Respuestas en el documento

---

## 🐛 TROUBLESHOOTING

### Error: "cannot find crate `soroban_sdk`"

**Solución:**
```bash
# Asegurate de estar en el directorio correcto
cd rust-ejercicios

# Reinstalar dependencias
cargo clean
cargo build
```

### Error: "error[E0308]: mismatched types"

**Solución:** Verificá que estás usando los tipos correctos de Soroban:
- `Vec` debe ser `SorobanVec` (o importado como `Vec`)
- `String` debe ser `SorobanString` (o importado como `String`)
- Address debe ser de `soroban_sdk::Address`

### Error en tests: "attempt to subtract with overflow"

**Solución:** Esto indica que necesitás usar `checked_sub` en lugar de `-`. Los ejercicios ya usan operaciones seguras, pero verificá tu código si estás modificando algo.

---

## 📖 PRÓXIMOS PASOS

Después de entender y ejecutar estos ejercicios:

1. **Leé el código fuente** (`src/lib.rs`) línea por línea
2. **Revisá los tests** (`src/test.rs`) para entender casos de uso
3. **Modificá los ejercicios** para experimentar
4. **Pasá a:** `03-CODIGO-EXPLICADO.md` para ver un contador completo

---

## 💡 TIPS

- **No tengas miedo de romper cosas:** Modificá el código y ve qué errores te da el compilador
- **Leé los mensajes de error:** El compilador de Rust es muy útil
- **Ejecutá los tests frecuentemente:** Te ayudan a verificar que todo funciona
- **Consultá el documento principal:** Volvé al "PASO A PASO - Rust Esencial" cuando tengas dudas

---

**¡Sigue construyendo, Tiburona!** 🦈⚡

