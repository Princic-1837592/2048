import init, {
    newGame as beNewGame, push, undo, getState, getScore
} from "./pkg/wasm_frontend.js";

init().then(_ => render());

function render() {
    document.getElementById("score").textContent = getScore().toString();
}

render();
