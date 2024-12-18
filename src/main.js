const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let greetInputEl;
let greetMsgEl;

// async function greet(data) {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   window.alert(await invoke("greet", { name: data }));
// }

window.addEventListener("DOMContentLoaded", () => {

  if (document.location.href.indexOf("left") >= 0) {
    for (let btn of document.querySelectorAll("button")) {
      btn.id="b"+btn.textContent;
      btn.addEventListener("click", () => {
        greet(btn.textContent);
      })
      btn.addEventListener("mouseover", () => {
        invoke("hover_left", { index: parseInt(btn.textContent) })
      })
    }
  }
});


let pages = {
  "1": [
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "o",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
  ],
  
  "default": [
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
    "?",
  ]
}

listen('custom-event', (event) => {
  if (document.location.href.indexOf("right") >= 0) {
    var buttons = document.querySelectorAll("button");
    var letters = pages[event.payload] ? pages[event.payload] : pages["default"];
    for (let i in buttons) {

      buttons[i].innerHTML = letters[i];
    }
  }
});