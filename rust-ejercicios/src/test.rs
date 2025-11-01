#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, BytesN, Env, Vec};

/// Test para Ejercicio 5: contar_mayores
#[test]
fn test_contar_mayores() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Crear Vec con números
    let mut numeros: Vec<u32> = Vec::new(&env);
    numeros.push_back(50);
    numeros.push_back(150);
    numeros.push_back(200);
    numeros.push_back(80);

    // Ejecutar función
    let resultado = client.contar_mayores(&numeros);

    // Verificar resultado: 2 números mayores a 100 (150 y 200)
    assert_eq!(resultado, 2);
}

#[test]
fn test_contar_mayores_todos_menores() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Vec con todos menores a 100
    let mut numeros: Vec<u32> = Vec::new(&env);
    numeros.push_back(10);
    numeros.push_back(20);
    numeros.push_back(30);

    let resultado = client.contar_mayores(&numeros);
    assert_eq!(resultado, 0);
}

#[test]
fn test_contar_mayores_vacio() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Vec vacío
    let numeros: Vec<u32> = Vec::new(&env);

    let resultado = client.contar_mayores(&numeros);
    assert_eq!(resultado, 0);
}

/// Tests para Ejercicio 6: validar_cantidad
#[test]
fn test_validar_cantidad_exitoso() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Casos válidos (necesita env)
    let resultado_1 = client.validar_cantidad(&env, &1);
    assert!(resultado_1.is_ok());
    assert_eq!(resultado_1.unwrap(), 1);

    let resultado_500 = client.validar_cantidad(&env, &500);
    assert!(resultado_500.is_ok());
    assert_eq!(resultado_500.unwrap(), 500);

    let resultado_1000 = client.validar_cantidad(&env, &1000);
    assert!(resultado_1000.is_ok());
    assert_eq!(resultado_1000.unwrap(), 1000);
}

#[test]
fn test_validar_cantidad_error_cero() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Caso error: cantidad = 0
    let resultado = client.validar_cantidad(&env, &0);
    assert!(resultado.is_err());
}

#[test]
fn test_validar_cantidad_error_mayor_1000() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Caso error: cantidad > 1000
    let resultado = client.validar_cantidad(&env, &1001);
    assert!(resultado.is_err());

    let resultado = client.validar_cantidad(&env, &5000);
    assert!(resultado.is_err());
}

#[test]
fn test_procesar_deposito_exitoso() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Depositar cantidad válida
    let resultado = client.procesar_deposito(&500);
    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), 500);
}

#[test]
fn test_procesar_deposito_error() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Depositar cantidad inválida (0)
    let resultado = client.procesar_deposito(&0);
    assert!(resultado.is_err());

    // Depositar cantidad inválida (>1000)
    let resultado = client.procesar_deposito(&2000);
    assert!(resultado.is_err());
}

/// Tests para Ejercicio 8: transferir
#[test]
fn test_transferir_exitoso() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Crear direcciones de prueba
    let from = Address::random(&env);
    let to = Address::random(&env);

    // Establecer balance inicial para 'from'
    client.establecer_balance(&from, &1_000_000);

    // Verificar balance inicial
    let balance_from_inicial = client.obtener_balance(&from);
    assert_eq!(balance_from_inicial, 1_000_000);

    let balance_to_inicial = client.obtener_balance(&to);
    assert_eq!(balance_to_inicial, 0);

    // Transferir 500,000 tokens
    let amount = 500_000;
    let resultado = client.transferir(&from, &to, &amount);
    assert!(resultado.is_ok());

    // Verificar balances después de la transferencia
    let balance_from_final = client.obtener_balance(&from);
    assert_eq!(balance_from_final, 500_000);

    let balance_to_final = client.obtener_balance(&to);
    assert_eq!(balance_to_final, 500_000);
}

#[test]
fn test_transferir_error_cantidad_cero() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    let from = Address::random(&env);
    let to = Address::random(&env);

    client.establecer_balance(&from, &1_000_000);

    // Intentar transferir 0 tokens
    let resultado = client.transferir(&from, &to, &0);
    assert!(resultado.is_err());
}

#[test]
fn test_transferir_error_balance_insuficiente() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    let from = Address::random(&env);
    let to = Address::random(&env);

    // Establecer balance menor al amount a transferir
    client.establecer_balance(&from, &100);

    // Intentar transferir más de lo que tiene
    let resultado = client.transferir(&from, &to, &1_000);
    assert!(resultado.is_err());
}

#[test]
fn test_transferir_multiple() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    let from = Address::random(&env);
    let to1 = Address::random(&env);
    let to2 = Address::random(&env);

    // Balance inicial alto
    client.establecer_balance(&from, &10_000_000);

    // Primera transferencia
    let resultado1 = client.transferir(&from, &to1, &2_000_000);
    assert!(resultado1.is_ok());

    // Segunda transferencia
    let resultado2 = client.transferir(&from, &to2, &3_000_000);
    assert!(resultado2.is_ok());

    // Verificar balances finales
    assert_eq!(client.obtener_balance(&from), 5_000_000);
    assert_eq!(client.obtener_balance(&to1), 2_000_000);
    assert_eq!(client.obtener_balance(&to2), 3_000_000);
}

/// Tests para funciones auxiliares
#[test]
fn test_sumar_segura_exitoso() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Suma válida
    let resultado = client.sumar_segura(&env, &100, &50);
    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), 150);
}

#[test]
fn test_sumar_segura_overflow() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Overflow (255 + 1 = 256, no cabe en u8)
    let resultado = client.sumar_segura(&env, &255, &1);
    assert!(resultado.is_err());
}

#[test]
fn test_restar_segura_exitoso() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Resta válida
    let resultado = client.restar_segura(&env, &100, &50);
    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), 50);
}

#[test]
fn test_restar_segura_underflow() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, EjerciciosSoroban);

    let client = EjerciciosSorobanClient::new(&env, &contract_id);

    // Underflow (50 - 100 = negativo, no permitido en u32)
    let resultado = client.restar_segura(&env, &50, &100);
    assert!(resultado.is_err());
}

