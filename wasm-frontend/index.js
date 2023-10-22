import init, {push, new_game} from "./pkg/wasm_frontend.js";

init().then(initialize_grid);

function new_tile(i, j, value, extra_class) {
    const outer = document.createElement("div");
    outer.classList.add("tile");
    outer.classList.add(`tile-${value}`);
    outer.classList.add(`position-${i}-${j}`);
    if (extra_class) {
        outer.classList.add(extra_class);
    }
    const inner = document.createElement("div");
    inner.classList.add("tile-inner");
    inner.textContent = `${value}`;
    outer.appendChild(inner);
    return outer;
}

function initialize_grid() {
    const seed_area = document.getElementById("seed").value;
    const {board, seed: seed_s} = JSON.parse(new_game(4, 4, 0, seed_area));
    const seed = BigInt(seed_s);
    let numbers = document.getElementById("numbers");
    numbers.innerHTML = "";
    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
            if (board[i][j] === 0) {
                continue;
            }
            const tile = new_tile(i, j, board[i][j], "tile-new");
            numbers.appendChild(tile);
        }
    }
    document.getElementById("current-seed").textContent = `${seed}`;
}

class ToMove {
    constructor(child, oi, oj, ni, nj) {
        this.child = child;
        this.oi = oi;
        this.oj = oj;
        this.ni = ni;
        this.nj = nj;
    }
}

function render(push_result) {
    const height = push_result.transitions.length;
    const width = push_result.transitions[0].length;
    document.querySelectorAll(`.tile.position-new`).forEach(e => e.classList.remove("position-new"));
    document.querySelectorAll(`.tile.tile-merged`).forEach(e => e.classList.remove("tile-merged"));
    document.querySelectorAll(`.tile.to-remove`).forEach(e => e.remove());
    const numbers = document.getElementById("numbers");
    const to_move = [];
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            for (const [oi, oj] of push_result.transitions[i][j]) {
                const child = document.querySelector(`.tile.position-${oi}-${oj}`);
                to_move.push(new ToMove(child, oi, oj, i, j));
            }
        }
    }
    for (const move of to_move) {
        move.child.classList.remove(`position-${move.oi}-${move.oj}`);
        move.child.classList.add(`position-${move.ni}-${move.nj}`);
    }
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            if (push_result.transitions[i][j].length === 2) {
                const old_children = document.querySelectorAll(`.tile.position-${i}-${j}`);
                const new_value = parseInt(old_children[0].querySelector(".tile-inner").textContent) * 2;
                const merged = new_tile(i, j, new_value, "tile-merged");
                numbers.appendChild(merged);
                old_children.forEach(e => e.classList.add("to-remove"));
            }
        }
    }
    const spawned = new_tile(
        push_result.spawned_row,
        push_result.spawned_col,
        push_result.spawned_value,
        "tile-new",
    )
    numbers.appendChild(spawned);
    document.getElementById("score").textContent = `${push_result.new_score}`;
}

function keydown_event(e) {
    let direction = '';
    switch (e.key) {
        case 'ArrowUp':
            direction = 'U';
            break;
        case 'ArrowDown':
            direction = 'D';
            break;
        case 'ArrowLeft':
            direction = 'L';
            break;
        case 'ArrowRight':
            direction = 'R';
            break;
    }
    if (direction !== '') {
        let result = JSON.parse(push(direction));
        if (result !== null) {
            render(result);
        }
    }
}

function only_numbers() {
    this.value = this.value.replace(/[^0-9]/g, '');
}

document.addEventListener('keydown', keydown_event);
document.getElementById("new-game").onclick = initialize_grid;
document.getElementById("seed").oninput = only_numbers;

