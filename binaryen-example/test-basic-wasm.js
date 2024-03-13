const fs = require('fs');
const path = require('path');

(async () => {
  try {
    const wasmFilePath = path.join(__dirname, 'sample.wasm');
    const wasmBuffer = fs.readFileSync(wasmFilePath);
    const wasmModule = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(wasmModule, {});

    const memory = instance.exports['mem'];
    const mem_view = new Uint8Array(memory.buffer);
    const decoder = new TextDecoder('utf-8');
    const text = decoder.decode(mem_view);
    console.log('Memory as UTF-8 string:', text);

    const getPrompt = instance.exports['getPrompt'];
    if (typeof getPrompt === 'function') {
      const result = getPrompt();
      //console.log(`result: ${result}`);
      const str = new TextDecoder('utf-8').decode(new Uint8Array(memory.buffer, result, 13));
      console.log(`prompt: ${str}`);
    } else {
      console.error('something function not found in WASM module');
    }

    const getData = instance.exports['getData'];
    console.log(`getData: ${getData()}`);

  } catch (error) {
    console.error('Failed to load the WASM module:', error);
  }

})();

