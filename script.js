"use strict";

import init, { new_game_web_v1, decode_game_v1 } from "./pkg/sudoku_puzzle_game.js";

function setTheme(level) {
  document.body.classList.remove("easy", "medium", "difficult");
  document.getElementById("game_cnt")
    .classList.remove("easy", "medium", "difficult");

  document.body.classList.add(level);
  document.getElementById("game_cnt").classList.add(level);
}

document.addEventListener("focusin", (e) => {
  if (e.target.tagName !== "INPUT") return;

  const row = e.target.parentElement;
  row.style.background = "rgba(255,255,255,0.05)";

  const index = [...row.children].indexOf(e.target);

  document.querySelectorAll(".game-raws").forEach((r) => {
    if (r.children[index]) {
      r.children[index].style.background = "rgba(0,255,200,0.1)";
    }
  });
});

document.addEventListener("focusout", () => {
  document.querySelectorAll(".game-raws").forEach((row) => {
    row.style.background = "";
    [...row.children].forEach((cell) => {
      cell.style.background = "";
    });
  });
});

let game_table = null;
let game_elements = null;

let input_history = [];
let undo = [];

function push_input_history(elm) {
  input_history.push([elm.value, elm]);
}

function grid_to_dom(grid, mask) {
  const size = grid.length;

  const root = document.createElement("div");
  root.id = "game";

  let elements = [];

  for (let i = 0; i < size; i++) {
    let rowDiv = document.createElement("div");
    rowDiv.className = "game-raws";

    let rowArr = [];

    for (let j = 0; j < size; j++) {
      const input = document.createElement("input");

      input.type = "number";
      input.style.width = "100%";
      input.style.textAlign = "center";

      const val = grid[i][j];
      input.setAttribute("data-is", val);

      if (mask[i][j]) {
        // hidden → editable
        input.addEventListener("change", () => {
          push_input_history(input);
        });
      } else {
        // fixed
        input.value = val;
        input.disabled = true;
      }

      rowArr.push(input);
      rowDiv.appendChild(input);
    }

    elements.push(rowArr);
    root.appendChild(rowDiv);
  }

  return { root, elements };
}
function set_lvl_msg(level) {

  const elm = document.getElementById("d_lvl");
  const cls = ["msg-easy", "msg-medium", "msg-difficult"];
  if (level == "easy") {
    elm.classList.remove(cls[1]);
    elm.classList.remove(cls[2]);
    elm.classList.add(cls[0]);
  } else if (level == "medium") {
    elm.classList.remove(cls[0]);
    elm.classList.remove(cls[2]);
    elm.classList.add(cls[1]);
  } else if (level == "difficult") {
    elm.classList.remove(cls[0]);
    elm.classList.remove(cls[1]);
    elm.classList.add(cls[2]);
  }
  elm.textContent = `Game Level: ${level.replace(/^./, level[0].toUpperCase())}`;
}
init().then(() => {
  document.getElementById("create_new").addEventListener("click", () => {
    const value = document.getElementById("create_new_inp").value.trim();
    if (value === "") {
      return;
    }

    const size = parseInt(value);
    const levelStr = document.getElementById("game_lvl").value;

    const result = new_game_web_v1(size, 100000, levelStr);

    if (!result) {
      alert("Failed to generate game");
      return;
    }

    const level = result[0];
    const grid = result[1];
    const mask = result[2];
    const encodedPuzzle = result[3];
    const encodedSolution = result[4];
    console.log("RESULT:", result);
    console.log("GRID:", grid);
    console.log("MASK:", mask);
    render_game(level, grid, mask);

    // store encoded in URL
    window.location.hash = encodedSolution;
  });
});

function render_game(level, grid, mask) {
  if (game_table) {
    game_table.remove();
  }

  input_history = [];
  undo = [];

  const { root, elements } = grid_to_dom(grid, mask);

  game_table = root;
  game_elements = elements;

  setTheme(level);
  document.getElementById("game_cnt").appendChild(root);
  document.getElementById("ctrls").style.display = "flex";
  set_lvl_msg(level);
}

function load_from_hash() {
  const hash = window.location.hash.replace(/^#/, "");

  if (!hash) return;


  init().then(() => {
    const result = decode_game_v1(hash);
    if (!result) {
      console.warn("Invalid game hash");
      return;
    }

    const level = result[0];
    const grid = result[1];
    const mask = result[2];

    render_game(level, grid, mask);
  });

}

window.addEventListener("load", load_from_hash);
document.getElementById("reset").addEventListener("click", () => {
  if (!game_elements) return;

  input_history = [];
  undo = [];

  game_elements.forEach(row => {
    row.forEach(cell => {
      if (!cell.disabled) {
        cell.value = "";
      }
    });
  });
});

document.getElementById("solve").addEventListener("click", () => {
  if (!game_elements) return;

  game_elements.forEach(row => {
    row.forEach(cell => {
      if (!cell.disabled) {
        cell.value = cell.getAttribute("data-is");
      }
    });
  });
});



document.getElementById("undo").addEventListener("click", () => {
  if (input_history.length === 0) return;

  const [val, elm] = input_history.pop();
  elm.value = "";
  undo.push([val, elm]);
});

document.getElementById("redo").addEventListener("click", () => {
  if (undo.length === 0) return;

  const [val, elm] = undo.pop();
  elm.value = val;
  input_history.push([val, elm]);
});

document.getElementById("varify").addEventListener("click", () => {
  if (!game_elements) return;

  let win = true;
  const n = game_elements.length;
  const expectedSum = (n * (n + 1)) / 2;

  for (const row of game_elements) {
    let set = new Set();
    let sum = 0;

    for (const cell of row) {
      let val = cell.disabled
        ? parseInt(cell.getAttribute("data-is"))
        : parseInt(cell.value);

      if (!val || val < 1 || val > n || set.has(val)) {
        win = false;
      }

      set.add(val);
      sum += val;
    }

    if (set.size !== n || sum !== expectedSum) {
      win = false;
    }
  }

  alert(win ? "You win 🎉" : "Wrong input ❌");
});
