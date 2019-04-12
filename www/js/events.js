function init() {
  $('.event-row').click(function (e) {
    if ($(window).width() <= 700) {
      var event_id = e.currentTarget.childNodes[1].textContent.trim();
      window.location.href = `/events/${event_id}`;
    }
  });
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}