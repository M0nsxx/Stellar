import { Horizon } from '@stellar/stellar-sdk';
const Server = Horizon.Server;

const server = new Server('https://horizon-testnet.stellar.org');

// ⚠️ CONFIGURACIÓN: Reemplaza con tu public key
const PUBLIC_KEY = 'GBXXX...'; // Cuenta a monitorear
const INTERVALO_MS = 10000; // Intervalo en milisegundos (10 segundos por defecto)

// Función para consultar balance
async function consultarBalance(publicKey) {
  try {
    const account = await server.loadAccount(publicKey);
    const balance = account.balances[0];
    
    if (balance.asset_type === 'native') {
      const baseReserve = 0.5;
      const subentryReserve = account.subentry_count * 0.5;
      const totalReserve = baseReserve + subentryReserve;
      const available = parseFloat(balance.balance) - totalReserve;
      
      return {
        total: parseFloat(balance.balance),
        bloqueado: totalReserve,
        disponible: available,
        sequenceNumber: account.sequenceNumber()
      };
    }
    
    return null;
  } catch (error) {
    console.error(`❌ Error consultando balance: ${error.message}`);
    return null;
  }
}

// Función principal para monitorear
async function monitorear(publicKey, intervalo) {
  console.log('🔍 INICIANDO MONITOR DE BALANCE\n');
  console.log(`📧 Cuenta: ${publicKey.substring(0, 12)}...`);
  console.log(`⏱️  Intervalo: ${intervalo / 1000} segundos`);
  console.log('💡 Presiona Ctrl+C para detener\n');
  console.log('═'.repeat(50) + '\n');
  
  let balanceAnterior = null;
  let consultaNum = 0;
  
  const intervaloId = setInterval(async () => {
    consultaNum++;
    const timestamp = new Date().toLocaleTimeString();
    
    const balanceInfo = await consultarBalance(publicKey);
    
    if (balanceInfo) {
      console.log(`[${timestamp}] Consulta #${consultaNum}`);
      console.log(`   💰 Balance Total: ${balanceInfo.total.toFixed(7)} XLM`);
      console.log(`   🔒 Bloqueado: ${balanceInfo.bloqueado.toFixed(7)} XLM`);
      console.log(`   ✅ Disponible: ${balanceInfo.disponible.toFixed(7)} XLM`);
      console.log(`   🔢 Sequence: ${balanceInfo.sequenceNumber}`);
      
      // Detectar cambios en el balance
      if (balanceAnterior !== null) {
        const diferencia = balanceInfo.total - balanceAnterior;
        if (diferencia > 0) {
          console.log(`   📈 Cambio: +${diferencia.toFixed(7)} XLM ⬆️\n`);
        } else if (diferencia < 0) {
          console.log(`   📉 Cambio: ${diferencia.toFixed(7)} XLM ⬇️\n`);
        } else {
          console.log(`   ➡️  Sin cambios\n`);
        }
      } else {
        console.log(`   🆕 Balance inicial\n`);
      }
      
      balanceAnterior = balanceInfo.total;
    } else {
      console.log(`[${timestamp}] Consulta #${consultaNum}`);
      console.log(`   ❌ No se pudo obtener el balance\n`);
    }
    
    console.log('─'.repeat(50));
  }, intervalo);
  
  // Manejar Ctrl+C para detener
  process.on('SIGINT', () => {
    console.log('\n\n🛑 Deteniendo monitor...');
    clearInterval(intervaloId);
    console.log('✅ Monitor detenido\n');
    process.exit(0);
  });
}

// Verificar configuración y ejecutar
if (PUBLIC_KEY !== 'GBXXX...') {
  // Obtener intervalo desde argumentos si se proporciona
  const args = process.argv.slice(2);
  const intervaloPersonalizado = args[0] ? parseInt(args[0]) * 1000 : INTERVALO_MS;
  
  monitorear(PUBLIC_KEY, intervaloPersonalizado);
} else {
  console.log('⚠️  CONFIGURACIÓN REQUERIDA:');
  console.log('1. Reemplaza PUBLIC_KEY con la public key que quieres monitorear\n');
  console.log('Uso:');
  console.log('  node monitor-balance.js [intervalo_en_segundos]');
  console.log('  Ejemplo: node monitor-balance.js 5  (monitorea cada 5 segundos)\n');
}

