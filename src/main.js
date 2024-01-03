// import { invoke } from '@tauri-apps/api/tauri'

const invoke = window.__TAURI__.invoke;


window.addEventListener("load", get_decks);
window.addEventListener("load", toggleShow);
window.addEventListener("load", poll);
window.addEventListener("load", get_models);
// document.getElementById('submitQuery').addEventListener("click", sendQuery);

const formRes = document.getElementById('query');

var query_deck;

var decks_got = false;
var deck_chosen = false;

model_count = 0;
async function get_models() {

  invoke('find_models').then((models) => {
    models.forEach(model => {
      let list = document.getElementById("dropList2")
      let btn = document.createElement('button');

      btn.innerText = model[0];
      btn.setAttribute('id', model[0]);
      btn.setAttribute('class', "models");
      // btn.addEventListener("click", inputDeck);
      list.appendChild(btn);
      console.log(model[0]);
    })
  })
  // var results = [];

  // for (var i = 0; i < arr1.length; i++) {
  //     if (arr2.indexOf(arr1[i]) !== -1) {
  //         results.push(arr1[i]);
  //     }
  // }
}

async function poll() {
  function lop() {
    invoke('poll_count').then((count) => {
      console.log(count);
    } )
    setTimeout(executeFunction, 5000);
  }
  lop();
}

async function get_decks() {
  if(decks_got == false) {
    invoke('get_decks').then((decks) => {
    console.log(decks.length);
    listDecks(decks);
    decks_got = true;
  });
  }else {
    showError("Decks already gotten! pick one.");
  }
}

function listDecks(decks) {
  let list = document.getElementById("dropList")
  decks.forEach(deck => {
    if(deck == 'Default') {
      return;
    }

    let btn = document.createElement('button');
    btn.innerText = deck;
    btn.setAttribute('id', deck);
    btn.setAttribute('class', "decks");
    btn.addEventListener("click", inputDeck);
    list.appendChild(btn);
    console.log(deck)

  });
}

let selectedDeck = null;
function inputDeck(deck) {
    query_deck = deck.target.id;
    console.log("deck selected: " + query_deck);

    if (selectedDeck){
      selectedDeck.classList.remove("selected");
    }

    deck.target.classList.add("selected");

    selectedDeck = deck.target;
    deck_chosen = true;
}
/* When the user clicks on the button,
toggle between hiding and showing the dropdown content */
function toggleShow() {
  document.getElementById("dropList").classList.toggle("show");
  document.getElementById("dropList2").classList.toggle("show");
}

function showError(error) {
  document.getElementById("wait").innerText = "";
  document.getElementById("error").innerText = error;
}

document.getElementById('query').addEventListener("submit", function(event) {
  event.preventDefault(); // Prevents the default form submission behavior

  var query_cards_with = formRes.elements['cardsWith'].value;
  var query_field = formRes.elements['field'].value;
  var query_replace = formRes.elements['replaceWith'].value;

  if (query_deck && query_field) {
    if(query_cards_with == "") {
      query_cards_with = "";
    }
    console.log("Deck: " + query_deck + "\nCards with: " + query_cards_with + "\nIn Field: " + query_field + "\nReplace with: " + query_replace);
    document.getElementById("error").innerText = "";

    let waiting = document.getElementById("wait");

    waiting.innerText = "Please wait...";
    invoke('query', {
      deck: query_deck,
      cardsWith: query_cards_with,
      field: query_field,
      replace: query_replace,
    }).then((res) => {
      waiting.innerText = res;
      console.log(res)
    })
  } else if (!query_deck) {
    showError("Pick a Deck");
  }else if(!query_field){
    showError("You cannot leave the field empty");
    document.getElementById("field").style.borderColor = "#8B0000";
  }
});

