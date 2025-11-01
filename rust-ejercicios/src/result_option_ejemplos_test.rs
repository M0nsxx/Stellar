#![cfg(test)]
use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
    symbol_short,
};
use crate::result_option_ejemplos::{
    TransferInseguro,
    TransferSeguro,
    OptionEjemplo,
    MicroCredito,
    ConversionOptionResult,
    DonacionValidada,
    Error,
    ValidacionHelper,
};

#[test]
fn test_transfer_inseguro_basico() {
    let env = Env::default();
    env.mock_all_auths();
    
    let cuenta1 = Address::generate(&env);
    let cuenta2 = Address::generate(&env);
    
    // Establecer balance inicial
    TransferInseguro::establecer_balance(env.clone(), cuenta1.clone(), 100);
    
    // Transferir (sin validaciones - inseguro)
    TransferInseguro::transfer_inseguro(env.clone(), cuenta1.clone(), cuenta2.clone(), 50);
    
    // Verificar balances
    assert_eq!(TransferInseguro::obtener_balance(env.clone(), cuenta1.clone()), 50);
    assert_eq!(TransferInseguro::obtener_balance(env.clone(), cuenta2.clone()), -50); // Problema!
}

#[test]
#[should_panic]
fn test_transfer_inseguro_panic_sin_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let cuenta1 = Address::generate(&env);
    let cuenta2 = Address::generate(&env);
    
    // Intentar transferir sin balance inicial → PANIC
    TransferInseguro::transfer_inseguro(env.clone(), cuenta1.clone(), cuenta2.clone(), 50);
}

#[test]
fn test_transfer_seguro_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let cuenta1 = Address::generate(&env);
    let cuenta2 = Address::generate(&env);
    
    // Establecer balance inicial
    TransferSeguro::establecer_balance(env.clone(), cuenta1.clone(), 100);
    
    // Transferir con validaciones
    let resultado = TransferSeguro::transfer(env.clone(), cuenta1.clone(), cuenta2.clone(), 50);
    
    // Verificar éxito
    assert!(resultado.is_ok());
    
    // Verificar balances
    assert_eq!(TransferSeguro::obtener_balance(env.clone(), cuenta1.clone()), 50);
    assert_eq!(TransferSeguro::obtener_balance(env.clone(), cuenta2.clone()), 50);
}

#[test]
fn test_transfer_seguro_monto_invalido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let cuenta1 = Address::generate(&env);
    let cuenta2 = Address::generate(&env);
    
    TransferSeguro::establecer_balance(env.clone(), cuenta1.clone(), 100);
    
    // Intentar transferir con monto negativo → Error
    let resultado = TransferSeguro::transfer(env.clone(), cuenta1.clone(), cuenta2.clone(), -50);
    
    assert_eq!(resultado, Err(Error::MontoInvalido));
    
    // Verificar que los balances no cambiaron
    assert_eq!(TransferSeguro::obtener_balance(env.clone(), cuenta1.clone()), 100);
    assert_eq!(TransferSeguro::obtener_balance(env.clone(), cuenta2.clone()), 0);
}

#[test]
fn test_transfer_seguro_balance_insuficiente() {
    let env = Env::default();
    env.mock_all_auths();
    
    let cuenta1 = Address::generate(&env);
    let cuenta2 = Address::generate(&env);
    
    TransferSeguro::establecer_balance(env.clone(), cuenta1.clone(), 100);
    
    // Intentar transferir más de lo que tiene → Error
    let resultado = TransferSeguro::transfer(env.clone(), cuenta1.clone(), cuenta2.clone(), 150);
    
    assert_eq!(resultado, Err(Error::BalanceInsuficiente));
    
    // Verificar que los balances no cambiaron
    assert_eq!(TransferSeguro::obtener_balance(env.clone(), cuenta1.clone()), 100);
    assert_eq!(TransferSeguro::obtener_balance(env.clone(), cuenta2.clone()), 0);
}

#[test]
fn test_transfer_seguro_cuenta_nueva() {
    let env = Env::default();
    env.mock_all_auths();
    
    let cuenta1 = Address::generate(&env);
    let cuenta2 = Address::generate(&env);
    
    // Cuenta nueva con balance 0
    TransferSeguro::establecer_balance(env.clone(), cuenta1.clone(), 0);
    
    // Intentar transferir → Error (balance insuficiente)
    let resultado = TransferSeguro::transfer(env.clone(), cuenta1.clone(), cuenta2.clone(), 50);
    
    assert_eq!(resultado, Err(Error::BalanceInsuficiente));
}

#[test]
fn test_option_get_balance_existe() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Establecer balance
    OptionEjemplo::establecer_balance(env.clone(), usuario.clone(), 100);
    
    // Obtener balance como Option
    let balance = OptionEjemplo::get_balance(env.clone(), usuario.clone());
    
    assert_eq!(balance, Some(100));
}

#[test]
fn test_option_get_balance_no_existe() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Usuario nuevo sin balance
    let balance = OptionEjemplo::get_balance(env.clone(), usuario.clone());
    
    assert_eq!(balance, None);
}

#[test]
fn test_option_get_balance_or_zero() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Usuario nuevo → retorna 0
    let balance = OptionEjemplo::get_balance_or_zero(env.clone(), usuario.clone());
    assert_eq!(balance, 0);
    
    // Establecer balance
    OptionEjemplo::establecer_balance(env.clone(), usuario.clone(), 100);
    
    // Usuario existente → retorna balance
    let balance = OptionEjemplo::get_balance_or_zero(env.clone(), usuario.clone());
    assert_eq!(balance, 100);
}

#[test]
fn test_option_map() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Sin balance → None
    let balance_doble = OptionEjemplo::get_balance_doble(env.clone(), usuario.clone());
    assert_eq!(balance_doble, None);
    
    // Con balance → Some(balance * 2)
    OptionEjemplo::establecer_balance(env.clone(), usuario.clone(), 50);
    let balance_doble = OptionEjemplo::get_balance_doble(env.clone(), usuario.clone());
    assert_eq!(balance_doble, Some(100));
}

#[test]
fn test_microcredito_get_limite_none() {
    let env = Env::default();
    
    let solicitante = Address::generate(&env);
    
    // Solicitante nueva sin límite
    let limite = MicroCredito::get_limite(env.clone(), solicitante.clone());
    
    assert_eq!(limite, None);
}

#[test]
fn test_microcredito_solicitar_prestamo_sin_limite() {
    let env = Env::default();
    env.mock_all_auths();
    
    let solicitante = Address::generate(&env);
    
    // Intentar solicitar préstamo sin límite establecido
    let resultado = MicroCredito::solicitar_prestamo(env.clone(), solicitante.clone(), 100);
    
    assert_eq!(resultado, Err(Error::SolicitanteNoValida));
}

#[test]
fn test_microcredito_solicitar_prestamo_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let solicitante = Address::generate(&env);
    
    // Establecer límite
    MicroCredito::establecer_limite(env.clone(), solicitante.clone(), 1000);
    
    // Solicitar préstamo
    let resultado = MicroCredito::solicitar_prestamo(env.clone(), solicitante.clone(), 500);
    
    assert!(resultado.is_ok());
    
    // Verificar balance
    assert_eq!(MicroCredito::obtener_balance(env.clone(), solicitante.clone()), 500);
    assert_eq!(MicroCredito::obtener_total_prestado(env.clone(), solicitante.clone()), 500);
}

#[test]
fn test_microcredito_solicitar_prestamo_limite_excedido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let solicitante = Address::generate(&env);
    
    // Establecer límite
    MicroCredito::establecer_limite(env.clone(), solicitante.clone(), 1000);
    
    // Intentar solicitar más del límite
    let resultado = MicroCredito::solicitar_prestamo(env.clone(), solicitante.clone(), 1500);
    
    assert_eq!(resultado, Err(Error::LimiteExcedido));
}

#[test]
fn test_microcredito_solicitar_prestamo_monto_invalido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let solicitante = Address::generate(&env);
    
    MicroCredito::establecer_limite(env.clone(), solicitante.clone(), 1000);
    
    // Intentar con monto negativo
    let resultado = MicroCredito::solicitar_prestamo(env.clone(), solicitante.clone(), -100);
    
    assert_eq!(resultado, Err(Error::MontoInvalido));
}

#[test]
fn test_conversion_option_result_admin_existe() {
    let env = Env::default();
    
    let admin = Address::generate(&env);
    
    // Establecer admin
    ConversionOptionResult::establecer_admin(env.clone(), admin.clone());
    
    // Obtener admin
    let resultado = ConversionOptionResult::obtener_admin(env.clone());
    
    assert_eq!(resultado, Ok(admin));
}

#[test]
fn test_conversion_option_result_admin_no_existe() {
    let env = Env::default();
    
    // Sin admin establecido
    let resultado = ConversionOptionResult::obtener_admin(env.clone());
    
    assert_eq!(resultado, Err(Error::NoInicializado));
}

#[test]
fn test_conversion_option_result_propagacion() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    
    // Con admin → éxito
    ConversionOptionResult::establecer_admin(env.clone(), admin.clone());
    let resultado = ConversionOptionResult::obtener_admin_y_usar(env.clone());
    assert!(resultado.is_ok());
    
    // Sin admin → error propagado
    let env2 = Env::default();
    env2.mock_all_auths();
    let resultado = ConversionOptionResult::obtener_admin_y_usar(env2);
    assert_eq!(resultado, Err(Error::NoInicializado));
}

#[test]
fn test_donacion_validada_exitosa() {
    let env = Env::default();
    env.mock_all_auths();
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    // Establecer balance del donante
    DonacionValidada::establecer_balance(env.clone(), donante.clone(), 1000);
    
    // Crear donación
    let resultado = DonacionValidada::crear_donacion(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        500,
    );
    
    assert!(resultado.is_ok());
    
    // Verificar balances
    assert_eq!(DonacionValidada::obtener_balance(env.clone(), donante.clone()), 500);
    assert_eq!(DonacionValidada::obtener_balance(env.clone(), beneficiaria.clone()), 500);
}

#[test]
fn test_donacion_validada_monto_invalido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    DonacionValidada::establecer_balance(env.clone(), donante.clone(), 1000);
    
    // Monto negativo
    let resultado = DonacionValidada::crear_donacion(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        -100,
    );
    
    assert_eq!(resultado, Err(Error::MontoInvalido));
}

#[test]
fn test_donacion_validada_limite_excedido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    DonacionValidada::establecer_balance(env.clone(), donante.clone(), 1_000_000);
    
    // Monto mayor al límite
    let resultado = DonacionValidada::crear_donacion(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        2_000_000,
    );
    
    assert_eq!(resultado, Err(Error::LimiteExcedido));
}

#[test]
fn test_donacion_validada_balance_insuficiente() {
    let env = Env::default();
    env.mock_all_auths();
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    DonacionValidada::establecer_balance(env.clone(), donante.clone(), 100);
    
    // Intentar donar más de lo que tiene
    let resultado = DonacionValidada::crear_donacion(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        500,
    );
    
    assert_eq!(resultado, Err(Error::BalanceInsuficiente));
}

#[test]
fn test_validacion_helper_validar_monto_exitoso() {
    let resultado = ValidacionHelper::validar_monto(100, 1000);
    assert!(resultado.is_ok());
}

#[test]
fn test_validacion_helper_validar_monto_invalido() {
    let resultado = ValidacionHelper::validar_monto(-100, 1000);
    assert_eq!(resultado, Err(Error::MontoInvalido));
}

#[test]
fn test_validacion_helper_validar_monto_limite_excedido() {
    let resultado = ValidacionHelper::validar_monto(2000, 1000);
    assert_eq!(resultado, Err(Error::LimiteExcedido));
}

#[test]
fn test_validacion_helper_validar_balance_exitoso() {
    let resultado = ValidacionHelper::validar_balance(100, 50);
    assert!(resultado.is_ok());
}

#[test]
fn test_validacion_helper_validar_balance_insuficiente() {
    let resultado = ValidacionHelper::validar_balance(100, 150);
    assert_eq!(resultado, Err(Error::BalanceInsuficiente));
}

