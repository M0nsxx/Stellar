#![cfg(test)]
use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
    symbol_short,
};
use crate::storage_patterns::{
    ConfiguracionGlobal,
    DatosUsuarios,
    CacheTemporal,
    PlataformaDonaciones,
    GestionUsuario,
    EstrategiaTTL,
    Error,
    DataKeyInstance,
    DataKeyPersistent,
    DataKeyTemporary,
    DataKeyDonaciones,
    DataKeyUsuario,
    DataKeyTTL,
    DonacionInfo,
};

#[test]
fn test_instance_storage_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("MiToken");
    
    // Inicializar
    let resultado = ConfiguracionGlobal::initialize(env.clone(), admin.clone(), nombre);
    assert!(resultado.is_ok());
    
    // Verificar que se guardó en Instance Storage
    let admin_guardado = ConfiguracionGlobal::get_admin(env.clone());
    assert_eq!(admin_guardado, Ok(admin));
    
    let nombre_guardado = ConfiguracionGlobal::get_nombre_token(env.clone());
    assert_eq!(nombre_guardado, Ok(nombre));
}

#[test]
fn test_instance_storage_already_initialized() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("MiToken");
    
    // Inicializar primera vez
    ConfiguracionGlobal::initialize(env.clone(), admin.clone(), nombre);
    
    // Intentar inicializar segunda vez → Error
    let resultado = ConfiguracionGlobal::initialize(env.clone(), admin.clone(), nombre);
    assert_eq!(resultado, Err(Error::YaInicializado));
}

#[test]
fn test_instance_storage_incrementar_operaciones() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("MiToken");
    
    // Inicializar
    ConfiguracionGlobal::initialize(env.clone(), admin.clone(), nombre);
    
    // Incrementar operaciones
    let resultado1 = ConfiguracionGlobal::incrementar_operaciones(env.clone());
    assert_eq!(resultado1, Ok(1));
    
    let resultado2 = ConfiguracionGlobal::incrementar_operaciones(env.clone());
    assert_eq!(resultado2, Ok(2));
    
    // Verificar total
    let total = ConfiguracionGlobal::get_total_operaciones(env.clone());
    assert_eq!(total, 2);
}

#[test]
fn test_persistent_storage_get_balance_lazy_init() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Lazy initialization: usuario nuevo → retorna 0
    let balance = DatosUsuarios::get_balance(env.clone(), usuario.clone());
    assert_eq!(balance, 0);
    
    // Establecer balance
    DatosUsuarios::set_balance(env.clone(), usuario.clone(), 100);
    
    // Ahora tiene balance
    let balance = DatosUsuarios::get_balance(env.clone(), usuario.clone());
    assert_eq!(balance, 100);
}

#[test]
fn test_persistent_storage_usuario_existe() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Usuario nuevo → no existe
    assert_eq!(DatosUsuarios::usuario_existe(env.clone(), usuario.clone()), false);
    
    // Establecer balance
    DatosUsuarios::set_balance(env.clone(), usuario.clone(), 100);
    
    // Ahora existe
    assert_eq!(DatosUsuarios::usuario_existe(env.clone(), usuario.clone()), true);
}

#[test]
fn test_persistent_storage_guardar_transaccion() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Guardar última transacción
    DatosUsuarios::guardar_ultima_transaccion(env.clone(), usuario.clone(), 50);
    
    // Obtener última transacción
    let transaccion = DatosUsuarios::get_ultima_transaccion(env.clone(), usuario.clone());
    assert_eq!(transaccion, Some(50));
}

#[test]
fn test_persistent_storage_registro_por_id() {
    let env = Env::default();
    
    // Guardar registros por ID
    DatosUsuarios::guardar_registro(env.clone(), 1, 100);
    DatosUsuarios::guardar_registro(env.clone(), 2, 200);
    
    // Obtener registros
    let registro1 = DatosUsuarios::get_registro(env.clone(), 1);
    assert_eq!(registro1, Some(100));
    
    let registro2 = DatosUsuarios::get_registro(env.clone(), 2);
    assert_eq!(registro2, Some(200));
    
    // Registro inexistente
    let registro3 = DatosUsuarios::get_registro(env.clone(), 3);
    assert_eq!(registro3, None);
}

#[test]
fn test_temporary_storage_cache() {
    let env = Env::default();
    
    // Guardar precio (cache)
    CacheTemporal::guardar_precio(env.clone(), 150);
    
    // Obtener precio
    let precio = CacheTemporal::get_precio(env.clone());
    assert_eq!(precio, Some(150));
}

#[test]
fn test_temporary_storage_lock() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // No tiene lock inicialmente
    assert_eq!(CacheTemporal::tiene_lock(env.clone(), usuario.clone()), false);
    
    // Crear lock
    CacheTemporal::crear_lock(env.clone(), usuario.clone());
    
    // Ahora tiene lock
    assert_eq!(CacheTemporal::tiene_lock(env.clone(), usuario.clone()), true);
    
    // Eliminar lock
    CacheTemporal::eliminar_lock(env.clone(), usuario.clone());
    
    // Ya no tiene lock
    assert_eq!(CacheTemporal::tiene_lock(env.clone(), usuario.clone()), false);
}

#[test]
fn test_plataforma_donaciones_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("Plataforma");
    
    // Inicializar
    let resultado = PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    assert!(resultado.is_ok());
    
    // Verificar que no se puede inicializar dos veces
    let resultado2 = PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    assert_eq!(resultado2, Err(Error::YaInicializado));
    
    // Verificar total de donaciones inicial
    let total = PlataformaDonaciones::get_total_donaciones(env.clone());
    assert_eq!(total, 0);
}

#[test]
fn test_plataforma_donaciones_donar_exitoso() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("Plataforma");
    
    // Inicializar
    PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    // Establecer balance inicial del donante
    PlataformaDonaciones::establecer_balance(env.clone(), donante.clone(), 1000);
    
    // Donar
    let resultado = PlataformaDonaciones::donar(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        500,
    );
    
    assert!(resultado.is_ok());
    
    // Verificar balances
    assert_eq!(PlataformaDonaciones::get_balance_donante(env.clone(), donante.clone()), 500);
    assert_eq!(PlataformaDonaciones::get_total_recibido(env.clone(), beneficiaria.clone()), 500);
    
    // Verificar total de donaciones
    let total = PlataformaDonaciones::get_total_donaciones(env.clone());
    assert_eq!(total, 1);
}

#[test]
fn test_plataforma_donaciones_donar_monto_invalido() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("Plataforma");
    
    PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    PlataformaDonaciones::establecer_balance(env.clone(), donante.clone(), 1000);
    
    // Intentar donar con monto negativo → Error
    let resultado = PlataformaDonaciones::donar(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        -100,
    );
    
    assert_eq!(resultado, Err(Error::MontoInvalido));
    
    // Verificar que los balances no cambiaron
    assert_eq!(PlataformaDonaciones::get_balance_donante(env.clone(), donante.clone()), 1000);
}

#[test]
fn test_plataforma_donaciones_donar_balance_insuficiente() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("Plataforma");
    
    PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    PlataformaDonaciones::establecer_balance(env.clone(), donante.clone(), 100);
    
    // Intentar donar más de lo que tiene → Error
    let resultado = PlataformaDonaciones::donar(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        500,
    );
    
    assert_eq!(resultado, Err(Error::BalanceInsuficiente));
    
    // Verificar que los balances no cambiaron
    assert_eq!(PlataformaDonaciones::get_balance_donante(env.clone(), donante.clone()), 100);
}

#[test]
fn test_plataforma_donaciones_get_donacion() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("Plataforma");
    
    PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    PlataformaDonaciones::establecer_balance(env.clone(), donante.clone(), 1000);
    
    // Donar
    PlataformaDonaciones::donar(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        500,
    );
    
    // Obtener información de donación
    let donacion = PlataformaDonaciones::get_donacion(env.clone(), 0);
    assert!(donacion.is_some());
    
    let info = donacion.unwrap();
    assert_eq!(info.donante, donante);
    assert_eq!(info.beneficiaria, beneficiaria);
    assert_eq!(info.monto, 500);
}

#[test]
fn test_plataforma_donaciones_donante_existe() {
    let env = Env::default();
    
    let donante = Address::generate(&env);
    
    // Donante nuevo → no existe
    assert_eq!(PlataformaDonaciones::donante_existe(env.clone(), donante.clone()), false);
    
    // Establecer balance
    PlataformaDonaciones::establecer_balance(env.clone(), donante.clone(), 100);
    
    // Ahora existe
    assert_eq!(PlataformaDonaciones::donante_existe(env.clone(), donante.clone()), true);
}

#[test]
fn test_gestion_usuario_crear() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Crear usuario con múltiples datos
    GestionUsuario::crear_usuario(
        env.clone(),
        usuario.clone(),
        1000,
        500,
    );
    
    // Verificar que los datos se guardaron
    assert_eq!(GestionUsuario::usuario_existe(env.clone(), usuario.clone()), true);
    assert_eq!(GestionUsuario::get_balance(env.clone(), usuario.clone()), 1000);
}

#[test]
fn test_gestion_usuario_eliminar() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Crear usuario
    GestionUsuario::crear_usuario(
        env.clone(),
        usuario.clone(),
        1000,
        500,
    );
    
    // Verificar que existe
    assert_eq!(GestionUsuario::usuario_existe(env.clone(), usuario.clone()), true);
    
    // Eliminar usuario (elimina todos los datos relacionados)
    let resultado = GestionUsuario::eliminar_usuario(env.clone(), usuario.clone());
    assert!(resultado.is_ok());
    
    // Verificar que ya no existe
    assert_eq!(GestionUsuario::usuario_existe(env.clone(), usuario.clone()), false);
    assert_eq!(GestionUsuario::get_balance(env.clone(), usuario.clone()), 0);
}

#[test]
fn test_gestion_usuario_eliminar_no_existe() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Intentar eliminar usuario que no existe → Error
    let resultado = GestionUsuario::eliminar_usuario(env.clone(), usuario.clone());
    assert_eq!(resultado, Err(Error::UsuarioNoExiste));
}

#[test]
fn test_estrategia_ttl_actualizar() {
    let env = Env::default();
    
    let usuario = Address::generate(&env);
    
    // Estrategia 1: Extender en cada operación
    EstrategiaTTL::actualizar_balance_estrategia1(env.clone(), usuario.clone(), 100);
    assert_eq!(EstrategiaTTL::get_balance(env.clone(), usuario.clone()), 100);
    
    // Estrategia 2: Extender solo si es necesario
    EstrategiaTTL::actualizar_balance_estrategia2(env.clone(), usuario.clone(), 200);
    assert_eq!(EstrategiaTTL::get_balance(env.clone(), usuario.clone()), 200);
    
    // Guardar datos críticos con TTL más largo
    EstrategiaTTL::guardar_datos_criticos(env.clone(), usuario.clone(), 300);
}

#[test]
fn test_multiple_donaciones() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let nombre = symbol_short!("Plataforma");
    
    PlataformaDonaciones::initialize(env.clone(), admin.clone(), nombre);
    
    let donante = Address::generate(&env);
    let beneficiaria = Address::generate(&env);
    
    PlataformaDonaciones::establecer_balance(env.clone(), donante.clone(), 2000);
    
    // Primera donación
    PlataformaDonaciones::donar(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        500,
    );
    
    // Segunda donación
    PlataformaDonaciones::donar(
        env.clone(),
        donante.clone(),
        beneficiaria.clone(),
        300,
    );
    
    // Verificar balances
    assert_eq!(PlataformaDonaciones::get_balance_donante(env.clone(), donante.clone()), 1200);
    assert_eq!(PlataformaDonaciones::get_total_recibido(env.clone(), beneficiaria.clone()), 800);
    
    // Verificar total de donaciones
    assert_eq!(PlataformaDonaciones::get_total_donaciones(env.clone()), 2);
    
    // Verificar información de ambas donaciones
    let donacion1 = PlataformaDonaciones::get_donacion(env.clone(), 0);
    assert_eq!(donacion1.unwrap().monto, 500);
    
    let donacion2 = PlataformaDonaciones::get_donacion(env.clone(), 1);
    assert_eq!(donacion2.unwrap().monto, 300);
}

