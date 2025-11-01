#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contracterror,
    Env,
    Address,
    Symbol,
    i128,
    Error,
    symbol_short,
    contracttype,
};

// ============================================================
// PARTE 2: RESULT Y OPTION - MANEJO DE ERRORES
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
    LimiteExcedido = 4,
    SolicitanteNoValida = 5,
    NoInicializado = 6,
}

// ============================================================
// EJEMPLO 1: Transfer Inseguro (QUÉ NO HACER)
// ============================================================

/// Transfer inseguro - EJEMPLO DE QUÉ NO HACER
/// 
/// Este contrato demuestra los problemas comunes
/// al NO validar inputs y estados.
/// 
/// ⚠️ NO USAR EN PRODUCCIÓN - Solo para aprendizaje
#[contract]
pub struct TransferInseguro;

#[contracttype]
#[derive(Clone)]
pub enum DataKeyInseguro {
    Balance(Address),
}

#[contractimpl]
impl TransferInseguro {
    /// ❌ EJEMPLO MALO: Transfer sin validaciones
    /// 
    /// Problemas:
    /// 1. No verifica que `de` autorizó la operación
    /// 2. Panic si `de` no existe (.unwrap())
    /// 3. No valida que `monto` sea positivo
    /// 4. No verifica balance suficiente
    /// 5. No informa por qué falló
    pub fn transfer_inseguro(
        env: Env,
        de: Address,
        para: Address,
        monto: i128,
    ) {
        // 💣 BOMBA 1: .unwrap() puede causar panic
        let balance = env.storage()
            .instance()
            .get(&DataKeyInseguro::Balance(de.clone()))
            .unwrap();  // Si `de` no existe → PANIC
        
        // 💣 BOMBA 2: No valida monto positivo
        // Si monto = -100, entonces balance aumenta en lugar de disminuir
        
        // 💣 BOMBA 3: No valida balance suficiente
        let nuevo_balance = balance - monto;  // Puede ser negativo
        
        // 💣 BOMBA 4: No verifica autenticación
        // Cualquiera puede transferir fondos de otra persona
        
        env.storage()
            .instance()
            .set(&DataKeyInseguro::Balance(de), &nuevo_balance);
    }
    
    /// Helper para testing: establecer balance inicial
    pub fn establecer_balance(env: Env, cuenta: Address, balance: i128) {
        env.storage()
            .instance()
            .set(&DataKeyInseguro::Balance(cuenta), &balance);
    }
    
    /// Helper para testing: obtener balance
    pub fn obtener_balance(env: Env, cuenta: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyInseguro::Balance(cuenta))
            .unwrap_or(0)
    }
}

// ============================================================
// EJEMPLO 2: Transfer Seguro (QUÉ HACER)
// ============================================================

/// Transfer seguro - EJEMPLO DE BUENAS PRÁCTICAS
/// 
/// Este contrato demuestra cómo validar correctamente
/// todas las operaciones antes de modificar el estado.
#[contract]
pub struct TransferSeguro;

#[contracttype]
#[derive(Clone)]
pub enum DataKeySeguro {
    Balance(Address),
}

#[contractimpl]
impl TransferSeguro {
    /// ✅ EJEMPLO BUENO: Transfer con validaciones completas
    /// 
    /// Validaciones en orden correcto:
    /// 1. Autenticación (require_auth)
    /// 2. Validaciones baratas (monto > 0)
    /// 3. Lectura de storage
    /// 4. Validaciones de estado (balance suficiente)
    /// 5. Modificación de storage
    pub fn transfer(
        env: Env,
        de: Address,
        para: Address,
        monto: i128,
    ) -> Result<(), Error> {
        // VALIDACIÓN 1: Autenticación
        // El que llama debe ser el dueño de los fondos
        de.require_auth();
        
        // VALIDACIÓN 2: Monto positivo
        // Verificar ANTES de tocar storage (barato)
        if monto <= 0 {
            return Err(Error::MontoInvalido);
        }
        
        // VALIDACIÓN 3: Balance del remitente
        // Leer storage solo si validaciones anteriores pasaron
        let balance_de = env.storage()
            .instance()
            .get(&DataKeySeguro::Balance(de.clone()))
            .unwrap_or(0);  // Usuario nuevo = balance 0
        
        // VALIDACIÓN 4: Fondos suficientes
        if balance_de < monto {
            return Err(Error::BalanceInsuficiente);
        }
        
        // ✅ TODAS LAS VALIDACIONES PASARON
        // Ahora SÍ es seguro cambiar el estado
        
        let balance_para = env.storage()
            .instance()
            .get(&DataKeySeguro::Balance(para.clone()))
            .unwrap_or(0);
        
        // Actualizar balances con operaciones seguras
        let nuevo_balance_de = balance_de
            .checked_sub(monto)
            .ok_or(Error::BalanceInsuficiente)?;
        
        let nuevo_balance_para = balance_para
            .checked_add(monto)
            .ok_or(Error::LimiteExcedido)?;
        
        env.storage()
            .instance()
            .set(&DataKeySeguro::Balance(de), &nuevo_balance_de);
        
        env.storage()
            .instance()
            .set(&DataKeySeguro::Balance(para), &nuevo_balance_para);
        
        Ok(())  // ✅ Éxito
    }
    
    /// Helper para testing: establecer balance inicial
    pub fn establecer_balance(env: Env, cuenta: Address, balance: i128) {
        env.storage()
            .instance()
            .set(&DataKeySeguro::Balance(cuenta), &balance);
    }
    
    /// Helper para testing: obtener balance
    pub fn obtener_balance(env: Env, cuenta: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeySeguro::Balance(cuenta))
            .unwrap_or(0)
    }
}

// ============================================================
// EJEMPLO 3: Option<T> - Para valores que pueden no existir
// ============================================================

/// Ejemplo de uso de Option<T> para balances
/// 
/// Demuestra cómo manejar correctamente valores
/// que pueden legítimamente no existir.
#[contract]
pub struct OptionEjemplo;

#[contracttype]
#[derive(Clone)]
pub enum DataKeyOption {
    Balance(Address),
    Config(Address),
}

#[contractimpl]
impl OptionEjemplo {
    /// Obtener balance con Option - Ser explícito
    /// 
    /// Retorna Option<i128> porque:
    /// - Usuario nuevo → None (no tiene balance aún)
    /// - Usuario existente → Some(balance)
    pub fn get_balance(env: Env, usuario: Address) -> Option<i128> {
        env.storage()
            .instance()
            .get(&DataKeyOption::Balance(usuario))
    }
    
    /// Obtener balance con default - Para casos donde 0 es válido
    /// 
    /// Usa unwrap_or(0) cuando None significa "balance 0"
    pub fn get_balance_or_zero(env: Env, usuario: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyOption::Balance(usuario))
            .unwrap_or(0)
    }
    
    /// Ejemplo de unwrap_or_else() - Cálculo por defecto
    /// 
    /// Si no existe, calcula el balance inicial basado en lógica
    pub fn get_balance_calculado(env: Env, usuario: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyOption::Balance(usuario.clone()))
            .unwrap_or_else(|| {
                // Lógica para calcular balance inicial
                // Por ejemplo: si es admin, dar 1000, sino 100
                Self::es_admin(&env, &usuario).then_some(1000).unwrap_or(100)
            })
    }
    
    /// Ejemplo de map() - Transformar el valor si existe
    /// 
    /// Duplica el balance si existe, retorna None si no existe
    pub fn get_balance_doble(env: Env, usuario: Address) -> Option<i128> {
        Self::get_balance(env, usuario)
            .map(|b| b * 2)
    }
    
    /// Helper privado para determinar si es admin
    fn es_admin(_env: &Env, _usuario: &Address) -> bool {
        // Lógica simplificada para ejemplo
        // En producción, consultarías storage o config
        false
    }
    
    /// Establecer balance
    pub fn establecer_balance(env: Env, cuenta: Address, balance: i128) {
        env.storage()
            .instance()
            .set(&DataKeyOption::Balance(cuenta), &balance);
    }
}

// ============================================================
// EJEMPLO 4: Sistema de Préstamos con Option y Result
// ============================================================

/// Sistema de préstamos completo
/// 
/// Demuestra el uso combinado de Option y Result:
/// - Option para datos que pueden no existir (límite de crédito)
/// - Result para operaciones que pueden fallar (solicitar préstamo)
#[contract]
pub struct MicroCredito;

#[contracttype]
#[derive(Clone)]
pub enum DataKeyCredito {
    LimiteCredito(Address),
    Balance(Address),
    TotalPrestado(Address),
}

#[contractimpl]
impl MicroCredito {
    /// Obtener límite de crédito (puede no existir)
    /// 
    /// Retorna Option<i128> porque:
    /// - Es válido que una solicitante no tenga límite aún
    /// - El caller decide cómo manejar None
    pub fn get_limite(env: Env, solicitante: Address) -> Option<i128> {
        env.storage()
            .instance()
            .get(&DataKeyCredito::LimiteCredito(solicitante))
    }
    
    /// Establecer límite de crédito
    pub fn establecer_limite(env: Env, solicitante: Address, limite: i128) {
        env.storage()
            .instance()
            .set(&DataKeyCredito::LimiteCredito(solicitante), &limite);
    }
    
    /// Solicitar préstamo (puede fallar)
    /// 
    /// Retorna Result porque:
    /// - Puede fallar por múltiples motivos específicos
    /// - Necesitamos informar exactamente qué salió mal
    pub fn solicitar_prestamo(
        env: Env,
        solicitante: Address,
        monto: i128,
    ) -> Result<(), Error> {
        // 1. Autenticación
        solicitante.require_auth();
        
        // 2. Validar monto
        if monto <= 0 {
            return Err(Error::MontoInvalido);
        }
        
        // 3. Verificar límite (Option → Result)
        // .ok_or() convierte None en Error
        // ? propaga el error automáticamente
        let limite = Self::get_limite(env.clone(), solicitante.clone())
            .ok_or(Error::SolicitanteNoValida)?;
        
        // 4. Verificar que no excede límite
        if monto > limite {
            return Err(Error::LimiteExcedido);
        }
        
        // 5. Procesar préstamo
        Self::ejecutar_prestamo(env, solicitante, monto)?;
        
        Ok(())
    }
    
    /// Ejecutar préstamo (helper privado)
    /// 
    /// Usa el operador ? para propagar errores
    fn ejecutar_prestamo(
        env: Env,
        solicitante: Address,
        monto: i128,
    ) -> Result<(), Error> {
        // Leer balance actual
        let balance_actual = env.storage()
            .instance()
            .get(&DataKeyCredito::Balance(solicitante.clone()))
            .unwrap_or(0);
        
        // Sumar de forma segura
        let nuevo_balance = balance_actual
            .checked_add(monto)
            .ok_or(Error::LimiteExcedido)?;
        
        // Actualizar balance
        env.storage()
            .instance()
            .set(&DataKeyCredito::Balance(solicitante.clone()), &nuevo_balance);
        
        // Actualizar total prestado
        let total_prestado = env.storage()
            .instance()
            .get(&DataKeyCredito::TotalPrestado(solicitante.clone()))
            .unwrap_or(0);
        
        let nuevo_total = total_prestado
            .checked_add(monto)
            .ok_or(Error::LimiteExcedido)?;
        
        env.storage()
            .instance()
            .set(&DataKeyCredito::TotalPrestado(solicitante), &nuevo_total);
        
        Ok(())
    }
    
    /// Obtener balance
    pub fn obtener_balance(env: Env, solicitante: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyCredito::Balance(solicitante))
            .unwrap_or(0)
    }
    
    /// Obtener total prestado
    pub fn obtener_total_prestado(env: Env, solicitante: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyCredito::TotalPrestado(solicitante))
            .unwrap_or(0)
    }
}

// ============================================================
// EJEMPLO 5: Helper Functions para Validación
// ============================================================

/// Funciones helper reutilizables para validación
/// 
/// Demuestra cómo crear funciones de validación
/// que pueden ser reutilizadas en múltiples contratos.
pub struct ValidacionHelper;

impl ValidacionHelper {
    /// Validar monto - Helper reutilizable
    /// 
    /// Verifica:
    /// - Monto > 0
    /// - Monto <= límite máximo
    pub fn validar_monto(monto: i128, limite_maximo: i128) -> Result<(), Error> {
        if monto <= 0 {
            return Err(Error::MontoInvalido);
        }
        
        if monto > limite_maximo {
            return Err(Error::LimiteExcedido);
        }
        
        Ok(())
    }
    
    /// Validar balance suficiente
    pub fn validar_balance(balance: i128, monto: i128) -> Result<(), Error> {
        if balance < monto {
            return Err(Error::BalanceInsuficiente);
        }
        
        Ok(())
    }
}

// ============================================================
// EJEMPLO 6: Conversión Option → Result
// ============================================================

/// Ejemplo de conversión Option a Result
/// 
/// Demuestra cuándo y cómo convertir Option<T>
/// en Result<T, E> cuando None debe tratarse como error.
#[contract]
pub struct ConversionOptionResult;

#[contracttype]
#[derive(Clone)]
pub enum DataKeyConversion {
    Admin,
    Config,
}

#[contractimpl]
impl ConversionOptionResult {
    /// Obtener admin - Option → Result
    /// 
    /// Convierte Option a Result porque:
    /// - Si no hay admin, el contrato no está inicializado
    /// - Esto es un error, no un estado válido
    pub fn obtener_admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKeyConversion::Admin)
            .ok_or(Error::NoInicializado)  // Convierte Option → Result
    }
    
    /// Obtener admin con propagación automática
    /// 
    /// Usa el operador ? para propagar el error automáticamente
    pub fn obtener_admin_y_usar(env: Env) -> Result<(), Error> {
        let admin = Self::obtener_admin(env.clone())?;  // Si es None → retorna Err
        
        // Usar admin...
        // Por ejemplo: verificar que el caller es el admin
        admin.require_auth();
        
        Ok(())
    }
    
    /// Establecer admin
    pub fn establecer_admin(env: Env, admin: Address) {
        env.storage()
            .instance()
            .set(&DataKeyConversion::Admin, &admin);
    }
}

// ============================================================
// EJEMPLO 7: Patrón de Validaciones en Capas
// ============================================================

/// Patrón de validaciones en capas
/// 
/// Demuestra el orden correcto de validaciones:
/// 1. Autenticación (más barata)
/// 2. Validaciones de input (baratas)
/// 3. Validaciones de estado (requieren storage)
/// 4. Ejecución (solo si todo pasó)
#[contract]
pub struct DonacionValidada;

#[contracttype]
#[derive(Clone)]
pub enum DataKeyDonacion {
    Balance(Address),
}

#[contractimpl]
impl DonacionValidada {
    /// Crear donación con validaciones en capas
    pub fn crear_donacion(
        env: Env,
        donante: Address,
        beneficiaria: Address,
        monto: i128,
    ) -> Result<(), Error> {
        // CAPA 1: Autenticación
        donante.require_auth();
        
        // CAPA 2: Validaciones de input (baratas)
        if monto <= 0 {
            return Err(Error::MontoInvalido);
        }
        
        if monto > 1_000_000 {
            return Err(Error::LimiteExcedido);
        }
        
        // CAPA 3: Validaciones de estado (requieren storage)
        let balance = env.storage()
            .instance()
            .get(&DataKeyDonacion::Balance(donante.clone()))
            .unwrap_or(0);
        
        if balance < monto {
            return Err(Error::BalanceInsuficiente);
        }
        
        // CAPA 4: Ejecución (solo si todo pasó)
        Self::ejecutar_donacion(env, donante, beneficiaria, monto)?;
        
        Ok(())
    }
    
    /// Ejecutar donación (helper privado)
    fn ejecutar_donacion(
        env: Env,
        donante: Address,
        beneficiaria: Address,
        monto: i128,
    ) -> Result<(), Error> {
        // Restar del donante
        let balance_donante = env.storage()
            .instance()
            .get(&DataKeyDonacion::Balance(donante.clone()))
            .unwrap_or(0);
        
        let nuevo_balance_donante = balance_donante
            .checked_sub(monto)
            .ok_or(Error::BalanceInsuficiente)?;
        
        // Sumar a beneficiaria
        let balance_beneficiaria = env.storage()
            .instance()
            .get(&DataKeyDonacion::Balance(beneficiaria.clone()))
            .unwrap_or(0);
        
        let nuevo_balance_beneficiaria = balance_beneficiaria
            .checked_add(monto)
            .ok_or(Error::LimiteExcedido)?;
        
        // Actualizar balances
        env.storage()
            .instance()
            .set(&DataKeyDonacion::Balance(donante), &nuevo_balance_donante);
        
        env.storage()
            .instance()
            .set(&DataKeyDonacion::Balance(beneficiaria), &nuevo_balance_beneficiaria);
        
        Ok(())
    }
    
    /// Establecer balance
    pub fn establecer_balance(env: Env, cuenta: Address, balance: i128) {
        env.storage()
            .instance()
            .set(&DataKeyDonacion::Balance(cuenta), &balance);
    }
    
    /// Obtener balance
    pub fn obtener_balance(env: Env, cuenta: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKeyDonacion::Balance(cuenta))
            .unwrap_or(0)
    }
}

