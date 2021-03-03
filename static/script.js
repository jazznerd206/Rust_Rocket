console.log("I'm a dom.")

let body = document.getElementById('root');
let tag = document.createElement('h2');
tag.innerHTML = 'Hello World';
body.appendChild(tag);