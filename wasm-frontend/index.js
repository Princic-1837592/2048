import init, {
    newGame as beNewGame, push, undo, getState, getScore
} from "./pkg/wasm_frontend.js";

init().then(render);

function render() {
    document.getElementById("score").textContent = getScore().toString();
    const state = getState().split(/\r?\n/).map(row => row.split(" "));
    console.log(state);
    let numbers = document.getElementById("numbers");
    numbers.innerHTML = "";
    for (let i = 0; i < state.length; i++) {
        for (let j = 0; j < state[0].length; j++) {
            if (state[i][j] === "0") {
                continue;
            }
            let child = document.createElement("div");
            child.classList.add("number");
            child.classList.add(`number-${state[i][j]}`);
            child.classList.add(`position-${i}-${j}`);
            child.textContent = state[i][j];
            numbers.appendChild(child);
        }
    }
}
