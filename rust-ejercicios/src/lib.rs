#![no_std]

// Módulos de ejercicios
pub mod contador;
pub mod ejercicios_practica;
pub mod traits_ejemplos;
pub mod result_option_ejemplos;
pub mod storage_patterns;
pub mod hello_tiburona;

// Re-exportar contratos principales
pub use contador::ContadorContract;
pub use ejercicios_practica::{
    MysteryFunctions,
    ContadorExtendido,
    ContadorConLimite,
    ContadorConSetValue,
    ContadorConHistorial,
    SistemaVotacion,
    ReputationContract,
};
pub use traits_ejemplos::{
    Donacion,
    Token,
    Votable,
    Ownable,
    DonacionEducacion,
    DonacionSalud,
    MicroCredito,
    PropuestaLey,
    registrar_donacion,
    contar_aprobadas,
};
pub use result_option_ejemplos::{
    Error as ResultOptionError,
    TransferInseguro,
    TransferSeguro,
    OptionEjemplo,
    MicroCredito as MicroCreditoResult,
    ConversionOptionResult,
    DonacionValidada,
    ValidacionHelper,
};
pub use storage_patterns::{
    Error as StorageError,
    ConfiguracionGlobal,
    DatosUsuarios,
    CacheTemporal,
    PlataformaDonaciones,
    GestionUsuario,
    EstrategiaTTL,
    DonacionInfo,
};
pub use hello_tiburona::{
    HelloContract,
    Error as HelloError,
    DataKey as HelloDataKey,
};

// Ejercicios prácticos originales
use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Vec as SorobanVec, 
    String as SorobanString, Symbol,
};

/// Estructura para el contrato de ejercicios
#[contract]
pub struct EjerciciosSoroban;

/// Implementación de todos los ejercicios prácticos
#[contractimpl]
impl EjerciciosSoroban {
    // ============================================================
    // EJERCICIO 5: Función con Vec - Contar mayores a 100
    // ============================================================
    /// Cuenta cuántos números en un Vec son mayores a 100
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `numeros`: Vector de números u32
    ///
    /// # Retorna
    /// Cantidad de números mayores a 100
    ///
    /// # Ejemplos
    /// ```
    /// contar_mayores([50, 150, 200, 80]) → 2
    /// contar_mayores([10, 20, 30]) → 0
    /// ```
    pub fn contar_mayores(env: Env, numeros: SorobanVec<u32>) -> u32 {
        let mut contador: u32 = 0;

        // Iterar sobre cada número en el Vec
        for numero in numeros.iter() {
            if numero > 100 {
                // Incrementar contador de forma segura
                contador = contador
                    .checked_add(1)
                    .expect("Error: overflow en contador");
            }
        }

        contador
    }

    // ============================================================
    // EJERCICIO 6: Validar cantidad con Result
    // ============================================================
    /// Valida que una cantidad esté en el rango permitido
    ///
    /// # Reglas
    /// - Si cantidad == 0: Error "Cantidad no puede ser 0"
    /// - Si cantidad > 1000: Error "Cantidad máxima: 1000"
    /// - Si 1 <= cantidad <= 1000: Ok(cantidad)
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban (necesario para crear String)
    /// - `cantidad`: Número a validar
    ///
    /// # Retorna
    /// Result<u32, String> - Ok con cantidad válida o Err con mensaje
    pub fn validar_cantidad(env: Env, cantidad: u32) -> Result<u32, SorobanString> {
        match cantidad {
            0 => Err(SorobanString::from_str(
                &env,
                "Cantidad no puede ser 0",
            )),
            1..=1000 => Ok(cantidad),
            _ => Err(SorobanString::from_str(
                &env,
                "Cantidad máxima: 1000",
            )),
        }
    }

    /// Procesa un depósito con validación de cantidad
    ///
    /// # Ejemplo de uso de validar_cantidad
    pub fn procesar_deposito(env: Env, cantidad: u32) -> Result<u128, SorobanString> {
        match Self::validar_cantidad(env.clone(), cantidad) {
            Ok(monto_valido) => {
                // Aquí podrías guardar en storage, emitir evento, etc.
                env.events().publish(
                    (symbol_short!("deposit"),),
                    monto_valido,
                );
                Ok(monto_valido as u128)
            }
            Err(mensaje) => Err(mensaje),
        }
    }

    // ============================================================
    // EJERCICIO 7: TokenInfo (estructura para el ejercicio)
    // ============================================================
    // Nota: En Soroban real, usarías #[contracttype] para structs
    // Este es un ejemplo simplificado para demostrar borrowing

    /// Procesa información de token de forma eficiente (usando borrowing)
    ///
    /// # Demuestra
    /// - Uso de referencias (&) en lugar de clones
    /// - Borrowing inmutable para solo lectura
    /// - Optimización de memoria
    pub fn procesar_token_info_eficiente(
        _env: Env,
        name: SorobanString,
        _symbol: Symbol,
        total_supply: u128,
    ) -> u128 {
        // Verificar nombre usando referencia (borrowing)
        Self::verificar_nombre(&name);

        // Verificar supply (parámetro simple, no necesita borrowing)
        Self::verificar_supply(total_supply);

        // Retornar el total_supply
        total_supply
    }

    /// Verifica que el nombre no esté vacío (usa borrowing)
    fn verificar_nombre(name: &SorobanString) {
        if name.len() == 0 {
            panic!("Nombre vacío");
        }
    }

    /// Verifica que el supply sea válido
    fn verificar_supply(total_supply: u128) {
        if total_supply == 0 {
            panic!("Supply no puede ser 0");
        }
    }

    // ============================================================
    // EJERCICIO 8: Transferencia de Tokens (DESAFÍO COMPLETO)
    // ============================================================
    /// Transfiere tokens de una cuenta a otra
    ///
    /// # Validaciones
    /// 1. Amount > 0
    /// 2. From tiene balance suficiente
    /// 3. Uso de checked_sub/checked_add para prevenir overflow/underflow
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `from`: Dirección del remitente
    /// - `to`: Dirección del destinatario
    /// - `amount`: Cantidad a transferir (u128)
    ///
    /// # Retorna
    /// Result<(), String> - Ok(()) si éxito, Err con mensaje si falla
    ///
    /// # Ejemplo de uso
    /// ```
    /// transferir(env, from_address, to_address, 1_000_000)
    /// ```
    pub fn transferir(
        env: Env,
        from: Address,
        to: Address,
        amount: u128,
    ) -> Result<(), SorobanString> {
        // Validación 1: Amount mayor a 0
        if amount == 0 {
            return Err(SorobanString::from_str(
                &env,
                "El monto debe ser mayor a 0",
            ));
        }

        // Leer balance del remitente
        // En Soroban, usamos una tupla (Address, Symbol) como key para storage por cuenta
        let key_balance = symbol_short!("balance");
        let balance_from: u128 = env
            .storage()
            .instance()
            .get(&(from.clone(), key_balance.clone()))
            .unwrap_or(0);

        // Validación 2: Balance suficiente
        if balance_from < amount {
            return Err(SorobanString::from_str(
                &env,
                "Balance insuficiente",
            ));
        }

        // Leer balance del destinatario
        let balance_to: u128 = env
            .storage()
            .instance()
            .get(&(to.clone(), key_balance.clone()))
            .unwrap_or(0);

        // Restar de forma segura (prevenir underflow)
        let nuevo_balance_from = balance_from
            .checked_sub(amount)
            .ok_or(SorobanString::from_str(
                &env,
                "Error: underflow al restar",
            ))?;

        // Sumar de forma segura (prevenir overflow)
        let nuevo_balance_to = balance_to
            .checked_add(amount)
            .ok_or(SorobanString::from_str(
                &env,
                "Error: overflow al sumar",
            ))?;

        // Actualizar balances en storage
        let key_balance = symbol_short!("balance");
        env.storage()
            .instance()
            .set(&(from.clone(), key_balance.clone()), &nuevo_balance_from);
        env.storage()
            .instance()
            .set(&(to.clone(), key_balance), &nuevo_balance_to);

        // Emitir evento de transferencia
        env.events().publish(
            (symbol_short!("transfer"),),
            (from, to, amount),
        );

        Ok(())
    }

    /// Obtiene el balance de una cuenta
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `account`: Dirección de la cuenta
    ///
    /// # Retorna
    /// Balance de la cuenta (u128)
    pub fn obtener_balance(env: Env, account: Address) -> u128 {
        let key_balance = symbol_short!("balance");
        env.storage()
            .instance()
            .get(&(account, key_balance))
            .unwrap_or(0)
    }

    /// Establece el balance inicial de una cuenta (útil para testing)
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `account`: Dirección de la cuenta
    /// - `balance`: Balance inicial
    pub fn establecer_balance(env: Env, account: Address, balance: u128) {
        let key_balance = symbol_short!("balance");
        env.storage().instance().set(&(account, key_balance), &balance);
    }

    // ============================================================
    // FUNCIONES AUXILIARES ADICIONALES
    // ============================================================

    /// Ejemplo de uso de checked_add para demostrar operaciones seguras
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `a`: Primer número (u8)
    /// - `b`: Segundo número (u8)
    ///
    /// # Retorna
    /// Result<u8, String> - Ok con suma o Err si hay overflow
    pub fn sumar_segura(env: Env, a: u8, b: u8) -> Result<u8, SorobanString> {
        a.checked_add(b)
            .ok_or(SorobanString::from_str(
                &env,
                "Overflow: resultado demasiado grande",
            ))
    }

    /// Ejemplo de uso de checked_sub para demostrar operaciones seguras
    ///
    /// # Argumentos
    /// - `env`: Entorno de Soroban
    /// - `a`: Primer número (u32)
    /// - `b`: Segundo número (u32)
    ///
    /// # Retorna
    /// Result<u32, String> - Ok con resta o Err si hay underflow
    pub fn restar_segura(env: Env, a: u32, b: u32) -> Result<u32, SorobanString> {
        a.checked_sub(b)
            .ok_or(SorobanString::from_str(
                &env,
                "Underflow: resultado negativo no permitido",
            ))
    }
}

