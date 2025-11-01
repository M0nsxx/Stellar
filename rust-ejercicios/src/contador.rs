#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    Env,
    Symbol,
    symbol_short,
};

/// Contador completo en Soroban
/// 
/// Demuestra todos los conceptos fundamentales:
/// - Tipos optimizados (u32, Symbol)
/// - Storage persistente en blockchain
/// - Borrowing en las funciones
/// - Option en lectura de storage
/// - Pattern matching en validaciones
/// - Eventos para transparencia
#[contract]
pub struct ContadorContract;

/// Implementación del contrato contador
#[contractimpl]
impl ContadorContract {
    /// Incrementa el contador en 1
    /// 
    /// # Flujo completo:
    /// 1. Leer del storage (Option -> unwrap_or(0))
    /// 2. Incrementar el valor
    /// 3. Guardar en storage
    /// 4. Emitir evento
    /// 5. Retornar nuevo valor
    /// 
    /// # Retorna
    /// El nuevo valor del contador después de incrementar
    /// 
    /// # Ejemplo
    /// ```
    /// let contador = ContadorContractClient::new(&env, &contract_id);
    /// assert_eq!(contador.increment(), 1);
    /// assert_eq!(contador.increment(), 2);
    /// ```
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

    /// Decrementa el contador en 1 (con validación de underflow)
    /// 
    /// # Validación:
    /// - Si contador == 0: panic! (no se puede decrementar)
    /// - Previene underflow (u32 no puede ser negativo)
    /// 
    /// # Retorna
    /// El nuevo valor del contador después de decrementar
    /// 
    /// # Panic
    /// Si intentas decrementar cuando el contador ya está en 0
    /// 
    /// # Ejemplo
    /// ```
    /// let contador = ContadorContractClient::new(&env, &contract_id);
    /// contador.increment();
    /// contador.increment();
    /// assert_eq!(contador.decrement(), 1);
    /// ```
    pub fn decrement(env: Env) -> u32 {
        // PASO 1: Leer del storage
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0);

        // PASO 2: VALIDACIÓN CRÍTICA - Prevenir underflow
        if contador == 0 {
            panic!("No se puede decrementar: contador ya está en 0");
        }

        // PASO 3: Decrementar
        contador -= 1;

        // PASO 4: Guardar en storage
        env.storage().instance().set(
            &symbol_short!("COUNTER"),
            &contador
        );

        // PASO 5: Emitir evento
        env.events().publish(
            (symbol_short!("decrement"),),
            contador
        );

        // PASO 6: Retornar
        contador
    }

    /// Obtiene el valor actual del contador (solo lectura)
    /// 
    /// # Características:
    /// - No modifica el estado
    /// - No necesita `mut`
    /// - Más barata en gas (solo lectura)
    /// 
    /// # Retorna
    /// El valor actual del contador (0 si nunca se incrementó)
    /// 
    /// # Ejemplo
    /// ```
    /// let contador = ContadorContractClient::new(&env, &contract_id);
    /// assert_eq!(contador.get_count(), 0);
    /// contador.increment();
    /// assert_eq!(contador.get_count(), 1);
    /// ```
    pub fn get_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0)
    }

    /// Resetea el contador a 0
    /// 
    /// # Características:
    /// - No retorna valor (tipo unit `()`)
    /// - Emite evento indicando el reset
    /// 
    /// # Ejemplo
    /// ```
    /// let contador = ContadorContractClient::new(&env, &contract_id);
    /// contador.increment();
    /// contador.increment();
    /// contador.reset();
    /// assert_eq!(contador.get_count(), 0);
    /// ```
    pub fn reset(env: Env) {
        // Guardar 0 en storage
        env.storage().instance().set(
            &symbol_short!("COUNTER"),
            &0u32
        );

        // Emitir evento
        env.events().publish(
            (symbol_short!("reset"),),
            0u32
        );
    }

    // ============================================================
    // EJERCICIOS GUIADOS ADICIONALES
    // ============================================================

    /// Ejercicio 1: Incrementa el contador por una cantidad específica
    /// 
    /// # Validación:
    /// - Usa `checked_add` para prevenir overflow
    /// - Si hay overflow, panic con mensaje descriptivo
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `amount`: Cantidad a incrementar
    /// 
    /// # Retorna
    /// El nuevo valor del contador después de incrementar por `amount`
    /// 
    /// # Panic
    /// Si `contador + amount` causaría overflow (u32::MAX)
    /// 
    /// # Ejemplo
    /// ```
    /// assert_eq!(contador.increment_by(5), 5);
    /// assert_eq!(contador.increment_by(3), 8);
    /// ```
    pub fn increment_by(env: Env, amount: u32) -> u32 {
        // PASO 1: Leer contador actual
        let contador_actual: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0);

        // PASO 2: VALIDACIÓN - Verificar que amount + contador no cause overflow
        let nuevo_contador = contador_actual
            .checked_add(amount)
            .expect("Error: overflow al incrementar contador");

        // PASO 3: Guardar en storage
        env.storage().instance().set(
            &symbol_short!("COUNTER"),
            &nuevo_contador
        );

        // PASO 4: Emitir evento con información adicional
        env.events().publish(
            (symbol_short!("inc_by"),),
            (amount, nuevo_contador)
        );

        // PASO 5: Retornar nuevo valor
        nuevo_contador
    }

    /// Ejercicio 2: Contador con límite máximo
    /// 
    /// Versión modificada de `increment()` que incluye límite máximo de 1000
    /// 
    /// # Validación:
    /// - Si contador >= 1000: panic!
    /// - Previene que el contador exceda el límite
    /// 
    /// # Nota:
    /// Esta es una versión alternativa de increment con límite.
    /// La función original `increment()` no tiene límite.
    pub fn increment_con_limite(env: Env) -> u32 {
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0);

        // VALIDACIÓN: Verificar límite máximo
        if contador >= 1000 {
            panic!("contador alcanzó el límite máximo de 1000");
        }

        contador += 1;

        env.storage().instance().set(
            &symbol_short!("COUNTER"),
            &contador
        );

        env.events().publish(
            (symbol_short!("increment"),),
            contador
        );

        contador
    }

    /// Ejercicio 4: Decrementa el contador por una cantidad específica
    /// 
    /// # Validación:
    /// - Verifica que contador >= amount antes de restar
    /// - Usa `checked_sub` para prevenir underflow
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `amount`: Cantidad a decrementar
    /// 
    /// # Retorna
    /// El nuevo valor del contador después de decrementar por `amount`
    /// 
    /// # Panic
    /// Si `contador < amount` (no hay suficiente para decrementar)
    /// 
    /// # Ejemplo
    /// ```
    /// contador.increment_by(10);
    /// assert_eq!(contador.decrement_by(3), 7);
    /// assert_eq!(contador.decrement_by(2), 5);
    /// ```
    pub fn decrement_by(env: Env, amount: u32) -> u32 {
        // PASO 1: Leer contador actual
        let contador_actual: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNTER"))
            .unwrap_or(0);

        // PASO 2: VALIDACIÓN - Verificar que contador >= amount
        if contador_actual < amount {
            panic!("No se puede decrementar: contador es menor que amount");
        }

        // PASO 3: Restar de forma segura (prevenir underflow)
        let nuevo_contador = contador_actual
            .checked_sub(amount)
            .expect("Error: underflow al decrementar contador");

        // PASO 4: Guardar en storage
        env.storage().instance().set(
            &symbol_short!("COUNTER"),
            &nuevo_contador
        );

        // PASO 5: Emitir evento con información adicional
        env.events().publish(
            (symbol_short!("dec_by"),),
            (amount, nuevo_contador)
        );

        // PASO 6: Retornar nuevo valor
        nuevo_contador
    }
}

/// Tests para el contador completo
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        testutils::Address as _,
        Address,
        BytesN,
        Env,
    };

    /// Test básico: incrementar funciona correctamente
    #[test]
    fn test_increment() {
        // ARRANGE: Preparar
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // ACT & ASSERT: Ejecutar y verificar
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);
        assert_eq!(client.get_count(), 3);
    }

    /// Test: decrementar funciona correctamente
    #[test]
    fn test_decrement() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar primero
        client.increment();
        client.increment();
        client.increment();

        // Decrementar
        assert_eq!(client.decrement(), 2);
        assert_eq!(client.decrement(), 1);
        assert_eq!(client.get_count(), 1);
    }

    /// Test: decrementar cuando está en 0 debe causar panic
    #[test]
    #[should_panic(expected = "contador ya está en 0")]
    fn test_decrement_panic() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Esto DEBE causar panic porque el contador está en 0
        client.decrement();
    }

    /// Test: reset funciona correctamente
    #[test]
    fn test_reset() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar varias veces
        client.increment();
        client.increment();
        client.increment();

        // Verificar que tiene valor
        assert_eq!(client.get_count(), 3);

        // Resetear
        client.reset();

        // Verificar que volvió a 0
        assert_eq!(client.get_count(), 0);
    }

    /// Test: get_count retorna el valor correcto
    #[test]
    fn test_get_count() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Inicialmente debe ser 0
        assert_eq!(client.get_count(), 0);

        // Después de incrementar
        client.increment();
        assert_eq!(client.get_count(), 1);

        client.increment();
        assert_eq!(client.get_count(), 2);
    }

    /// Test para increment_by: funciona correctamente
    #[test]
    fn test_increment_by() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar por 5
        assert_eq!(client.increment_by(&5), 5);

        // Incrementar por 3 más
        assert_eq!(client.increment_by(&3), 8);

        // Verificar valor final
        assert_eq!(client.get_count(), 8);
    }

    /// Test para increment_by: overflow debe causar panic
    #[test]
    #[should_panic(expected = "overflow al incrementar contador")]
    fn test_increment_by_overflow() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Establecer contador cerca del máximo
        // Incrementar hasta llegar al límite
        let max_value = u32::MAX;
        
        // Intentar incrementar más allá del máximo
        client.increment_by(&max_value);
        
        // Esto debe causar panic
        client.increment_by(&1);
    }

    /// Test para increment_con_limite: funciona hasta el límite
    #[test]
    fn test_increment_con_limite() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar hasta 999
        for _ in 0..999 {
            client.increment_con_limite();
        }

        // Verificar que llegó a 999
        assert_eq!(client.get_count(), 999);

        // Incrementar una vez más (llegará a 1000)
        assert_eq!(client.increment_con_limite(), 1000);
    }

    /// Test para increment_con_limite: panic al exceder límite
    #[test]
    #[should_panic(expected = "contador alcanzó el límite máximo de 1000")]
    fn test_increment_con_limite_panic() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar hasta el límite
        for _ in 0..1000 {
            client.increment_con_limite();
        }

        // Verificar que está en 1000
        assert_eq!(client.get_count(), 1000);

        // Intentar incrementar una vez más - debe causar panic
        client.increment_con_limite();
    }

    /// Test para decrement_by: funciona correctamente
    #[test]
    fn test_decrement_by() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar por 10
        client.increment_by(&10);
        assert_eq!(client.get_count(), 10);

        // Decrementar por 3
        assert_eq!(client.decrement_by(&3), 7);
        assert_eq!(client.get_count(), 7);

        // Decrementar por 2
        assert_eq!(client.decrement_by(&2), 5);
        assert_eq!(client.get_count(), 5);
    }

    /// Test para decrement_by: panic si no hay suficiente
    #[test]
    #[should_panic(expected = "No se puede decrementar: contador")]
    fn test_decrement_by_insuficiente() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar por 5
        client.increment_by(&5);

        // Intentar decrementar por 10 (más de lo que hay) - debe causar panic
        client.decrement_by(&10);
    }

    /// Test: Flujo completo de uso
    #[test]
    fn test_flujo_completo() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Inicialmente 0
        assert_eq!(client.get_count(), 0);

        // Incrementar varias veces
        client.increment();
        client.increment();
        client.increment_by(&5);
        assert_eq!(client.get_count(), 7);

        // Decrementar
        client.decrement();
        client.decrement_by(&2);
        assert_eq!(client.get_count(), 4);

        // Resetear
        client.reset();
        assert_eq!(client.get_count(), 0);

        // Volver a incrementar
        client.increment();
        assert_eq!(client.get_count(), 1);
    }

    /// Test: Múltiples incrementos seguidos
    #[test]
    fn test_multiple_increments() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar 100 veces
        for i in 1..=100 {
            assert_eq!(client.increment(), i);
        }

        // Verificar valor final
        assert_eq!(client.get_count(), 100);
    }

    /// Test: Alternar increment y decrement
    #[test]
    fn test_alternar_increment_decrement() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, ContadorContract);

        let client = ContadorContractClient::new(&env, &contract_id);

        // Incrementar
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);

        // Decrementar
        assert_eq!(client.decrement(), 2);
        assert_eq!(client.decrement(), 1);

        // Incrementar de nuevo
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);

        // Verificar estado final
        assert_eq!(client.get_count(), 3);
    }
}

