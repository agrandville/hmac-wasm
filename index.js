// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, {sha256} from "./pkg/hmac.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init("./pkg/hmac_bg.wasm");

  const addResult = sha256(document.getElementById("secret").value,
    document.getElementById("data").value);

  document.getElementById("hmac").value=addResult;
};

document.getElementById("secret").addEventListener("keyup", runWasm);
document.getElementById("data").addEventListener("keyup", runWasm);

runWasm();