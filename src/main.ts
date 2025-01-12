import "./style.css";
import typescriptLogo from "./typescript.svg";
import viteLogo from "/vite.svg";
import { setupCounter } from "./counter.ts";
import {
  greet,
  owned_mutable_greet,
  Person,
  remote_instance_param,
} from "wasm-on-web";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <a href="https://vite.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`;

class TSDef {
  constructor(public id: string) {}

  run() {
    console.log(this.id);
  }
}

setupCounter(document.querySelector<HTMLButtonElement>("#counter")!);

// Create a Person object which is shared between WASM and JS
const person = new Person(2343, "John");

// Automatically call the setter on the Rust side
person.name = "Jane";

// Call the greet function from WASM with the
//// greet(person);

// Don't forget to free the WASM memory when you're done with the shared object!
person.free();

// const str = "Hello";
// owned_mutable_greet(str);
// alert(str);

const tsdef = new TSDef("abcd");
remote_instance_param(tsdef);
// alert(tsdef.id);
