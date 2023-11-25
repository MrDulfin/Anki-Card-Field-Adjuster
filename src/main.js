// import { invoke } from '@tauri-apps/api/tauri'

const invoke = window.__TAURI__.invoke;


    let button = document.getElementById("button")
    button.addEventListener("click", greet)


    async function hello() {
      document.getElementById("yourMom").innerText = "hello";
    }

    async function greet() {
      invoke('string').then((message) => document.getElementById("yourMom").innerText = message);
    }

    async function theThird() {
      invoke('my_custom_command').then((message) => console.log(message))
    }