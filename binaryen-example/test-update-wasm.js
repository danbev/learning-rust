const fs = require('fs');
const path = require('path');

(async () => {
  try {
    const wasmFilePath = path.join(__dirname, 'config.wasm');
    const wasmBuffer = fs.readFileSync(wasmFilePath);
    const wasmModule = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(wasmModule, {});

    const memory = instance.exports['mem'];
    console.log('Memory:', memory);

    const func_name = "inf:wasi/config#get-name";
    const func = instance.exports[func_name];
    if (typeof func === 'function') {
      const result = func();
      console.log(`${func_name} result: ${result}`);
    } else {
      console.error(`${func_name} function not found in WASM module`);
    }

  } catch (error) {
    console.error('Failed to load the WASM module:', error);
  }

})();

