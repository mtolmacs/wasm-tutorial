import "./style.css";
import typescriptLogo from "./typescript.svg";
import viteLogo from "/vite.svg";
import { setupCounter } from "./counter.ts";
import { greet, Person } from "wasm-on-web";

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

setupCounter(document.querySelector<HTMLButtonElement>("#counter")!);

// Create a Person object which is shared between WASM and JS
const person = new Person(2343);

// Call the greet function from WASM with the
greet("John", person);

// Don't forget to free the WASM memory when you're done with the shared object!
person.free();
