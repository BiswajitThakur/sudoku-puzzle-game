"use strict";

function random_select(elem) {
  if (!Array.isArray(elem)) {
    throw new Error("input elem must be array.");
  }
  const len = elem.length;
  const rand_index = Math.floor(Math.random() * len);
  return elem[rand_index];
}

class MetrixCell {
  constructor(size) {
    let set_elem = new Set();
    for (let i = 1; i <= size; i++) {
      set_elem.add(i);
    }
    this.up = [];
    this.down = set_elem;
  }
  next(left) {
    let selectFrom = new Set();
    for (const elem of this.down) {
      selectFrom.add(elem);
    }
    for (const elem of left) {
      selectFrom.delete(elem);
    }
    for (const elem of this.up) {
      selectFrom.delete(elem);
    }
    if (selectFrom.size === 0) {
      return null;
    }
    let selected_val = random_select([...selectFrom]);
    this.up.push(selected_val);
    left.add(selected_val);
    this.down.delete(selected_val);
    return selected_val;
  }
}

class Metrix {
  constructor(size) {
    let val = [];
    for (let i = 0; i < size; i++) {
      val.push(new MetrixCell(size));
    }
    this.size = size;
    this.raw = 0;
    this.value = val;
  }
  next() {
    if (this.size == this.raw) {
      return null;
    }
    let new_raw = [];
    let left = new Set();
    for (let i = 0; i < this.size; i++) {
      let v = this.value[i].next(left);
      if (!v) {
        throw new Error("Faild to generate value.");
      }
      new_raw.push(v);
    }
    this.raw += 1;
    return new_raw;
  }
}

function create_mx(size) {
  let mx = new Metrix(size);
  let rc = [];
  while (true) {
    try {
      if (mx.size == mx.raw) {
        break;
      }
      let m = mx.next();
      if (!m) {
        mx = new Metrix(size);
        rc = [];
        break;
      }
      rc.push(m);
    } catch (err) {
      mx = new Metrix(size);
      rc = [];
    }
  }
  return rc;
}

function select_rand_from_metrix(size) {
  return [Math.floor(Math.random() * size), Math.floor(Math.random() * size)];
}

function create_node_elm(val) {
  const t_or_f = Math.floor(Math.random() * 3) == 0 ? true : false;
  const attr_name = "data-is";

  if (!t_or_f) {
    const elem = document.createElement("span");
    elem.setAttribute(attr_name, val);
    elem.textContent = val;
    elem.style.width = "100%";
    return elem;
  } else {
    const input = document.createElement("input");
    input.setAttribute(attr_name, val);
    input.style.width = "100%";
    input.style.overflow = "hidden";
    input.style.textAlign = "center";
    return input;
  }
}

let game_table = null;
let game_elements = null;

function createRoot(size) {
  if (size > 13) {
    alert("Size must be less then or equal to 13");
    return;
  }
  const g = document.getElementById("my_game");
  if (game_table) {
    game_table.remove();
  }
  let root = document.createElement("table");
  root.id = "my-table";
  let my_table = create_mx(size);
  game_elements = [];
  my_table.forEach((m) => {
    let tr = document.createElement("tr");
    let tr_arr = [];
    m.forEach((n) => {
      let td = document.createElement("td");
      const e = create_node_elm(n);
      td.appendChild(e);
      tr_arr.push(e);
      tr.appendChild(td);
    });
    root.appendChild(tr);
    game_elements.push(tr_arr);
  });
  game_table = root;
  g.appendChild(root);
}

const my_input = document.querySelector("#root > .my-input");

my_input.addEventListener("keyup", (e) => {
  if (!/^Digit\d+$/.test(e.code)) {
    my_input.value = my_input.value.replace(/\D/g, "");
  }
});

document.querySelector("#root > .create-game").addEventListener("click", () => {
  const value = my_input.value.trim();
  if (/^\s*$/.test(value)) {
    return;
  }
  const user_input = parseInt(value);
  createRoot(user_input);
  // console.log(game_elements);
});

document.querySelector("#root > .reset").addEventListener("click", () => {
  if (!game_elements || !Array.isArray(game_elements)) {
    return;
  }
  game_elements.forEach((m) => {
    m.forEach((n) => {
      if (n.tagName === "INPUT") {
        n.value = "";
      }
    });
  });
});

document.querySelector("#root > .solve").addEventListener("click", () => {
  if (!game_elements || !Array.isArray(game_elements)) {
    return;
  }
  game_elements.forEach((m) => {
    m.forEach((n) => {
      if (n.tagName === "INPUT") {
        n.value = n.getAttribute("data-is");
      }
    });
  });
});

let undo = [];
let redo = [];

document.getElementById("undo").addEventListener("click", () => {
  alert("TODO");
});
document.getElementById("redo").addEventListener("click", () => {
  alert("TODO");
});
