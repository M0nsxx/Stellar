#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    Env,
    symbol_short,
    Address,
    Symbol,
    Vec as SorobanVec,
    String as SorobanString,
};

// ============================================================
// NIVEL 1: Entendiendo el código
// ============================================================

/// Ejercicio 1.1: Mystery functions para análisis
#[contract]
pub struct MysteryFunctions;

#[contractimpl]
impl MysteryFunctions {
    /// Mystery Function A - Analizar comportamiento
    /// 
    /// # Preguntas:
    /// 1. ¿Qué hace esta función? ¿Modifica el storage?
    /// 2. ¿Qué valor inicial usa si "DATA" no existe?
    /// 3. ¿Por qué no necesita `mut`?
    pub fn mystery_function_a(env: Env) -> u32 {
        let value: u32 = env.storage()
            .instance()
            .get(&symbol_short!("DATA"))
            .unwrap_or(10);
        
        value * 2
    }

    /// Mystery Function B - Analizar comportamiento
    /// 
    /// # Preguntas:
    /// 1. ¿Qué hace esta función? ¿Modifica el storage?
    /// 2. ¿Por qué `current` necesita `mut`?
    /// 3. ¿Qué pasa si llamamos esta función tres veces seguidas?
    pub fn mystery_function_b(env: Env, x: u32) {
        let mut current: u32 = env.storage()
            .instance()
            .get(&symbol_short!("TOTAL"))
            .unwrap_or(0);
        
        current += x;
        
        env.storage().instance().set(
            &symbol_short!("TOTAL"),
            &current
        );
    }
}

// ============================================================
// NIVEL 2: Modificando el contador
// ============================================================

/// Ejercicio 2.1: Contador con increment_by
/// 
/// Versión extendida del contador con función increment_by
#[contract]
pub struct ContadorExtendido;

#[contractimpl]
impl ContadorExtendido {
    /// Obtiene el valor actual del contador
    pub fn get_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0)
    }

    /// Incrementa el contador por una cantidad específica
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `amount`: Cantidad a incrementar
    /// 
    /// # Retorna
    /// El nuevo valor del contador después de incrementar
    pub fn increment_by(env: Env, amount: u32) -> u32 {
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);
        
        contador += amount;
        
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &contador
        );
        
        env.events().publish(
            (symbol_short!("incr_by"),),
            contador
        );
        
        contador
    }

    /// Incrementa el contador en 1
    pub fn increment(env: Env) -> u32 {
        Self::increment_by(env, 1)
    }

    /// Decrementa el contador en 1 (con validación)
    pub fn decrement(env: Env) -> u32 {
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);
        
        if contador == 0 {
            panic!("No se puede decrementar: contador ya está en 0");
        }
        
        contador -= 1;
        
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &contador
        );
        
        env.events().publish(
            (symbol_short!("decrement"),),
            contador
        );
        
        contador
    }

    /// Resetea el contador a 0
    pub fn reset(env: Env) {
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &0u32
        );
        
        env.events().publish(
            (symbol_short!("reset"),),
            0u32
        );
    }
}

/// Ejercicio 2.2: Contador con límite máximo
/// 
/// Contador que no puede exceder 1000
#[contract]
pub struct ContadorConLimite;

#[contractimpl]
impl ContadorConLimite {
    /// Obtiene el valor actual del contador
    pub fn get_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0)
    }

    /// Incrementa el contador en 1 (con límite máximo de 1000)
    pub fn increment(env: Env) -> u32 {
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);
        
        // Validación de límite máximo
        if contador >= 1000 {
            panic!("Contador ha alcanzado el máximo de 1000");
        }
        
        contador += 1;
        
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &contador
        );
        
        env.events().publish(
            (symbol_short!("increment"),),
            contador
        );
        
        contador
    }
}

/// Ejercicio 2.3: Contador con función set_value
/// 
/// Permite establecer el contador a un valor específico
#[contract]
pub struct ContadorConSetValue;

#[contractimpl]
impl ContadorConSetValue {
    /// Obtiene el valor actual del contador
    pub fn get_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0)
    }

    /// Establece el contador a un valor específico
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `new_value`: Nuevo valor del contador (0-1000)
    /// 
    /// # Panic
    /// Si `new_value` > 1000
    pub fn set_value(env: Env, new_value: u32) {
        // Validación
        if new_value > 1000 {
            panic!("Valor debe estar entre 0 y 1000");
        }
        
        // Guardar
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &new_value
        );
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("set_val"),),
            new_value
        );
    }

    /// Incrementa el contador en 1
    pub fn increment(env: Env) -> u32 {
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);
        
        contador += 1;
        
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &contador
        );
        
        env.events().publish(
            (symbol_short!("increment"),),
            contador
        );
        
        contador
    }
}

// ============================================================
// NIVEL 3: Proyectos nuevos
// ============================================================

/// Ejercicio 3.1: Contador con historial
/// 
/// Mantiene historial de los últimos 5 valores
#[contract]
pub struct ContadorConHistorial;

#[contractimpl]
impl ContadorConHistorial {
    /// Obtiene el valor actual del contador
    pub fn get_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0)
    }

    /// Incrementa el contador y agrega al historial
    pub fn increment(env: Env) -> u32 {
        // 1. Leer contador actual
        let mut contador: u32 = env.storage()
            .instance()
            .get(&symbol_short!("COUNT"))
            .unwrap_or(0);
        
        // 2. Incrementar
        contador += 1;
        
        // 3. Leer historial actual
        let mut history: SorobanVec<u32> = env.storage()
            .instance()
            .get(&symbol_short!("HIST"))
            .unwrap_or(SorobanVec::new(&env));
        
        // 4. Agregar nuevo valor al historial
        history.push_back(contador);
        
        // 5. Si historial > 5, mantener solo los últimos 5
        // Nota: En Soroban, Vec no tiene método para crear slice directamente
        // Mantenemos todos los elementos pero documentamos que deben limitarse
        // En una implementación real, podrías usar un Vec con máximo 5 elementos
        // y remover el primero cuando se agrega uno nuevo
        if history.len() > 5 {
            // Remover el primer elemento hasta tener solo 5
            // Nota: Soroban Vec no tiene remove(0) directo, 
            // así que reconstruimos el Vec sin el primer elemento
            let mut new_history = SorobanVec::new(&env);
            let start_idx = history.len() - 5;
            for i in start_idx..history.len() {
                if let Some(val) = history.get(i) {
                    new_history.push_back(val);
                }
            }
            history = new_history;
        }
        
        // 6. Guardar contador e historial
        env.storage().instance().set(
            &symbol_short!("COUNT"),
            &contador
        );
        env.storage().instance().set(
            &symbol_short!("HIST"),
            &history
        );
        
        // 7. Emitir evento
        env.events().publish(
            (symbol_short!("increment"),),
            contador
        );
        
        // 8. Retornar nuevo valor
        contador
    }

    /// Obtiene el historial completo (máx 5 valores)
    pub fn get_history(env: Env) -> SorobanVec<u32> {
        env.storage()
            .instance()
            .get(&symbol_short!("HIST"))
            .unwrap_or(SorobanVec::new(&env))
    }
}

/// Ejercicio 3.2: Sistema de votación simple
/// 
/// Sistema de votación con dos opciones
#[contract]
pub struct SistemaVotacion;

#[contractimpl]
impl SistemaVotacion {
    /// Vota por opción A
    pub fn vote_a(env: Env) {
        let mut votos_a: u32 = env.storage()
            .instance()
            .get(&symbol_short!("VOTE_A"))
            .unwrap_or(0);
        
        votos_a += 1;
        
        env.storage().instance().set(
            &symbol_short!("VOTE_A"),
            &votos_a
        );
        
        env.events().publish(
            (symbol_short!("vote_a"),),
            votos_a
        );
    }

    /// Vota por opción B
    pub fn vote_b(env: Env) {
        let mut votos_b: u32 = env.storage()
            .instance()
            .get(&symbol_short!("VOTE_B"))
            .unwrap_or(0);
        
        votos_b += 1;
        
        env.storage().instance().set(
            &symbol_short!("VOTE_B"),
            &votos_b
        );
        
        env.events().publish(
            (symbol_short!("vote_b"),),
            votos_b
        );
    }

    /// Obtiene los resultados (votos_a, votos_b)
    pub fn get_results(env: Env) -> (u32, u32) {
        let votos_a: u32 = env.storage()
            .instance()
            .get(&symbol_short!("VOTE_A"))
            .unwrap_or(0);
        
        let votos_b: u32 = env.storage()
            .instance()
            .get(&symbol_short!("VOTE_B"))
            .unwrap_or(0);
        
        (votos_a, votos_b)
    }

    /// Obtiene el ganador: "A", "B", o "tie"
    pub fn get_winner(env: Env) -> Symbol {
        let (votos_a, votos_b) = Self::get_results(env);
        
        if votos_a > votos_b {
            symbol_short!("A")
        } else if votos_b > votos_a {
            symbol_short!("B")
        } else {
            symbol_short!("tie")
        }
    }
}

// ============================================================
// PROYECTO INTEGRADOR: Sistema de reputación
// ============================================================

/// Proyecto integrador: Sistema de reputación simple
/// 
/// Permite dar likes y dislikes a entidades
#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    /// Da un "like" a una entidad
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `entity`: Symbol identificando la entidad
    /// - `user`: Address del usuario que vota
    pub fn like(env: Env, entity: Symbol, user: Address) {
        // 1. Verificar que el usuario no haya votado
        let vote_key = (entity.clone(), user.clone());
        let existing_vote: Option<Symbol> = env.storage()
            .instance()
            .get(&vote_key);
        
        if existing_vote.is_some() {
            panic!("Usuario ya votó por esta entidad");
        }
        
        // 2. Incrementar contador de likes para entity
        let key_likes = (entity.clone(), symbol_short!("likes"));
        let mut likes: u32 = env.storage()
            .instance()
            .get(&key_likes)
            .unwrap_or(0);
        
        likes += 1;
        
        // 3. Registrar que user votó por entity
        env.storage().instance().set(
            &vote_key,
            &symbol_short!("like")
        );
        
        // 4. Guardar nuevo contador de likes
        env.storage().instance().set(
            &key_likes,
            &likes
        );
        
        // 5. Emitir evento
        env.events().publish(
            (symbol_short!("like"),),
            (entity, user, likes)
        );
    }

    /// Da un "dislike" a una entidad
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `entity`: Symbol identificando la entidad
    /// - `user`: Address del usuario que vota
    pub fn dislike(env: Env, entity: Symbol, user: Address) {
        // 1. Verificar que el usuario no haya votado
        let vote_key = (entity.clone(), user.clone());
        let existing_vote: Option<Symbol> = env.storage()
            .instance()
            .get(&vote_key);
        
        if existing_vote.is_some() {
            panic!("Usuario ya votó por esta entidad");
        }
        
        // 2. Incrementar contador de dislikes para entity
        let key_dislikes = (entity.clone(), symbol_short!("dislikes"));
        let mut dislikes: u32 = env.storage()
            .instance()
            .get(&key_dislikes)
            .unwrap_or(0);
        
        dislikes += 1;
        
        // 3. Registrar que user votó por entity
        env.storage().instance().set(
            &vote_key,
            &symbol_short!("dislike")
        );
        
        // 4. Guardar nuevo contador de dislikes
        env.storage().instance().set(
            &key_dislikes,
            &dislikes
        );
        
        // 5. Emitir evento
        env.events().publish(
            (symbol_short!("dislike"),),
            (entity, user, dislikes)
        );
    }

    /// Obtiene el número de likes para una entidad
    pub fn get_likes(env: Env, entity: Symbol) -> u32 {
        let key_likes = (entity, symbol_short!("likes"));
        env.storage()
            .instance()
            .get(&key_likes)
            .unwrap_or(0)
    }

    /// Obtiene el número de dislikes para una entidad
    pub fn get_dislikes(env: Env, entity: Symbol) -> u32 {
        let key_dislikes = (entity, symbol_short!("dislikes"));
        env.storage()
            .instance()
            .get(&key_dislikes)
            .unwrap_or(0)
    }

    /// Obtiene el score (likes - dislikes) para una entidad
    /// 
    /// # Retorna
    /// i32 - Puede ser negativo si hay más dislikes que likes
    pub fn get_score(env: Env, entity: Symbol) -> i32 {
        let likes = Self::get_likes(env.clone(), entity.clone()) as i32;
        let dislikes = Self::get_dislikes(env, entity) as i32;
        
        likes - dislikes
    }

    /// Verifica si un usuario ya votó por una entidad
    /// 
    /// # Retorna
    /// true si el usuario ya votó, false si no
    pub fn has_voted(env: Env, entity: Symbol, user: Address) -> bool {
        let vote_key = (entity, user);
        env.storage()
            .instance()
            .get(&vote_key)
            .is_some()
    }
}

