import { Keypair } from '@stellar/stellar-sdk';

async function crearMultiplasCuentas(cantidad) {
  const cuentas = [];

  for (let i = 1; i <= cantidad; i++) {
    console.log(`🔐 Creando cuenta ${i}...\n`);

    const pair = Keypair.random();

    try {
      const response = await fetch(
        `https://friendbot.stellar.org/?addr=${pair.publicKey()}`
      );

      const result = await response.json();

      cuentas.push({
        numero: i,
        publicKey: pair.publicKey(),
        secretKey: pair.secret(),
        balance: '10,000 XLM',
        hash: result.hash
      });

      console.log(`✅ Cuenta ${i} fondeada`);
      console.log(`   Public Key: ${pair.publicKey()}`);
      console.log(`   Balance: 10,000 XLM\n`);

    } catch (error) {
      console.error(`❌ Error en cuenta ${i}:`, error.message);
    }
  }

  return cuentas;
}

const misCuentas = await crearMultiplasCuentas(5);
console.log('\n📊 RESUMEN FINAL:');
console.log(JSON.stringify(misCuentas, null, 2));

