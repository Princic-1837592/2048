import init, {
    newGame as beNewGame, push, undo, getState, getScore
} from "./pkg/wasm_frontend.js";

init().then(initialize_grid);

function initialize_grid() {
    document.getElementById("score").textContent = getScore().toString();
    const state = JSON.parse(getState());
    let numbers = document.getElementById("numbers");
    numbers.innerHTML = "";
    for (let i = 0; i < state.length; i++) {
        for (let j = 0; j < state[0].length; j++) {
            if (state[i][j] === 0) {
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

function render(push_result, hcoeff, vcoeff) {
    const height = push_result.movements.length;
    const width = push_result.movements[0].length;
    document.querySelectorAll(`#numbers>.position-new`).forEach(e => e.classList.remove("position-new"));
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            if (push_result.movements[i][j] === 0) {
                continue;
            }
            const child = document.querySelector(`#numbers>.position-${i}-${j}`);
            if(child === null) {
                continue;
            }
            child.classList.remove(`position-${i}-${j}`);
            child.classList.add(`position-${i + push_result.movements[i][j] * vcoeff}-${j + push_result.movements[i][j] * hcoeff}`);
        }
    }
    const numbers = document.getElementById("numbers");
    let child = document.createElement("div");
    child.classList.add("number");
    child.classList.add(`number-${push_result.spawned_value}`);
    child.classList.add(`position-${push_result.spawned_row}-${push_result.spawned_col}`);
    child.classList.add(`position-new`);
    child.textContent = push_result.spawned_value;
    numbers.appendChild(child);
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            if (push_result.merged[i][j] === 0) {
                continue;
            }
            console.log(i, j);
            let [first,second] = document.querySelectorAll(`#numbers>.position-${i}-${j}`);
            let child = document.createElement("div");
            child.classList.add("number");
            child.classList.add(`number-${push_result.merged[i][j]}`);
            child.classList.add(`position-${i}-${j}`);
            child.textContent = push_result.merged[i][j];
            numbers.appendChild(child);
            first.remove();
            second.remove();
        }
    }
    console.log(JSON.parse(getState()));
}

function keydown_event(e) {
    let direction = '';
    let hcoeff, vcoeff;
    switch (e.key) {
        case 'ArrowUp':
            direction = 'U';
            hcoeff = 0;
            vcoeff = -1;
            break;
        case 'ArrowDown':
            direction = 'D';
            hcoeff = 0;
            vcoeff = 1;
            break;
        case 'ArrowLeft':
            direction = 'L';
            hcoeff = -1;
            vcoeff = 0;
            break;
        case 'ArrowRight':
            direction = 'R';
            hcoeff = 1;
            vcoeff = 0;
            break;
    }
    if (direction !== '') {
        let result = push(direction);
        if (result !== undefined) {
            render(JSON.parse(result), hcoeff, vcoeff);
        }
    }
}

document.addEventListener('keydown', keydown_event);
