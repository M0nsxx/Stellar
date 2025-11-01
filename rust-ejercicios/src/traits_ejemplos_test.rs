#![cfg(test)]

use super::traits_ejemplos::*;
use soroban_sdk::{
    testutils::Address as _,
    Address,
    BytesN,
    Env,
    Symbol,
    Vec as SorobanVec,
    symbol_short,
};

// ============================================================
// TESTS PARA TRAIT DONACION
// ============================================================

#[test]
fn test_donacion_educacion_impl_trait() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, DonacionEducacion);

    let beneficiaria = Address::random(&env);
    let monto = 1000i128;
    let escuela = symbol_short!("ESCOLAR");

    // Inicializar
    DonacionEducacion::new(env.clone(), beneficiaria.clone(), monto, escuela);

    // Crear instancia para usar el trait
    let donacion = DonacionEducacion;

    // Verificar que implementa el trait Donacion
    assert_eq!(donacion.beneficiaria(&env), beneficiaria);
    assert_eq!(donacion.monto(&env), monto);
}

#[test]
fn test_donacion_salud_impl_trait() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, DonacionSalud);

    let beneficiaria = Address::random(&env);
    let monto = 2000i128;
    let hospital = symbol_short!("CENTRAL");

    // Inicializar
    DonacionSalud::new(env.clone(), beneficiaria.clone(), monto, hospital);

    // Crear instancia para usar el trait
    let donacion = DonacionSalud;

    // Verificar que implementa el trait Donacion
    assert_eq!(donacion.beneficiaria(&env), beneficiaria);
    assert_eq!(donacion.monto(&env), monto);
}

#[test]
fn test_donacion_educacion_procesar() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, DonacionEducacion);

    let beneficiaria = Address::random(&env);
    let monto = 1000i128;
    let escuela = symbol_short!("ESCOLAR");
    let donante = Address::random(&env);

    // Inicializar
    DonacionEducacion::new(env.clone(), beneficiaria.clone(), monto, escuela);

    // Procesar donación
    let mut donacion = DonacionEducacion;
    let resultado = donacion.procesar(&env, donante);
    
    // Debe ser exitoso
    assert!(resultado.is_ok());
}

#[test]
fn test_donacion_procesar_monto_invalido() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, DonacionEducacion);

    let beneficiaria = Address::random(&env);
    let monto = -100i128; // Monto negativo
    let escuela = symbol_short!("ESCOLAR");
    let donante = Address::random(&env);

    // Inicializar
    DonacionEducacion::new(env.clone(), beneficiaria, monto, escuela);

    // Procesar donación (debe fallar)
    let mut donacion = DonacionEducacion;
    let resultado = donacion.procesar(&env, donante);
    
    // Debe fallar
    assert!(resultado.is_err());
}

// ============================================================
// TESTS PARA PATRÓN OWNABLE
// ============================================================

#[test]
fn test_micro_credito_initialize() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);

    // Inicializar
    MicroCredito::initialize(env.clone(), owner.clone());

    // Verificar owner
    let contrato = MicroCredito;
    assert_eq!(contrato.get_owner(&env), owner);
}

#[test]
fn test_micro_credito_solicitar_credito() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);
    let solicitante = Address::random(&env);
    let monto = 5000i128;

    // Inicializar
    MicroCredito::initialize(env.clone(), owner);

    // Cualquiera puede solicitar crédito
    let resultado = MicroCredito::solicitar_credito(env.clone(), solicitante, monto);
    assert!(resultado.is_ok());

    // Verificar que se actualizó el total prestado
    assert_eq!(MicroCredito::get_total_prestado(env), monto);
}

#[test]
fn test_micro_credito_cambiar_tasa_owner() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);
    let nueva_tasa = 15u32;

    // Inicializar
    MicroCredito::initialize(env.clone(), owner.clone());

    // El owner puede cambiar la tasa
    let resultado = MicroCredito::cambiar_tasa_interes(env.clone(), owner, nueva_tasa);
    assert!(resultado.is_ok());

    // Verificar que se cambió la tasa
    assert_eq!(MicroCredito::get_tasa_interes(env), nueva_tasa);
}

#[test]
fn test_micro_credito_cambiar_tasa_no_owner() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);
    let no_owner = Address::random(&env);
    let nueva_tasa = 15u32;

    // Inicializar
    MicroCredito::initialize(env.clone(), owner);

    // No-owner intenta cambiar la tasa (debe fallar)
    let resultado = MicroCredito::cambiar_tasa_interes(env.clone(), no_owner, nueva_tasa);
    assert!(resultado.is_err());
}

#[test]
fn test_require_owner_exitoso() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);

    // Inicializar
    MicroCredito::initialize(env.clone(), owner.clone());

    // require_owner debe pasar si es el owner
    let resultado = MicroCredito::require_owner(&env, owner);
    assert!(resultado.is_ok());
}

#[test]
fn test_require_owner_falla() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);
    let no_owner = Address::random(&env);

    // Inicializar
    MicroCredito::initialize(env.clone(), owner);

    // require_owner debe fallar si no es el owner
    let resultado = MicroCredito::require_owner(&env, no_owner);
    assert!(resultado.is_err());
}

#[test]
fn test_tasa_interes_default() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, MicroCredito);

    let owner = Address::random(&env);

    // Inicializar
    MicroCredito::initialize(env.clone(), owner);

    // La tasa por defecto debe ser 10%
    assert_eq!(MicroCredito::get_tasa_interes(env), 10u32);
}

// ============================================================
// TESTS PARA TRAIT VOTABLE
// ============================================================

#[test]
fn test_propuesta_ley_initialize() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, PropuestaLey);

    let titulo = symbol_short!("LEY001");

    // Inicializar
    PropuestaLey::initialize(env.clone(), titulo);

    // Verificar que inicia con 0 votos
    assert_eq!(PropuestaLey::get_votos_si(env.clone()), 0);
    assert_eq!(PropuestaLey::get_votos_no(env.clone()), 0);
}

#[test]
fn test_propuesta_ley_votar() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, PropuestaLey);

    let titulo = symbol_short!("LEY001");

    // Inicializar
    PropuestaLey::initialize(env.clone(), titulo);

    // Votar a favor
    PropuestaLey::votar_a_favor(env.clone());
    PropuestaLey::votar_a_favor(env.clone());

    // Votar en contra
    PropuestaLey::votar_en_contra(env.clone());

    // Verificar votos
    assert_eq!(PropuestaLey::get_votos_si(env.clone()), 2);
    assert_eq!(PropuestaLey::get_votos_no(env.clone()), 1);
}

#[test]
fn test_propuesta_ley_impl_votable() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, PropuestaLey);

    let titulo = symbol_short!("LEY001");

    // Inicializar
    PropuestaLey::initialize(env.clone(), titulo);

    // Votar
    PropuestaLey::votar_a_favor(env.clone());
    PropuestaLey::votar_a_favor(env.clone());
    PropuestaLey::votar_en_contra(env.clone());

    // Usar el trait Votable
    let propuesta = PropuestaLey;
    assert_eq!(propuesta.votos_a_favor(&env), 2);
    assert_eq!(propuesta.votos_en_contra(&env), 1);
    assert_eq!(propuesta.paso(&env), true); // 2 > 1, pasa
}

#[test]
fn test_propuesta_ley_no_pasa() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, PropuestaLey);

    let titulo = symbol_short!("LEY001");

    // Inicializar
    PropuestaLey::initialize(env.clone(), titulo);

    // Más votos en contra que a favor
    PropuestaLey::votar_a_favor(env.clone());
    PropuestaLey::votar_en_contra(env.clone());
    PropuestaLey::votar_en_contra(env.clone());

    // Usar el trait Votable
    let propuesta = PropuestaLey;
    assert_eq!(propuesta.votos_a_favor(&env), 1);
    assert_eq!(propuesta.votos_en_contra(&env), 2);
    assert_eq!(propuesta.paso(&env), false); // 1 < 2, no pasa
}

// ============================================================
// TESTS PARA FUNCIONES GENÉRICAS CON TRAITS
// ============================================================

#[test]
fn test_registrar_donacion_genérico() {
    let env = Env::default();
    let contract_id_educ = BytesN::from_array(&env, &[0; 32]);
    let contract_id_salud = BytesN::from_array(&env, &[1; 32]);
    
    env.register_contract(&contract_id_educ, DonacionEducacion);
    env.register_contract(&contract_id_salud, DonacionSalud);

    let beneficiaria1 = Address::random(&env);
    let beneficiaria2 = Address::random(&env);
    let monto1 = 1000i128;
    let monto2 = 2000i128;

    // Crear donación de educación
    DonacionEducacion::new(env.clone(), beneficiaria1.clone(), monto1, symbol_short!("ESC1"));
    let donacion_educ = DonacionEducacion;

    // Crear donación de salud
    DonacionSalud::new(env.clone(), beneficiaria2.clone(), monto2, symbol_short!("HOSP1"));
    let donacion_salud = DonacionSalud;

    // Crear registro
    let mut registro = SorobanVec::new(&env);

    // Registrar ambas donaciones usando la función genérica
    registrar_donacion(&donacion_educ, &env, &mut registro);
    registrar_donacion(&donacion_salud, &env, &mut registro);

    // Verificar que ambas se registraron
    assert_eq!(registro.len(), 2);
    assert_eq!(registro.get(0).unwrap(), beneficiaria1);
    assert_eq!(registro.get(1).unwrap(), beneficiaria2);
}

#[test]
fn test_contar_aprobadas_genérico() {
    let env = Env::default();
    
    // Crear múltiples propuestas con diferentes resultados
    let propuestas = vec![
        // Propuesta 1: Pasa (5 a favor, 2 en contra)
        // Propuesta 2: No pasa (2 a favor, 5 en contra)
        // Propuesta 3: Pasa (10 a favor, 1 en contra)
    ];

    // Nota: Este test requiere múltiples instancias del contrato
    // En una implementación real, usarías diferentes contract_ids
    
    // Para este test, verificamos la función genérica con una sola propuesta
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, PropuestaLey);

    let titulo = symbol_short!("LEY001");
    PropuestaLey::initialize(env.clone(), titulo);
    
    // Votar (más a favor que en contra)
    PropuestaLey::votar_a_favor(env.clone());
    PropuestaLey::votar_a_favor(env.clone());
    PropuestaLey::votar_en_contra(env.clone());

    let propuesta = PropuestaLey;
    
    // Verificar que pasa
    assert_eq!(propuesta.paso(&env), true);
    
    // Contar aprobadas (con una sola propuesta)
    let propuestas_array = [propuesta];
    let aprobadas = contar_aprobadas(&propuestas_array, &env);
    assert_eq!(aprobadas, 1);
}

