function init() {
  var rows = document.getElementsByClassName('event-row');
  for (let i = 0; i < rows.length; i++) {
    rows[i].addEventListener('click', event => {
      if (window.innerWidth <= 700) {
        var event_id = event.currentTarget.childNodes[1].textContent.trim();
        window.location.href = `/events/${event_id}`;
      }
    });
  }
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}