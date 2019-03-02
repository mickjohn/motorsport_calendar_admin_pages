
class Session {
  constructor(id, name, datetime) {
    this.id = id;
    this.name = name;
    this.datetime = datetime;
  }
}

class NewSessionGuess {
  constructor(name, date) {
    this.name = name;
    this.date = date;
  }
}

function getSporttype() {
  return $('#event_sport').val();
}

function getSessions() {
  var session_rows = $('#sessions_form').find('table').find('tbody').children()
  var sessions = [];
  // Index starts at one to skip the row header
  for (var i = 1; i < session_rows.length; i++) {
    var row = session_rows[i];
    var id = row.children[0].innerText;
    var name = row.children[2].children[0].value;
    var date = new Date(row.children[3].children[0].value);
    sessions.push(new Session(id, name, date));
  }
  return sessions;
}

function deriveNextSession() {
  var sport = getSporttype();
  var sessions = getSessions();

  var retVal = new NewSessionGuess("", "");

  if (sessions.length === 0) {
    var lastSession = new Session("", "");
  } else {
    var lastSession = sessions[sessions.length - 1];
  }

  switch (sport) {
    case "Formula 1":
    case "Indycar":
      switch (lastSession.name) {
        case "Practice 1":
          retVal = new NewSessionGuess("Practice 2", lastSession.datetime);
          break;
        case "Practice 2":
          retVal = new NewSessionGuess("Practice 3", addDay(lastSession.datetime));
          break;
        case "Practice 3":
          retVal = new NewSessionGuess("Qualifying", lastSession.datetime);
          break;
        case "Qualifying":
          retVal = new NewSessionGuess("Race", addDay(lastSession.datetime));
          break;
        default:
          retVal = new NewSessionGuess("Practice 1", "");
          break;
      };
      break;
  }
  return retVal;
}

function addDay(date) {
  date.setDate(date.getDate() + 1);
  return date;
}

function setNewSessionValues() {
  var newSession = deriveNextSession();
  if (newSession.name === "") { return; }

  $('#new-session-name').val(newSession.name);
  if (newSession.date === "") { return; }

  var year = newSession.date.getFullYear();
  var month = ("00" + (newSession.date.getMonth() + 1)).slice(-2);
  var day = ("00" + (newSession.date.getDate() + 1)).slice(-2);
  var dateString = `${year}-${month}-${day}`;
  $('#new-session-date').val(dateString);
}

function dateAndTimeStringsToDate(d, t) {
  var timeString = `${d}T${t}:00`;
  return timeString;
}

function init() {
  $('#new-session-button').click(function () {
    // Show the new session form
    $('#new-session-form').slideDown("slow", function () { });
    // Hide the 'new' buttin and show the cancel button
    $('#new-session-button').fadeOut("fast", function () {
      $('#cancel-new-session-top').fadeIn("fast");
    });
  });

  // If either of the cancel buttons are clicked hide the new session from
  $('#cancel-new-session, #cancel-new-session-top').click(function () {
    $('#new-session-form').slideUp("slow", function () {
      // Hide the top cancel button
      $('#cancel-new-session-top').fadeOut("fast", function () {
        // Show the new session button
        $('#new-session-button').fadeIn("fast");
      });
    });
  });

  $('#new-session-form').submit(function (event) {
    event.preventDefault();
    var date = $('#new-session-date').val();
    var time = $('#new-session-time').val();
    var datetime = dateAndTimeStringsToDate(date, time);
    $('#new-session-datetime').val(datetime);
    this.submit();
  });

  $('#sessions_form').submit(function (event) {
    event.preventDefault();
    var numOfRows = $('#sessions-table-body').children().length;
    for(var i = 0;i < numOfRows; i++) {
      var time = $(`[name='time_in_${i}']`).val();
      var date = $(`[name='date_in_${i}']`).val();
      $(`[name='time_${i}']`).val(dateAndTimeStringsToDate(date, time));
    }
    console.log( $(this).serializeArray () );
    this.submit();
  });

  setNewSessionValues();
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}