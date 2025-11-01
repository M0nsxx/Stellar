import { BASE_FEE } from '@stellar/stellar-sdk';

// Constantes
const STROOPS_POR_XLM = 10000000; // 1 XLM = 10,000,000 stroops
const FEE_POR_OPERATION = BASE_FEE; // 100 stroops por operación
const FEE_POR_TRANSACCION = BASE_FEE; // 100 stroops base por transacción

// Función para calcular costo total
function calcularCostoTotal(numTransacciones, opsPerTx) {
  // Fee por transacción = fee base + (fee por operación × número de operaciones)
  const feePorTx = FEE_POR_TRANSACCION + (FEE_POR_OPERATION * opsPerTx);
  
  // Fee total en stroops
  const feeTotalStroops = feePorTx * numTransacciones;
  
  // Convertir a XLM
  const feeTotalXLM = feeTotalStroops / STROOPS_POR_XLM;
  
  return {
    numTransacciones,
    opsPerTx,
    feePorTx: feePorTx / STROOPS_POR_XLM,
    feeTotalXLM,
    feeTotalStroops,
    feePorTxStroops: feePorTx
  };
}

// Función para formatear resultados
function mostrarResultados(resultado) {
  console.log('╔════════════════════════════════════════════════╗');
  console.log('💰 CALCULADORA DE FEES - STELLAR');
  console.log('╚════════════════════════════════════════════════╝\n');
  
  console.log('📊 CONFIGURACIÓN:');
  console.log(`   Transacciones: ${resultado.numTransacciones}`);
  console.log(`   Operaciones por transacción: ${resultado.opsPerTx}\n`);
  
  console.log('💵 COSTOS:');
  console.log(`   Fee por transacción: ${resultado.feePorTx.toFixed(7)} XLM (${resultado.feePorTxStroops} stroops)`);
  console.log(`   Fee total: ${resultado.feeTotalXLM.toFixed(7)} XLM (${resultado.feeTotalStroops} stroops)\n`);
  
  // Comparación si es múltiples operaciones vs múltiples transacciones
  if (resultado.opsPerTx > 1) {
    const feeSiFueranSeparadas = calcularCostoTotal(
      resultado.numTransacciones * resultado.opsPerTx,
      1
    );
    
    const ahorro = feeSiFueranSeparadas.feeTotalXLM - resultado.feeTotalXLM;
    
    console.log('💡 COMPARACIÓN:');
    console.log(`   Si fueran ${resultado.numTransacciones * resultado.opsPerTx} transacciones separadas (1 op cada una):`);
    console.log(`   Costo: ${feeSiFueranSeparadas.feeTotalXLM.toFixed(7)} XLM`);
    console.log(`   💰 Ahorro agrupando: ${ahorro.toFixed(7)} XLM (${((ahorro / feeSiFueranSeparadas.feeTotalXLM) * 100).toFixed(2)}%)\n`);
  }
  
  console.log('📚 NOTAS:');
  console.log('   • BASE_FEE = 100 stroops por operación');
  console.log('   • Cada transacción tiene un fee base de 100 stroops');
  console.log('   • Agrupar operaciones en una transacción reduce costos');
  console.log('   • 1 XLM = 10,000,000 stroops\n');
}

// Función para tabla comparativa
function tablaComparativa(maxTransacciones = 10, maxOps = 5) {
  console.log('╔════════════════════════════════════════════════════════════╗');
  console.log('📊 TABLA COMPARATIVA DE FEES');
  console.log('╚════════════════════════════════════════════════════════════╝\n');
  
  console.log('Transacciones | Ops/Tx | Fee/Tx (XLM) | Fee Total (XLM)');
  console.log('─'.repeat(65));
  
  for (let ops = 1; ops <= maxOps; ops++) {
    for (let txs = 1; txs <= maxTransacciones; txs++) {
      const resultado = calcularCostoTotal(txs, ops);
      console.log(
        `${txs.toString().padStart(13)} | ${ops.toString().padStart(6)} | ${resultado.feePorTx.toFixed(7).padStart(13)} | ${resultado.feeTotalXLM.toFixed(7)}`
      );
    }
    console.log('─'.repeat(65));
  }
  console.log('');
}

// Ejecutar según argumentos
const args = process.argv.slice(2);

if (args.includes('--tabla')) {
  const maxTxs = parseInt(args[args.indexOf('--tabla') + 1]) || 10;
  const maxOps = parseInt(args[args.indexOf('--tabla') + 2]) || 5;
  tablaComparativa(maxTxs, maxOps);
} else if (args.length >= 2) {
  const numTransacciones = parseInt(args[0]);
  const opsPerTx = parseInt(args[1]);
  
  if (isNaN(numTransacciones) || isNaN(opsPerTx) || numTransacciones <= 0 || opsPerTx <= 0) {
    console.log('❌ Error: Los argumentos deben ser números positivos');
    console.log('Uso: node calculadora-fees.js [transacciones] [operaciones_por_tx]');
    console.log('Ejemplo: node calculadora-fees.js 100 1\n');
  } else {
    const resultado = calcularCostoTotal(numTransacciones, opsPerTx);
    mostrarResultados(resultado);
  }
} else {
  // Ejemplo por defecto
  console.log('💡 Calculando ejemplo: 100 transacciones con 1 operación cada una\n');
  const resultado = calcularCostoTotal(100, 1);
  mostrarResultados(resultado);
  
  console.log('\n📖 USO:');
  console.log('  node calculadora-fees.js [transacciones] [operaciones_por_tx]');
  console.log('  Ejemplo: node calculadora-fees.js 50 3');
  console.log('\n  --tabla [max_transacciones] [max_operaciones]');
  console.log('  Ejemplo: node calculadora-fees.js --tabla 10 5\n');
}

