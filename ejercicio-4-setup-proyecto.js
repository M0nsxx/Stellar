import fs from 'fs';
import { execSync } from 'child_process';

console.log('╔════════════════════════════════════════╗');
console.log('🚀 EJERCICIO 4: CONFIGURACIÓN DE PROYECTO');
console.log('╚════════════════════════════════════════╝\n');

console.log('📋 Pasos para configurar un proyecto Stellar desde cero:\n');

console.log('1️⃣  Crear directorio del proyecto:');
console.log('   mkdir stellar-tarea2');
console.log('   cd stellar-tarea2\n');

console.log('2️⃣  Inicializar proyecto Node.js:');
console.log('   npm init -y\n');

console.log('3️⃣  Instalar Stellar SDK:');
console.log('   npm install @stellar/stellar-sdk\n');

console.log('4️⃣  Configurar package.json con módulos ES6:');
console.log('   Agregar "type": "module" en package.json\n');

// Verificar configuración actual
console.log('╔════════════════════════════════════════╗');
console.log('✅ VERIFICACIÓN DE CONFIGURACIÓN ACTUAL');
console.log('╚════════════════════════════════════════╝\n');

try {
  const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  
  console.log('📦 package.json encontrado:');
  console.log(`   Nombre: ${packageJson.name}`);
  console.log(`   Versión: ${packageJson.version}`);
  
  if (packageJson.type === 'module') {
    console.log('   ✅ "type": "module" configurado correctamente');
  } else {
    console.log('   ⚠️  "type": "module" no está configurado');
  }
  
  if (packageJson.dependencies && packageJson.dependencies['@stellar/stellar-sdk']) {
    const sdkVersion = packageJson.dependencies['@stellar/stellar-sdk'];
    console.log(`   ✅ @stellar/stellar-sdk instalado: ${sdkVersion}`);
  } else {
    console.log('   ⚠️  @stellar/stellar-sdk no está instalado');
  }
  
  console.log('\n📝 Scripts disponibles:');
  if (packageJson.scripts) {
    Object.keys(packageJson.scripts).forEach(script => {
      console.log(`   - npm run ${script}`);
    });
  }
  
  console.log('\n✅ Proyecto configurado correctamente!\n');
  
} catch (error) {
  console.error('❌ Error al leer package.json:', error.message);
}

console.log('╔════════════════════════════════════════╗');
console.log('📄 CONTENIDO RECOMENDADO DE package.json');
console.log('╚════════════════════════════════════════╝\n');

const ejemploPackageJson = {
  "name": "stellar-tarea2",
  "version": "1.0.0",
  "description": "Scripts de JavaScript para trabajar con Stellar SDK",
  "type": "module",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [
    "stellar",
    "blockchain",
    "cryptocurrency"
  ],
  "author": "",
  "license": "MIT",
  "dependencies": {
    "@stellar/stellar-sdk": "^latest"
  }
};

console.log(JSON.stringify(ejemploPackageJson, null, 2));
console.log('\n');

