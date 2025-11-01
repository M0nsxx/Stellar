#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contracterror,
    contracttype,
    Env,
    Address,
    Symbol,
    i128,
    u32,
    u64,
    Error,
    symbol_short,
};

// ============================================================
// PARTE 3: STORAGE PATTERNS EN SOROBAN
// ============================================================

// ============================================================
// DEFINICIÓN DE ERRORES PERSONALIZADOS
// ============================================================

/// Errores personalizados para el contrato
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    BalanceInsuficiente = 1,
    MontoInvalido = 2,
    NoAutorizada = 3,
    YaInicializado = 4,
    NoInicializado = 5,
    UsuarioNoExiste = 6,
}

// ============================================================
// EJEMPLO 1: Instance Storage - Configuración Global
// ============================================================

/// Ejemplo de Instance Storage para configuración global
/// 
/// Instance Storage es para:
/// - Configuración del contrato (global)
/// - Datos que pertenecen al contrato, no a usuarios específicos
/// - Se extiende con el contrato completo
#[contract]
pub struct ConfiguracionGlobal;

/// DataKey para Instance Storage
#[contracttype]
#[derive(Clone)]
pub enum DataKeyInstance {
    Admin,
    NombreToken,
    TotalOperaciones,
    ConfiguracionEspecial,
}

#[contractimpl]
impl ConfiguracionGlobal {
    /// Inicializar configuración global
    pub fn initialize(env: Env, admin: Address, nombre: Symbol) -> Result<(), Error> {
        // Verificar que no esté ya inicializado
        if env.storage().instance().has(&DataKeyInstance::Admin) {
            return Err(Error::YaInicializado);
        }
        
        // Guardar configuración en Instance Storage
        env.storage().instance().set(&DataKeyInstance::Admin, &admin);
        env.storage().instance().set(&DataKeyInstance::NombreToken, &nombre);
        env.storage().instance().set(&DataKeyInstance::TotalOperaciones, &0u32);
        
        // Extender TTL del contrato completo
        env.storage().instance().extend_ttl(100, 100);
        
        Ok(())
    }
    
    /// Obtener admin (configuración global)
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKeyInstance::Admin)
            .ok_or(Error::NoInicializado)
    }
    
    /// Obtener nombre del token
    pub fn get_nombre_token(env: Env) -> Result<Symbol, Error> {
        env.storage()
            .instance()
            .get(&DataKeyInstance::NombreToken)
            .ok_or(Error::NoInicializado)
    }
    
    /// Incrementar contador global
    pub fn incrementar_operaciones(env: Env) -> Result<u32, Error> {
        let actual = env.storage()
            .instance()
            .get(&DataKeyInstance::TotalOperaciones)
            .unwrap_or(0);
        
        let nuevo = actual
            .checked_add(1)
            .ok_or(Error::NoInicializado)?;
        
        env.storage()
            .instance()
            .set(&DataKeyInstance::TotalOperaciones, &nuevo);
        
        // Extender TTL después de modificar
        env.storage().instance().extend_ttl(100, 100);
        
        Ok(nuevo)
    }
    
    /// Obtener total de operaciones
    pub fn get_total_operaciones(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKeyInstance::TotalOperaciones)
            .unwrap_or(0)
    }
}

// ============================================================
// EJEMPLO 2: Persistent Storage - Datos de Usuarios
// ============================================================

/// Ejemplo de Persistent Storage para datos críticos de usuarios
/// 
/// Persistent Storage es para:
/// - Datos críticos que DEBEN persistir
/// - Datos específicos por usuario
/// - Se controla TTL individualmente
/// - Más caro pero más seguro
#[contract]
pub struct DatosUsuarios;

/// DataKey para Persistent Storage
#[contracttype]
#[derive(Clone)]
pub enum DataKeyPersistent {
    Balance(Address),              // Key compuesta por usuario
    UltimaTransaccion(Address),   // Key compuesta por usuario
    TotalRecibido(Address),      // Key compuesta por usuario
    Registro(u32),                // Key compuesta por ID
}

#[contractimpl]
impl DatosUsuarios {
    /// Obtener balance de usuario (lazy initialization)
    pub fn get_balance(env: Env, usuario: Address) -> i128 {
        // Patrón: Lazy initialization - si no existe, devuelve 0
        env.storage()
            .persistent()
            .get(&DataKeyPersistent::Balance(usuario))
            .unwrap_or(0)
    }
    
    /// Establecer balance de usuario
    pub fn set_balance(env: Env, usuario: Address, balance: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyPersistent::Balance(usuario.clone()), &balance);
        
        // Extender TTL después de modificar
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyPersistent::Balance(usuario), 100, 100);
    }
    
    /// Verificar si usuario existe (patrón de verificación de existencia)
    pub fn usuario_existe(env: Env, usuario: Address) -> bool {
        // Usar has() en lugar de get() - más barato (no deserializa)
        env.storage()
            .persistent()
            .has(&DataKeyPersistent::Balance(usuario))
    }
    
    /// Guardar última transacción
    pub fn guardar_ultima_transaccion(env: Env, usuario: Address, monto: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyPersistent::UltimaTransaccion(usuario.clone()), &monto);
        
        // Extender TTL
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyPersistent::UltimaTransaccion(usuario), 100, 100);
    }
    
    /// Obtener última transacción
    pub fn get_ultima_transaccion(env: Env, usuario: Address) -> Option<i128> {
        env.storage()
            .persistent()
            .get(&DataKeyPersistent::UltimaTransaccion(usuario))
    }
    
    /// Guardar registro por ID
    pub fn guardar_registro(env: Env, id: u32, valor: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyPersistent::Registro(id), &valor);
        
        // Extender TTL
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyPersistent::Registro(id), 100, 100);
    }
    
    /// Obtener registro por ID
    pub fn get_registro(env: Env, id: u32) -> Option<i128> {
        env.storage()
            .persistent()
            .get(&DataKeyPersistent::Registro(id))
    }
}

// ============================================================
// EJEMPLO 3: Temporary Storage - Cache Temporal
// ============================================================

/// Ejemplo de Temporary Storage para cache temporal
/// 
/// Temporary Storage es para:
/// - Datos temporales o cache
/// - TTL corto
/// - Más barato
/// - Puede expirar rápido
#[contract]
pub struct CacheTemporal;

/// DataKey para Temporary Storage
#[contracttype]
#[derive(Clone)]
pub enum DataKeyTemporary {
    PrecioActual,
    CacheCalculo,
    LockTransaccion(Address),
    DatosTemporales(u32),
}

#[contractimpl]
impl CacheTemporal {
    /// Guardar precio actual (cache)
    pub fn guardar_precio(env: Env, precio: i128) {
        env.storage()
            .temporary()
            .set(&DataKeyTemporary::PrecioActual, &precio);
        
        // No extender TTL - es cache, puede expirar
    }
    
    /// Obtener precio actual (cache)
    pub fn get_precio(env: Env) -> Option<i128> {
        env.storage()
            .temporary()
            .get(&DataKeyTemporary::PrecioActual)
    }
    
    /// Guardar cálculo cacheado
    pub fn guardar_calculo(env: Env, resultado: i128) {
        env.storage()
            .temporary()
            .set(&DataKeyTemporary::CacheCalculo, &resultado);
    }
    
    /// Obtener cálculo cacheado
    pub fn get_calculo(env: Env) -> Option<i128> {
        env.storage()
            .temporary()
            .get(&DataKeyTemporary::CacheCalculo)
    }
    
    /// Crear lock temporal durante transacción
    pub fn crear_lock(env: Env, usuario: Address) {
        env.storage()
            .temporary()
            .set(&DataKeyTemporary::LockTransaccion(usuario.clone()), &true);
    }
    
    /// Verificar si existe lock
    pub fn tiene_lock(env: Env, usuario: Address) -> bool {
        env.storage()
            .temporary()
            .has(&DataKeyTemporary::LockTransaccion(usuario))
    }
    
    /// Eliminar lock
    pub fn eliminar_lock(env: Env, usuario: Address) {
        env.storage()
            .temporary()
            .remove(&DataKeyTemporary::LockTransaccion(usuario));
    }
}

// ============================================================
// EJEMPLO 4: Plataforma de Donaciones Completa
// ============================================================

/// Estructura para información de donación
#[contracttype]
#[derive(Clone)]
pub struct DonacionInfo {
    pub donante: Address,
    pub beneficiaria: Address,
    pub monto: i128,
    pub timestamp: u64,
}

/// DataKey para PlataformaDonaciones
#[contracttype]
#[derive(Clone)]
pub enum DataKeyDonaciones {
    // Instance: Configuración del contrato
    Admin,
    NombrePlataforma,
    TotalDonaciones,
    
    // Persistent: Datos críticos de usuarios
    BalanceDonante(Address),
    DonacionesRecibidas(Address),
    Donacion(u32),
    
    // Temporary: Cache
    CacheTotalDonado,
}

/// Plataforma de donaciones completa con todos los patrones
#[contract]
pub struct PlataformaDonaciones;

#[contractimpl]
impl PlataformaDonaciones {
    /// Inicializar (una sola vez)
    pub fn initialize(env: Env, admin: Address, nombre: Symbol) -> Result<(), Error> {
        // Verificar que no esté ya inicializado
        if env.storage().instance().has(&DataKeyDonaciones::Admin) {
            return Err(Error::YaInicializado);
        }
        
        // Guardar configuración global (Instance Storage)
        env.storage().instance().set(&DataKeyDonaciones::Admin, &admin);
        env.storage().instance().set(&DataKeyDonaciones::NombrePlataforma, &nombre);
        env.storage().instance().set(&DataKeyDonaciones::TotalDonaciones, &0u32);
        
        // Extender TTL de configuración
        env.storage().instance().extend_ttl(100, 100);
        
        Ok(())
    }
    
    /// Donar (operación crítica con todos los patrones)
    pub fn donar(
        env: Env,
        donante: Address,
        beneficiaria: Address,
        monto: i128,
    ) -> Result<(), Error> {
        // 1. Validaciones
        donante.require_auth();
        
        if monto <= 0 {
            return Err(Error::MontoInvalido);
        }
        
        // 2. Actualizar balance del donante (Persistent Storage)
        let balance_donante = env.storage()
            .persistent()
            .get(&DataKeyDonaciones::BalanceDonante(donante.clone()))
            .unwrap_or(0);
        
        if balance_donante < monto {
            return Err(Error::BalanceInsuficiente);
        }
        
        let nuevo_balance_donante = balance_donante
            .checked_sub(monto)
            .ok_or(Error::BalanceInsuficiente)?;
        
        env.storage()
            .persistent()
            .set(&DataKeyDonaciones::BalanceDonante(donante.clone()), &nuevo_balance_donante);
        
        // 3. Actualizar donaciones recibidas por beneficiaria (Persistent Storage)
        let total_recibido = env.storage()
            .persistent()
            .get(&DataKeyDonaciones::DonacionesRecibidas(beneficiaria.clone()))
            .unwrap_or(0);
        
        let nuevo_total_recibido = total_recibido
            .checked_add(monto)
            .ok_or(Error::NoInicializado)?;
        
        env.storage()
            .persistent()
            .set(&DataKeyDonaciones::DonacionesRecibidas(beneficiaria.clone()), &nuevo_total_recibido);
        
        // 4. Guardar detalle de donación (Persistent Storage con struct)
        let id_donacion: u32 = env.storage()
            .instance()
            .get(&DataKeyDonaciones::TotalDonaciones)
            .unwrap_or(0);
        
        let donacion = DonacionInfo {
            donante: donante.clone(),
            beneficiaria: beneficiaria.clone(),
            monto,
            timestamp: env.ledger().timestamp(),
        };
        
        env.storage()
            .persistent()
            .set(&DataKeyDonaciones::Donacion(id_donacion), &donacion);
        
        // 5. Incrementar contador global (Instance Storage)
        let nuevo_total = id_donacion
            .checked_add(1)
            .ok_or(Error::NoInicializado)?;
        
        env.storage()
            .instance()
            .set(&DataKeyDonaciones::TotalDonaciones, &nuevo_total);
        
        // 6. Extender TTL de datos críticos (después de operaciones exitosas)
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyDonaciones::BalanceDonante(donante.clone()), 100, 100);
        
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyDonaciones::DonacionesRecibidas(beneficiaria.clone()), 100, 100);
        
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyDonaciones::Donacion(id_donacion), 100, 100);
        
        // 7. Extender TTL de instance storage
        env.storage().instance().extend_ttl(100, 100);
        
        Ok(())
    }
    
    /// Consulta: Obtener balance del donante
    pub fn get_balance_donante(env: Env, donante: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKeyDonaciones::BalanceDonante(donante))
            .unwrap_or(0)
    }
    
    /// Consulta: Obtener total recibido por beneficiaria
    pub fn get_total_recibido(env: Env, beneficiaria: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKeyDonaciones::DonacionesRecibidas(beneficiaria))
            .unwrap_or(0)
    }
    
    /// Consulta: Obtener información de donación por ID
    pub fn get_donacion(env: Env, id: u32) -> Option<DonacionInfo> {
        env.storage()
            .persistent()
            .get(&DataKeyDonaciones::Donacion(id))
    }
    
    /// Consulta: Obtener total de donaciones (global)
    pub fn get_total_donaciones(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKeyDonaciones::TotalDonaciones)
            .unwrap_or(0)
    }
    
    /// Helper: Establecer balance inicial (para testing)
    pub fn establecer_balance(env: Env, donante: Address, balance: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyDonaciones::BalanceDonante(donante.clone()), &balance);
        
        // Extender TTL
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyDonaciones::BalanceDonante(donante), 100, 100);
    }
    
    /// Verificar si donante existe (patrón de verificación)
    pub fn donante_existe(env: Env, donante: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKeyDonaciones::BalanceDonante(donante))
    }
}

// ============================================================
// EJEMPLO 5: Patrón de Datos Relacionados
// ============================================================

/// Ejemplo del patrón de eliminar datos relacionados juntos
#[contract]
pub struct GestionUsuario;

/// DataKey para gestión de usuario
#[contracttype]
#[derive(Clone)]
pub enum DataKeyUsuario {
    Balance(Address),
    UltimaDonacion(Address),
    TotalDonado(Address),
    Perfil(Address),
}

#[contractimpl]
impl GestionUsuario {
    /// Crear usuario con múltiples datos relacionados
    pub fn crear_usuario(
        env: Env,
        usuario: Address,
        balance: i128,
        total_donado: i128,
    ) {
        // Guardar múltiples datos relacionados
        env.storage()
            .persistent()
            .set(&DataKeyUsuario::Balance(usuario.clone()), &balance);
        
        env.storage()
            .persistent()
            .set(&DataKeyUsuario::TotalDonado(usuario.clone()), &total_donado);
        
        // Extender TTL de todos los datos relacionados
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyUsuario::Balance(usuario.clone()), 100, 100);
        
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyUsuario::TotalDonado(usuario.clone()), 100, 100);
    }
    
    /// Eliminar usuario (patrón de eliminar datos relacionados)
    pub fn eliminar_usuario(env: Env, usuario: Address) -> Result<(), Error> {
        // Verificar que el usuario existe
        if !env.storage().persistent().has(&DataKeyUsuario::Balance(usuario.clone())) {
            return Err(Error::UsuarioNoExiste);
        }
        
        // Eliminar todos los datos relacionados juntos
        env.storage()
            .persistent()
            .remove(&DataKeyUsuario::Balance(usuario.clone()));
        
        env.storage()
            .persistent()
            .remove(&DataKeyUsuario::UltimaDonacion(usuario.clone()));
        
        env.storage()
            .persistent()
            .remove(&DataKeyUsuario::TotalDonado(usuario.clone()));
        
        // Si existe perfil, también eliminarlo
        if env.storage().persistent().has(&DataKeyUsuario::Perfil(usuario.clone())) {
            env.storage()
                .persistent()
                .remove(&DataKeyUsuario::Perfil(usuario));
        }
        
        Ok(())
    }
    
    /// Verificar si usuario existe
    pub fn usuario_existe(env: Env, usuario: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKeyUsuario::Balance(usuario))
    }
    
    /// Obtener balance
    pub fn get_balance(env: Env, usuario: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKeyUsuario::Balance(usuario))
            .unwrap_or(0)
    }
}

// ============================================================
// EJEMPLO 6: Estrategia de Extensión de TTL
// ============================================================

/// Ejemplo de diferentes estrategias de extensión de TTL
#[contract]
pub struct EstrategiaTTL;

/// DataKey para estrategia TTL
#[contracttype]
#[derive(Clone)]
pub enum DataKeyTTL {
    Balance(Address),
    DatosCriticos(Address),
}

#[contractimpl]
impl EstrategiaTTL {
    /// Estrategia 1: Extender en cada operación
    pub fn actualizar_balance_estrategia1(env: Env, usuario: Address, balance: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyTTL::Balance(usuario.clone()), &balance);
        
        // Siempre extender después de modificar
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyTTL::Balance(usuario), 100, 100);
    }
    
    /// Estrategia 2: Extender solo si está cerca de expirar
    pub fn actualizar_balance_estrategia2(env: Env, usuario: Address, balance: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyTTL::Balance(usuario.clone()), &balance);
        
        // Verificar TTL actual antes de extender
        // Nota: En Soroban real, necesitarías obtener el TTL
        // Aquí mostramos el patrón conceptual
        // Siempre extender después de modificar (para el ejemplo)
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyTTL::Balance(usuario), 100, 100);
    }
    
    /// Guardar datos críticos con extensión de TTL
    pub fn guardar_datos_criticos(env: Env, usuario: Address, datos: i128) {
        env.storage()
            .persistent()
            .set(&DataKeyTTL::DatosCriticos(usuario.clone()), &datos);
        
        // Extender TTL más largo para datos críticos
        env.storage()
            .persistent()
            .extend_ttl(&DataKeyTTL::DatosCriticos(usuario), 100, 200);
    }
    
    /// Obtener balance
    pub fn get_balance(env: Env, usuario: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKeyTTL::Balance(usuario))
            .unwrap_or(0)
    }
}

