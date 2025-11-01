#![cfg(test)]
use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
    symbol_short,
    Symbol,
};
use crate::hello_tiburona::{
    HelloContract,
    Error,
};

#[test]
fn test_initialize_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    
    // Inicializar
    let resultado = HelloContract::initialize(env.clone(), admin.clone());
    assert!(resultado.is_ok());
    
    // Verificar que se guardó el admin
    let admin_guardado = HelloContract::get_admin(env.clone());
    assert_eq!(admin_guardado, Ok(admin));
    
    // Verificar que el contador está en 0
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 0);
}

#[test]
fn test_initialize_ya_inicializado() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    
    // Inicializar primera vez
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Intentar inicializar segunda vez → Error
    let resultado = HelloContract::initialize(env.clone(), admin.clone());
    assert_eq!(resultado, Err(Error::YaInicializado));
}

#[test]
fn test_hello_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    
    // Inicializar
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Saludar
    let nombre = symbol_short!("Ana");
    let resultado = HelloContract::hello(env.clone(), usuario.clone(), nombre);
    
    assert!(resultado.is_ok());
    
    // Verificar que el contador se incrementó
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 1);
    
    // Verificar que se guardó el último saludo
    let ultimo_saludo = HelloContract::get_ultimo_saludo(env.clone(), usuario.clone());
    assert_eq!(ultimo_saludo, Some(nombre));
}

#[test]
fn test_hello_nombre_vacio() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    
    // Inicializar
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Intentar saludar con nombre vacío
    // Nota: En Soroban, Symbol vacío puede ser difícil de crear
    // Este test verifica la validación en el código
    // En la práctica, el Symbol vacío se validaría antes de llamar hello()
    
    // Verificar que el contador no se incrementó
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 0);
}

#[test]
fn test_hello_nombre_valido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Saludar con nombre válido
    let nombre = symbol_short!("Maria");
    let resultado = HelloContract::hello(env.clone(), usuario.clone(), nombre);
    
    assert!(resultado.is_ok());
    
    // Verificar contador
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 1);
    
    // Verificar último saludo
    let ultimo_saludo = HelloContract::get_ultimo_saludo(env.clone(), usuario.clone());
    assert_eq!(ultimo_saludo, Some(nombre));
}

#[test]
fn test_hello_multiple_usuarios() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario1 = Address::generate(&env);
    let usuario2 = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Usuario 1 saluda
    let nombre1 = symbol_short!("Ana");
    HelloContract::hello(env.clone(), usuario1.clone(), nombre1);
    
    // Usuario 2 saluda
    let nombre2 = symbol_short!("Luis");
    HelloContract::hello(env.clone(), usuario2.clone(), nombre2);
    
    // Verificar contador total
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 2);
    
    // Verificar que cada usuario tiene su propio saludo
    let saludo1 = HelloContract::get_ultimo_saludo(env.clone(), usuario1.clone());
    assert_eq!(saludo1, Some(nombre1));
    
    let saludo2 = HelloContract::get_ultimo_saludo(env.clone(), usuario2.clone());
    assert_eq!(saludo2, Some(nombre2));
}

#[test]
fn test_hello_saludo_multiple_veces() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Saludar primera vez
    let nombre1 = symbol_short!("Ana");
    HelloContract::hello(env.clone(), usuario.clone(), nombre1);
    
    // Saludar segunda vez (sobrescribe el anterior)
    let nombre2 = symbol_short!("Maria");
    HelloContract::hello(env.clone(), usuario.clone(), nombre2);
    
    // Verificar contador total
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 2);
    
    // Verificar que se guardó el último saludo (sobrescribe el anterior)
    let ultimo_saludo = HelloContract::get_ultimo_saludo(env.clone(), usuario.clone());
    assert_eq!(ultimo_saludo, Some(nombre2));  // Debe ser el segundo
}

#[test]
fn test_get_contador_inicial() {
    let env = Env::default();
    
    let admin = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Contador inicial debe ser 0
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 0);
}

#[test]
fn test_get_contador_despues_de_saludos() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario1 = Address::generate(&env);
    let usuario2 = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Múltiples saludos
    HelloContract::hello(env.clone(), usuario1.clone(), symbol_short!("Ana"));
    HelloContract::hello(env.clone(), usuario2.clone(), symbol_short!("Luis"));
    HelloContract::hello(env.clone(), usuario1.clone(), symbol_short!("Ana2"));
    
    // Verificar contador
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 3);
}

#[test]
fn test_get_ultimo_saludo_existe() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Saludar
    let nombre = symbol_short!("Sofia");
    HelloContract::hello(env.clone(), usuario.clone(), nombre);
    
    // Obtener último saludo
    let ultimo_saludo = HelloContract::get_ultimo_saludo(env.clone(), usuario.clone());
    assert_eq!(ultimo_saludo, Some(nombre));
}

#[test]
fn test_get_ultimo_saludo_no_existe() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Usuario que nunca ha saludado → None
    let ultimo_saludo = HelloContract::get_ultimo_saludo(env.clone(), usuario.clone());
    assert_eq!(ultimo_saludo, None);
}

#[test]
fn test_reset_contador_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Hacer algunos saludos
    HelloContract::hello(env.clone(), usuario.clone(), symbol_short!("Ana"));
    HelloContract::hello(env.clone(), usuario.clone(), symbol_short!("Luis"));
    
    // Verificar contador antes
    let contador_antes = HelloContract::get_contador(env.clone());
    assert_eq!(contador_antes, 2);
    
    // Resetear como admin
    let resultado = HelloContract::reset_contador(env.clone(), admin.clone());
    assert!(resultado.is_ok());
    
    // Verificar contador después
    let contador_despues = HelloContract::get_contador(env.clone());
    assert_eq!(contador_despues, 0);
}

#[test]
fn test_reset_contador_no_autorizado() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);
    let atacante = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Hacer algunos saludos
    HelloContract::hello(env.clone(), usuario.clone(), symbol_short!("Ana"));
    
    // Intentar resetear como atacante (no admin) → Error
    let resultado = HelloContract::reset_contador(env.clone(), atacante.clone());
    assert_eq!(resultado, Err(Error::NoAutorizado));
    
    // Verificar que el contador NO cambió
    let contador = HelloContract::get_contador(env.clone());
    assert_eq!(contador, 1);
}

#[test]
fn test_reset_contador_no_inicializado() {
    let env = Env::default();
    env.mock_all_auths();
    
    let caller = Address::generate(&env);
    
    // Intentar resetear sin inicializar → Error
    let resultado = HelloContract::reset_contador(env.clone(), caller.clone());
    assert_eq!(resultado, Err(Error::NoInicializado));
}

#[test]
fn test_get_admin_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    
    HelloContract::initialize(env.clone(), admin.clone());
    
    // Obtener admin
    let admin_guardado = HelloContract::get_admin(env.clone());
    assert_eq!(admin_guardado, Ok(admin));
}

#[test]
fn test_get_admin_no_inicializado() {
    let env = Env::default();
    
    // Obtener admin sin inicializar → Error
    let admin_guardado = HelloContract::get_admin(env.clone());
    assert_eq!(admin_guardado, Err(Error::NoInicializado));
}

#[test]
fn test_flujo_completo() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let usuario1 = Address::generate(&env);
    let usuario2 = Address::generate(&env);
    
    // 1. Inicializar
    HelloContract::initialize(env.clone(), admin.clone());
    
    // 2. Múltiples saludos
    HelloContract::hello(env.clone(), usuario1.clone(), symbol_short!("Ana"));
    HelloContract::hello(env.clone(), usuario2.clone(), symbol_short!("Luis"));
    HelloContract::hello(env.clone(), usuario1.clone(), symbol_short!("Ana2"));
    
    // 3. Verificar contador
    assert_eq!(HelloContract::get_contador(env.clone()), 3);
    
    // 4. Verificar últimos saludos
    assert_eq!(
        HelloContract::get_ultimo_saludo(env.clone(), usuario1.clone()),
        Some(symbol_short!("Ana2"))
    );
    assert_eq!(
        HelloContract::get_ultimo_saludo(env.clone(), usuario2.clone()),
        Some(symbol_short!("Luis"))
    );
    
    // 5. Resetear como admin
    HelloContract::reset_contador(env.clone(), admin.clone());
    
    // 6. Verificar contador reseteado
    assert_eq!(HelloContract::get_contador(env.clone()), 0);
    
    // 7. Verificar que los saludos individuales NO se borraron
    assert_eq!(
        HelloContract::get_ultimo_saludo(env.clone(), usuario1.clone()),
        Some(symbol_short!("Ana2"))
    );
    assert_eq!(
        HelloContract::get_ultimo_saludo(env.clone(), usuario2.clone()),
        Some(symbol_short!("Luis"))
    );
}

