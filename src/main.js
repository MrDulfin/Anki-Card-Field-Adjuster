// import { invoke } from '@tauri-apps/api/tauri'

const invoke = window.__TAURI__.invoke;


let button = document.getElementById("button")
button.addEventListener("click", greet)


async function hello() {
  document.getElementById("yourMom").innerText = "hello";
}

async function greet() {
  invoke('get_decks').then((message) => document.getElementById("yourMom").innerText = message);
}

async function theThird() {
  invoke('my_custom_command').then((message) => console.log(message))
}


/* When the user clicks on the button,
toggle between hiding and showing the dropdown content */
function myFunction() {
  document.getElementById("myDropdown").classList.toggle("show");
}

// Close the dropdown menu if the user clicks outside of it
window.onclick = function(event) {
  if (!event.target.matches('.dropbtn')) {
    var dropdowns = document.getElementsByClassName("dropdown-content");
    var i;
    for (i = 0; i < dropdowns.length; i++) {
      var openDropdown = dropdowns[i];
      if (openDropdown.classList.contains('show')) {
        openDropdown.classList.remove('show');
      }
    }
  }
} 