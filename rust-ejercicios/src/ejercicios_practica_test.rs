#![cfg(test)]

use super::ejercicios_practica::*;
use soroban_sdk::{
    testutils::Address as _,
    Address,
    BytesN,
    Env,
    Symbol,
    Vec as SorobanVec,
};

// ============================================================
// NIVEL 1: Tests para Mystery Functions
// ============================================================

#[test]
fn test_mystery_function_a() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MysteryFunctions);

    let client = MysteryFunctionsClient::new(&env, &contract_id);

    // Si "DATA" no existe, usa 10 como valor por defecto
    let resultado = client.mystery_function_a();
    assert_eq!(resultado, 20); // 10 * 2 = 20

    // Establecer valor manualmente
    env.storage().instance().set(
        &symbol_short!("DATA"),
        &5u32
    );

    // Ahora debe retornar 5 * 2 = 10
    let resultado = client.mystery_function_a();
    assert_eq!(resultado, 10);
}

#[test]
fn test_mystery_function_b() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MysteryFunctions);

    let client = MysteryFunctionsClient::new(&env, &contract_id);

    // Primera llamada: 0 + 5 = 5
    client.mystery_function_b(&5);
    let total: u32 = env.storage().instance().get(&symbol_short!("TOTAL")).unwrap();
    assert_eq!(total, 5);

    // Segunda llamada: 5 + 5 = 10
    client.mystery_function_b(&5);
    let total: u32 = env.storage().instance().get(&symbol_short!("TOTAL")).unwrap();
    assert_eq!(total, 10);

    // Tercera llamada: 10 + 5 = 15
    client.mystery_function_b(&5);
    let total: u32 = env.storage().instance().get(&symbol_short!("TOTAL")).unwrap();
    assert_eq!(total, 15);
}

// ============================================================
// NIVEL 2: Tests para Contador Extendido
// ============================================================

#[test]
fn test_increment_by() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorExtendido);

    let client = ContadorExtendidoClient::new(&env, &contract_id);

    // Incrementar por 5
    assert_eq!(client.increment_by(&5), 5);
    assert_eq!(client.get_count(), 5);

    // Incrementar por 3 más
    assert_eq!(client.increment_by(&3), 8);
    assert_eq!(client.get_count(), 8);
}

#[test]
fn test_contador_con_limite_increment() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorConLimite);

    let client = ContadorConLimiteClient::new(&env, &contract_id);

    // Incrementar hasta 999
    for _ in 0..999 {
        client.increment();
    }
    assert_eq!(client.get_count(), 999);

    // Incrementar una vez más (llegará a 1000)
    assert_eq!(client.increment(), 1000);
    assert_eq!(client.get_count(), 1000);
}

#[test]
#[should_panic(expected = "máximo de 1000")]
fn test_contador_con_limite_panic() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorConLimite);

    let client = ContadorConLimiteClient::new(&env, &contract_id);

    // Llevar a 1000
    for _ in 0..1000 {
        client.increment();
    }

    // Esto debe fallar
    client.increment();
}

#[test]
fn test_set_value() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorConSetValue);

    let client = ContadorConSetValueClient::new(&env, &contract_id);

    // Establecer a 500
    client.set_value(&500);
    assert_eq!(client.get_count(), 500);

    // Establecer a 0
    client.set_value(&0);
    assert_eq!(client.get_count(), 0);

    // Establecer a 1000 (límite)
    client.set_value(&1000);
    assert_eq!(client.get_count(), 1000);
}

#[test]
#[should_panic(expected = "entre 0 y 1000")]
fn test_set_value_invalid() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorConSetValue);

    let client = ContadorConSetValueClient::new(&env, &contract_id);

    // Debe fallar
    client.set_value(&2000);
}

// ============================================================
// NIVEL 3: Tests para Contador con Historial
// ============================================================

#[test]
fn test_contador_con_historial() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorConHistorial);

    let client = ContadorConHistorialClient::new(&env, &contract_id);

    // Incrementar 7 veces
    for i in 1..=7 {
        let valor = client.increment();
        assert_eq!(valor, i);
    }

    // Verificar historial (debe tener solo los últimos 5: 3, 4, 5, 6, 7)
    let history = client.get_history();
    assert_eq!(history.len(), 5);
    
    // Verificar que contiene los valores correctos
    assert_eq!(history.get(0).unwrap(), 3);
    assert_eq!(history.get(4).unwrap(), 7);
}

#[test]
fn test_historial_menos_de_5() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ContadorConHistorial);

    let client = ContadorConHistorialClient::new(&env, &contract_id);

    // Incrementar solo 3 veces
    client.increment();
    client.increment();
    client.increment();

    // Verificar historial (debe tener 3 elementos)
    let history = client.get_history();
    assert_eq!(history.len(), 3);
    assert_eq!(history.get(0).unwrap(), 1);
    assert_eq!(history.get(2).unwrap(), 3);
}

// ============================================================
// NIVEL 3: Tests para Sistema de Votación
// ============================================================

#[test]
fn test_sistema_votacion() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, SistemaVotacion);

    let client = SistemaVotacionClient::new(&env, &contract_id);

    // Votar por A
    client.vote_a();
    client.vote_a();
    client.vote_a();

    // Votar por B
    client.vote_b();
    client.vote_b();

    // Verificar resultados
    let (votos_a, votos_b) = client.get_results();
    assert_eq!(votos_a, 3);
    assert_eq!(votos_b, 2);

    // Verificar ganador
    let winner = client.get_winner();
    assert_eq!(winner, symbol_short!("A"));
}

#[test]
fn test_sistema_votacion_empate() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, SistemaVotacion);

    let client = SistemaVotacionClient::new(&env, &contract_id);

    // Votar igual por ambas
    client.vote_a();
    client.vote_b();

    // Verificar empate
    let winner = client.get_winner();
    assert_eq!(winner, symbol_short!("tie"));
}

#[test]
fn test_sistema_votacion_b_gana() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, SistemaVotacion);

    let client = SistemaVotacionClient::new(&env, &contract_id);

    // Votar más por B
    client.vote_a();
    client.vote_b();
    client.vote_b();
    client.vote_b();

    // Verificar que B gana
    let winner = client.get_winner();
    assert_eq!(winner, symbol_short!("B"));
}

// ============================================================
// PROYECTO INTEGRADOR: Tests para Sistema de Reputación
// ============================================================

#[test]
fn test_reputation_like() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ReputationContract);

    let client = ReputationContractClient::new(&env, &contract_id);

    let entity = symbol_short!("PRODUCT");
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    // User1 da like
    client.like(&entity, &user1);
    assert_eq!(client.get_likes(&entity), 1);
    assert_eq!(client.get_dislikes(&entity), 0);
    assert_eq!(client.get_score(&entity), 1);

    // User2 da like
    client.like(&entity, &user2);
    assert_eq!(client.get_likes(&entity), 2);
    assert_eq!(client.get_score(&entity), 2);

    // Verificar que ambos usuarios votaron
    assert_eq!(client.has_voted(&entity, &user1), true);
    assert_eq!(client.has_voted(&entity, &user2), true);
}

#[test]
fn test_reputation_dislike() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ReputationContract);

    let client = ReputationContractClient::new(&env, &contract_id);

    let entity = symbol_short!("PRODUCT");
    let user = Address::random(&env);

    // User da dislike
    client.dislike(&entity, &user);
    assert_eq!(client.get_likes(&entity), 0);
    assert_eq!(client.get_dislikes(&entity), 1);
    assert_eq!(client.get_score(&entity), -1);
}

#[test]
fn test_reputation_score_mixto() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ReputationContract);

    let client = ReputationContractClient::new(&env, &contract_id);

    let entity = symbol_short!("PRODUCT");
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    let user3 = Address::random(&env);

    // 2 likes
    client.like(&entity, &user1);
    client.like(&entity, &user2);

    // 1 dislike
    client.dislike(&entity, &user3);

    // Score: 2 - 1 = 1
    assert_eq!(client.get_score(&entity), 1);
    assert_eq!(client.get_likes(&entity), 2);
    assert_eq!(client.get_dislikes(&entity), 1);
}

#[test]
#[should_panic(expected = "ya votó")]
fn test_reputation_no_voto_duplicado() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ReputationContract);

    let client = ReputationContractClient::new(&env, &contract_id);

    let entity = symbol_short!("PRODUCT");
    let user = Address::random(&env);

    // Primer voto OK
    client.like(&entity, &user);

    // Segundo voto debe fallar
    client.like(&entity, &user);
}

#[test]
fn test_reputation_multiple_entidades() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ReputationContract);

    let client = ReputationContractClient::new(&env, &contract_id);

    let entity1 = symbol_short!("PROD1");
    let entity2 = symbol_short!("PROD2");
    let user = Address::random(&env);

    // Votar por ambas entidades
    client.like(&entity1, &user);
    client.like(&entity2, &user);

    // Verificar que ambas tienen likes
    assert_eq!(client.get_likes(&entity1), 1);
    assert_eq!(client.get_likes(&entity2), 1);

    // Verificar que el usuario votó por ambas
    assert_eq!(client.has_voted(&entity1, &user), true);
    assert_eq!(client.has_voted(&entity2, &user), true);
}

#[test]
fn test_reputation_score_negativo() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ReputationContract);

    let client = ReputationContractClient::new(&env, &contract_id);

    let entity = symbol_short!("PRODUCT");
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    let user3 = Address::random(&env);

    // 1 like
    client.like(&entity, &user1);

    // 2 dislikes
    client.dislike(&entity, &user2);
    client.dislike(&entity, &user3);

    // Score: 1 - 2 = -1
    assert_eq!(client.get_score(&entity), -1);
}

