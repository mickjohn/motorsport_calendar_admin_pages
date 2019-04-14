function init() {
  // Get the modal
  var modal = document.getElementById('myModal');
  var createEventButton = document.getElementById('create-event');
  var closeButtons = document.getElementsByClassName('close');

  // When the user clicks the button, open the modal 
  createEventButton.addEventListener('click', event => {
    modal.style.display = "block";
  });

  // When the user clicks any of the close buttons, close it
  for (let i = 0; i < closeButtons.length; i++) {
    closeButtons[i].addEventListener('click', event => {
      modal.style.display = "none";
    });
  }

  // When the user clicks anywhere outside of the modal, close it
  window.onclick = function (event) {
    if (event.target == modal) {
      modal.style.display = "none";
    }
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}