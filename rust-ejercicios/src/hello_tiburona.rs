#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contracterror,
    contracttype,
    Env,
    Symbol,
    Address,
};

// ============================================================
// PARTE 4: HELLO TIBURONA MEJORADO
// ============================================================
// De código básico a código profesional

// ============================================================
// DEFINICIÓN DE ERRORES PERSONALIZADOS
// ============================================================

/// Errores personalizados para el contrato
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// Error cuando el nombre está vacío
    NombreVacio = 1,
    /// Error cuando el nombre es muy largo (> 32 caracteres)
    NombreMuyLargo = 2,
    /// Error cuando el caller no está autorizado
    NoAutorizado = 3,
    /// Error cuando el contrato no está inicializado
    NoInicializado = 4,
    /// Error cuando el contrato ya está inicializado
    YaInicializado = 5,
}

// ============================================================
// DATAKEY PARA ORGANIZAR STORAGE
// ============================================================

/// DataKey enum para organizar el storage
/// 
/// Patrón: Instance = del contrato, Persistent = de los usuarios
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    // Instance storage - configuración global
    /// Dirección de la administradora del contrato
    Admin,
    /// Contador total de saludos (global)
    ContadorSaludos,
    
    // Persistent storage - datos de usuarios
    /// Último saludo guardado por cada Tiburona
    UltimoSaludo(Address),
}

// ============================================================
// CONTRATO HELLO TIBURONA MEJORADO
// ============================================================

/// Hello Tiburona - Contrato profesional mejorado
/// 
/// Este contrato demuestra todas las mejores prácticas:
/// - Manejo de errores con Result
/// - Validaciones exhaustivas
/// - Storage organizado con DataKey
/// - Control de acceso (admin)
/// - Gestión de TTL
/// - Funciones de consulta
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // ============================================================
    // SECCIÓN 1: INICIALIZACIÓN
    // ============================================================
    
    /// Inicializa el contrato con una administradora
    /// 
    /// Solo puede llamarse una vez.
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `admin`: Dirección de la administradora
    /// 
    /// # Retorna
    /// `Result<(), Error>` - Ok si éxito, Err si ya está inicializado
    /// 
    /// # Ejemplo
    /// ```rust
    /// initialize(env, admin_address)
    /// ```
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        // 1. Verificar que no esté ya inicializado
        // Usar has() en lugar de get() - más barato (solo verifica existencia)
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::YaInicializado);
        }
        
        // 2. Guardar la administradora (Instance Storage)
        // Admin es configuración del contrato (global)
        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        
        // 3. Inicializar contador en 0 (Instance Storage)
        // Estado inicial explícito > asumir defaults
        // Documentación clara de estado inicial
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        // 4. Extender TTL del instance storage
        // Asegurar que la configuración persista
        // 100 ledgers = umbral conservador
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }
    
    // ============================================================
    // SECCIÓN 2: FUNCIÓN PRINCIPAL - hello()
    // ============================================================
    
    /// Saluda a una Tiburona y registra el evento
    /// 
    /// Valida el nombre y actualiza estadísticas.
    /// 
    /// # Validaciones
    /// 1. Nombre no vacío
    /// 2. Nombre no mayor a 32 caracteres
    /// 
    /// # Operaciones
    /// 1. Incrementa contador global
    /// 2. Guarda último saludo de la Tiburona
    /// 3. Extiende TTL de datos críticos
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `usuario`: Dirección de la Tiburona que saluda
    /// - `nombre`: Nombre de la Tiburona (Symbol)
    /// 
    /// # Retorna
    /// `Result<Symbol, Error>` - Ok con saludo si éxito, Err con error específico
    /// 
    /// # Errores
    /// - `NombreVacio` - Si el nombre está vacío
    /// - `NombreMuyLargo` - Si el nombre es > 32 caracteres
    /// 
    /// # Ejemplo
    /// ```rust
    /// hello(env, usuario_address, symbol_short!("Ana"))
    /// ```
    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: Symbol,
    ) -> Result<Symbol, Error> {
        // Validación 1: Nombre no vacío
        // Primera validación = más barata (no requiere storage)
        let nombre_str = nombre.to_string();
        if nombre_str.len() == 0 {
            return Err(Error::NombreVacio);
        }
        
        // Validación 2: Longitud máxima
        // Prevenir consumo excesivo de gas
        // Storage de strings largos es caro
        if nombre_str.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }
        
        // Incrementar contador de saludos (Instance Storage)
        // Declarar la key una vez (reusable)
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);  // Lazy initialization: si no existe → 0
        
        // Incrementar de forma segura
        let nuevo_contador = contador
            .checked_add(1)
            .ok_or(Error::NoInicializado)?;
        
        env.storage()
            .instance()
            .set(&key_contador, &nuevo_contador);
        
        // Guardar este saludo para la Tiburona (Persistent Storage)
        // Key compuesta: cada usuario tiene su propia key
        // Sobrescribir el saludo anterior (solo último saludo)
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);
        
        // Extender TTL de los datos de la Tiburona
        // Datos específicos de usuario - críticos (no deben expirar)
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        // Extender TTL del instance storage
        // Configuración global y contador
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        // Retornar saludo personalizado
        Ok(Symbol::new(&env, "Hola"))
    }
    
    // ============================================================
    // SECCIÓN 3: FUNCIONES DE CONSULTA
    // ============================================================
    
    /// Obtiene el contador total de saludos
    /// 
    /// No requiere autenticación (lectura pública).
    /// 
    /// # Retorna
    /// `u32` - Contador total de saludos (0 si nunca se ha saludado)
    /// 
    /// # Nota
    /// Retorna `u32` (no `Result`) porque nunca falla.
    /// `.unwrap_or(0)` maneja el caso de no existencia.
    /// 
    /// # Ejemplo
    /// ```rust
    /// let total = get_contador(env);
    /// ```
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)  // Si no existe → 0 (estado inicial válido)
    }
    
    /// Obtiene el último saludo de una Tiburona específica
    /// 
    /// Retorna None si la Tiburona nunca ha saludado.
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `usuario`: Dirección de la Tiburona
    /// 
    /// # Retorna
    /// `Option<Symbol>` - Some(saludo) si existe, None si nunca saludó
    /// 
    /// # Nota
    /// Retorna `Option<Symbol>` (no `unwrap_or`) porque queremos distinguir
    /// entre "no existe" y "existe". El caller decide cómo manejar `None`.
    /// 
    /// # Ejemplo
    /// ```rust
    /// let saludo = get_ultimo_saludo(env, usuario_address);
    /// match saludo {
    ///     Some(s) => println!("Último saludo: {}", s),
    ///     None => println!("Esta Tiburona nunca ha saludado"),
    /// }
    /// ```
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<Symbol> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }
    
    // ============================================================
    // SECCIÓN 4: FUNCIÓN ADMINISTRATIVA
    // ============================================================
    
    /// Resetea el contador a 0
    /// 
    /// Solo la administradora puede ejecutar esta función.
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `caller`: Dirección de quien llama la función
    /// 
    /// # Retorna
    /// `Result<(), Error>` - Ok si éxito, Err si no autorizado o no inicializado
    /// 
    /// # Errores
    /// - `NoInicializado` - Si el contrato no está inicializado
    /// - `NoAutorizado` - Si el caller no es el admin
    /// 
    /// # Ejemplo
    /// ```rust
    /// reset_contador(env, caller_address)
    /// ```
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        // Paso 1: Obtener admin guardado
        // Convertir Option → Result con .ok_or()
        // El operador ? propaga el error automáticamente
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        // Paso 2: Verificar permisos
        // Comparación directa de Address
        // Early return si no autorizado
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        
        // Paso 3: Ejecutar operación privilegiada
        // Si llegamos aquí, caller ES admin
        // Seguro modificar estado
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        // Extender TTL después de modificar
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }
    
    // ============================================================
    // FUNCIONES HELPER (para testing)
    // ============================================================
    
    /// Obtener admin (helper para testing)
    /// 
    /// # Retorna
    /// `Result<Address, Error>` - Ok con admin si existe, Err si no inicializado
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)
    }
}

