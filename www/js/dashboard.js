
function init() {
  // Get the modal
  var modal = document.getElementById('myModal');

  // When the user clicks the button, open the modal 
  $('#create-event').click(function () {
    modal.style.display = "block";
  });

  // When the user clicks on <span> (x), close the modal
  $('.close').click(function () {
    modal.style.display = "none";
  });

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