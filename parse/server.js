import wasm from 'balabala'

async function main() {
  return await wasm.greet();
}

const result = await main();
console.log("【 result 】==>", result);
