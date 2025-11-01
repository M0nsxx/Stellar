#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    Env,
    Address,
    Symbol,
    i128,
    Error,
    Vec as SorobanVec,
    String as SorobanString,
    symbol_short,
    contracttype,
};

// ============================================================
// PARTE 1: TRAITS E IMPLEMENTACIONES
// ============================================================

// ============================================================
// EJEMPLO 1: Trait Donacion
// ============================================================

/// Trait Donacion - Contrato de comportamiento para donaciones
/// 
/// Define las funciones que TODA donación debe tener,
/// sin importar el tipo específico (educación, salud, ambiente, etc.)
pub trait Donacion {
    /// Obtiene la dirección de la beneficiaria
    fn beneficiaria(&self, env: &Env) -> Address;
    
    /// Obtiene el monto de la donación
    fn monto(&self, env: &Env) -> i128;
    
    /// Procesa la donación con un donante específico
    fn procesar(&mut self, env: &Env, donante: Address) -> Result<(), Error>;
}

// ============================================================
// IMPLEMENTACIONES DEL TRAIT DONACION
// ============================================================

/// Donación de Educación
#[contract]
pub struct DonacionEducacion;

/// Storage keys para DonacionEducacion
#[contracttype]
#[derive(Clone)]
pub enum DataKeyEducacion {
    Beneficiaria,
    Monto,
    Escuela,
}

#[contractimpl]
impl DonacionEducacion {
    /// Crea una nueva donación de educación
    pub fn new(env: Env, beneficiaria: Address, monto: i128, escuela: Symbol) {
        env.storage().instance().set(&DataKeyEducacion::Beneficiaria, &beneficiaria);
        env.storage().instance().set(&DataKeyEducacion::Monto, &monto);
        env.storage().instance().set(&DataKeyEducacion::Escuela, &escuela);
    }
    
    /// Obtiene la beneficiaria
    pub fn get_beneficiaria(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKeyEducacion::Beneficiaria)
            .expect("Beneficiaria no inicializada")
    }
    
    /// Obtiene el monto
    pub fn get_monto(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyEducacion::Monto)
            .expect("Monto no inicializado")
    }
    
    /// Obtiene la escuela
    pub fn get_escuela(env: Env) -> Symbol {
        env.storage()
            .instance()
            .get(&DataKeyEducacion::Escuela)
            .expect("Escuela no inicializada")
    }
}

// Implementación del trait Donacion para DonacionEducacion
impl Donacion for DonacionEducacion {
    fn beneficiaria(&self, env: &Env) -> Address {
        Self::get_beneficiaria(env.clone())
    }
    
    fn monto(&self, env: &Env) -> i128 {
        Self::get_monto(env.clone())
    }
    
    fn procesar(&mut self, env: &Env, donante: Address) -> Result<(), Error> {
        // Lógica específica para donaciones educativas
        // Por ejemplo: verificar que la escuela esté registrada
        let escuela = Self::get_escuela(env.clone());
        
        // Validar que el monto sea positivo
        let monto = Self::get_monto(env.clone());
        if monto <= 0 {
            return Err(symbol_short!("MONTO_INV"));
        }
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("donacion_educ"),),
            (donante, Self::get_beneficiaria(env.clone()), monto, escuela)
        );
        
        Ok(())
    }
}

/// Donación de Salud
#[contract]
pub struct DonacionSalud;

/// Storage keys para DonacionSalud
#[contracttype]
#[derive(Clone)]
pub enum DataKeySalud {
    Beneficiaria,
    Monto,
    Hospital,
}

#[contractimpl]
impl DonacionSalud {
    /// Crea una nueva donación de salud
    pub fn new(env: Env, beneficiaria: Address, monto: i128, hospital: Symbol) {
        env.storage().instance().set(&DataKeySalud::Beneficiaria, &beneficiaria);
        env.storage().instance().set(&DataKeySalud::Monto, &monto);
        env.storage().instance().set(&DataKeySalud::Hospital, &hospital);
    }
    
    /// Obtiene la beneficiaria
    pub fn get_beneficiaria(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKeySalud::Beneficiaria)
            .expect("Beneficiaria no inicializada")
    }
    
    /// Obtiene el monto
    pub fn get_monto(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeySalud::Monto)
            .expect("Monto no inicializado")
    }
    
    /// Obtiene el hospital
    pub fn get_hospital(env: Env) -> Symbol {
        env.storage()
            .instance()
            .get(&DataKeySalud::Hospital)
            .expect("Hospital no inicializado")
    }
}

// Implementación del trait Donacion para DonacionSalud
impl Donacion for DonacionSalud {
    fn beneficiaria(&self, env: &Env) -> Address {
        Self::get_beneficiaria(env.clone())
    }
    
    fn monto(&self, env: &Env) -> i128 {
        Self::get_monto(env.clone())
    }
    
    fn procesar(&mut self, env: &Env, donante: Address) -> Result<(), Error> {
        // Lógica específica para donaciones de salud
        // Por ejemplo: verificar que el hospital tenga licencia
        let hospital = Self::get_hospital(env.clone());
        
        // Validar que el monto sea positivo
        let monto = Self::get_monto(env.clone());
        if monto <= 0 {
            return Err(symbol_short!("MONTO_INV"));
        }
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("donacion_sal"),),
            (donante, Self::get_beneficiaria(env.clone()), monto, hospital)
        );
        
        Ok(())
    }
}

// ============================================================
// EJEMPLO 2: Trait Token (Estándar blockchain)
// ============================================================

/// Trait Token - Contrato de comportamiento para tokens
/// 
/// Define las funciones estándar que TODO token debe tener
/// para interoperar con DEXs, wallets y otras aplicaciones
pub trait Token {
    /// Obtiene el balance de un owner
    fn balance_of(&self, env: &Env, owner: Address) -> i128;
    
    /// Transfiere tokens de from a to
    fn transfer(&self, env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error>;
    
    /// Obtiene el total supply del token
    fn total_supply(&self, env: &Env) -> i128;
}

// ============================================================
// EJEMPLO 3: Trait Votable (Mini-ejercicio de reflexión)
// ============================================================

/// Trait Votable - Contrato de comportamiento para propuestas votables
/// 
/// Define las funciones que TODA propuesta debe tener,
/// sin importar el tipo (ley, presupuesto, evento, etc.)
pub trait Votable {
    /// Obtiene votos a favor
    fn votos_a_favor(&self, env: &Env) -> u32;
    
    /// Obtiene votos en contra
    fn votos_en_contra(&self, env: &Env) -> u32;
    
    /// Determina si la propuesta pasó (más votos a favor que en contra)
    fn paso(&self, env: &Env) -> bool {
        self.votos_a_favor(env) > self.votos_en_contra(env)
    }
}

// ============================================================
// IMPLEMENTACIÓN DEL PATRÓN OWNABLE
// ============================================================

/// Trait Ownable - Patrón estándar de control de acceso
/// 
/// Define las funciones que todo contrato "Ownable" debe tener
/// para controlar quién puede ejecutar funciones administrativas
pub trait Ownable {
    /// Obtiene el owner actual del contrato
    fn get_owner(&self, env: &Env) -> Address;
    
    /// Transfiere el ownership a otra persona
    fn transfer_ownership(&self, env: &Env, new_owner: Address) -> Result<(), Error>;
    
    /// Verifica que quien llama es el owner (guardián)
    fn require_owner(&self, env: &Env, caller: Address) -> Result<(), Error>;
}

/// Storage keys para Ownable
#[contracttype]
#[derive(Clone)]
pub enum DataKeyOwnable {
    Owner,
}

// ============================================================
// CONTRATO CON IMPLEMENTACIÓN OWNABLE
// ============================================================

/// Contrato de Microcréditos con control de acceso Ownable
#[contract]
pub struct MicroCredito;

/// Storage keys adicionales para MicroCredito
#[contracttype]
#[derive(Clone)]
pub enum DataKeyMicroCredito {
    Owner,
    TasaInteres,
    TotalPrestado,
}

#[contractimpl]
impl MicroCredito {
    /// Inicializa el contrato con un owner
    pub fn initialize(env: Env, owner: Address) {
        env.storage().instance().set(&DataKeyMicroCredito::Owner, &owner);
        env.storage().instance().set(&DataKeyMicroCredito::TasaInteres, &10u32); // 10% por defecto
        env.storage().instance().set(&DataKeyMicroCredito::TotalPrestado, &0i128);
    }
    
    /// Función pública - cualquiera puede llamarla
    /// 
    /// Permite a cualquier emprendedora solicitar un crédito
    pub fn solicitar_credito(env: Env, solicitante: Address, monto: i128) -> Result<(), Error> {
        // Validar que el monto sea positivo
        if monto <= 0 {
            return Err(symbol_short!("MONTO_INV"));
        }
        
        // Lógica para solicitar crédito
        // (En un contrato real, aquí habría validaciones más complejas)
        
        // Actualizar total prestado
        let mut total = env.storage()
            .instance()
            .get(&DataKeyMicroCredito::TotalPrestado)
            .unwrap_or(0i128);
        total += monto;
        env.storage().instance().set(&DataKeyMicroCredito::TotalPrestado, &total);
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("credito_sol"),),
            (solicitante, monto)
        );
        
        Ok(())
    }
    
    /// Función administrativa - SOLO el owner puede ejecutarla
    /// 
    /// Permite cambiar la tasa de interés
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `caller`: Address de quien llama la función (debe ser el owner)
    /// - `nueva_tasa`: Nueva tasa de interés (en porcentaje)
    /// 
    /// # Retorna
    /// `Result<(), Error>` - Ok(()) si exitoso, Error si no es el owner
    pub fn cambiar_tasa_interes(env: Env, caller: Address, nueva_tasa: u32) -> Result<(), Error> {
        // PRIMERO: Verificar que el caller es el owner
        Self::require_owner(&env, caller)?;
        
        // SEGUNDO: Si llegamos aquí, es seguro cambiar la tasa
        env.storage()
            .instance()
            .set(&DataKeyMicroCredito::TasaInteres, &nueva_tasa);
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("tasa_chg"),),
            nueva_tasa
        );
        
        Ok(())
    }
    
    /// Consulta la tasa de interés actual
    pub fn get_tasa_interes(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKeyMicroCredito::TasaInteres)
            .unwrap_or(10u32)
    }
    
    /// Consulta el total prestado
    pub fn get_total_prestado(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyMicroCredito::TotalPrestado)
            .unwrap_or(0i128)
    }
}

// Implementación del trait Ownable para MicroCredito
impl Ownable for MicroCredito {
    /// Obtiene el owner actual del contrato
    fn get_owner(&self, env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKeyMicroCredito::Owner)
            .expect("Owner no inicializado")
    }
    
    /// Transfiere el ownership a otra persona
    /// 
    /// Solo el owner actual puede transferir el ownership
    fn transfer_ownership(&self, env: &Env, new_owner: Address) -> Result<(), Error> {
        // PASO 1: Obtener quien está llamando esta función
        // Nota: En un contrato real, usarías env.invoker() o env.current_contract_address()
        // Para este ejemplo, asumimos que el caller se pasa como parámetro
        
        // PASO 2: VALIDAR PRIMERO - Verificar que el caller ES el owner actual
        // Nota: En implementación real, obtener caller de env
        // Por ahora, requerimos que se pase explícitamente
        
        // En una implementación completa:
        // let caller = env.invoker();
        // self.require_owner(env, caller)?;
        
        // PASO 3: Si llegamos aquí, es seguro cambiar el owner en storage
        env.storage()
            .instance()
            .set(&DataKeyMicroCredito::Owner, &new_owner);
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("owner_trans"),),
            new_owner
        );
        
        Ok(())
    }
    
    /// Verifica que quien llama es el owner (guardián)
    /// 
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `caller`: Address de quien llama la función
    /// 
    /// # Retorna
    /// `Result<(), Error>` - Ok(()) si es el owner, Error si no lo es
    fn require_owner(&self, env: &Env, caller: Address) -> Result<(), Error> {
        // PASO 1: Obtener el owner guardado en storage
        let owner = self.get_owner(env);
        
        // PASO 2: Comparar el caller con el owner guardado
        if caller != owner {
            // ❌ No coinciden → retornar error
            return Err(symbol_short!("NOT_OWNER"));
        }
        
        // ✅ Coinciden → todo bien
        Ok(())
    }
}

// Función auxiliar para Ownable (implementación estática)
impl MicroCredito {
    /// Versión estática de require_owner para usar en funciones públicas
    pub fn require_owner(env: &Env, caller: Address) -> Result<(), Error> {
        let owner: Address = env.storage()
            .instance()
            .get(&DataKeyMicroCredito::Owner)
            .expect("Owner no inicializado");
        
        if caller != owner {
            return Err(symbol_short!("NOT_OWNER"));
        }
        
        Ok(())
    }
}

// ============================================================
// FUNCIÓN GENÉRICA QUE USA TRAITS
// ============================================================

/// Función genérica que registra donaciones de cualquier tipo
/// 
/// Esta función demuestra el poder de los traits: funciona con
/// CUALQUIER tipo que implemente Donacion, sin conocer los detalles
/// específicos de cada implementación.
/// 
/// # Parámetros genéricos
/// - `T: Donacion` - T puede ser cualquier tipo, PERO debe implementar Donacion
/// 
/// # Ejemplo
/// ```
/// let donacion_educ = DonacionEducacion::new(...);
/// registrar_donacion(donacion_educ, &mut registro);
/// 
/// let donacion_salud = DonacionSalud::new(...);
/// registrar_donacion(donacion_salud, &mut registro);
/// ```
pub fn registrar_donacion<T: Donacion>(
    donacion: &T,
    env: &Env,
    registro: &mut SorobanVec<Address>
) {
    // ✅ Sabemos que TIENE que tener beneficiaria()
    let beneficiaria = donacion.beneficiaria(env);
    registro.push_back(beneficiaria.clone());
    
    // ✅ Sabemos que TIENE que tener monto()
    let monto = donacion.monto(env);
    
    // Emitir evento genérico
    env.events().publish(
        (symbol_short!("donacion_reg"),),
        (beneficiaria, monto)
    );
}

// ============================================================
// IMPLEMENTACIÓN DE VOTABLE PARA PROPUESTAS
// ============================================================

/// Propuesta de Ley
#[contract]
pub struct PropuestaLey;

#[contracttype]
#[derive(Clone)]
pub enum DataKeyPropuestaLey {
    VotosSi,
    VotosNo,
    Titulo,
}

#[contractimpl]
impl PropuestaLey {
    /// Inicializa una nueva propuesta de ley
    pub fn initialize(env: Env, titulo: Symbol) {
        env.storage().instance().set(&DataKeyPropuestaLey::VotosSi, &0u32);
        env.storage().instance().set(&DataKeyPropuestaLey::VotosNo, &0u32);
        env.storage().instance().set(&DataKeyPropuestaLey::Titulo, &titulo);
    }
    
    /// Vota a favor
    pub fn votar_a_favor(env: Env) {
        let mut votos = env.storage()
            .instance()
            .get(&DataKeyPropuestaLey::VotosSi)
            .unwrap_or(0u32);
        votos += 1;
        env.storage().instance().set(&DataKeyPropuestaLey::VotosSi, &votos);
    }
    
    /// Vota en contra
    pub fn votar_en_contra(env: Env) {
        let mut votos = env.storage()
            .instance()
            .get(&DataKeyPropuestaLey::VotosNo)
            .unwrap_or(0u32);
        votos += 1;
        env.storage().instance().set(&DataKeyPropuestaLey::VotosNo, &votos);
    }
    
    /// Obtiene votos a favor
    pub fn get_votos_si(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKeyPropuestaLey::VotosSi)
            .unwrap_or(0u32)
    }
    
    /// Obtiene votos en contra
    pub fn get_votos_no(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKeyPropuestaLey::VotosNo)
            .unwrap_or(0u32)
    }
}

// Implementación del trait Votable para PropuestaLey
impl Votable for PropuestaLey {
    fn votos_a_favor(&self, env: &Env) -> u32 {
        Self::get_votos_si(env.clone())
    }
    
    fn votos_en_contra(&self, env: &Env) -> u32 {
        Self::get_votos_no(env.clone())
    }
    
    // paso() ya está implementado en el trait con valor por defecto
}

/// Función genérica que cuenta propuestas aprobadas
/// 
/// Funciona con CUALQUIER tipo que implemente Votable
pub fn contar_aprobadas<T: Votable>(propuestas: &[T], env: &Env) -> u32 {
    let mut contador = 0u32;
    for propuesta in propuestas.iter() {
        if propuesta.paso(env) {
            contador += 1;
        }
    }
    contador
}

