
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
  var blankSession = new NewSessionGuess("", "");
  var newSession = deriveNextSession();

  if (blankSession === newSession) { return; }

  $('#new_session_name').val(newSession.name);
  var dateString = newSession.date.toISOString();
  dateString = dateString.substring(0, dateString.length - 1);
  $('#new_session_date').val(dateString);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", setNewSessionValues);
} else {
  setNewSessionValues();
}