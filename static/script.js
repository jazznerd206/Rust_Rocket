console.log("I'm a dom.")

let title = document.getElementById('title-row');
let tag = document.createElement('h1');
tag.innerHTML = 'This is Rocket.';
title.appendChild(tag);