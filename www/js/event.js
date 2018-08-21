
var new_session_count = 1;


// Get the Event input fields and convert them to JSON
function getEventJson() {
  var event = {
    sport: document.getElementById('event_sport').value,
    round: parseInt(document.getElementById('event_round').value),
    country: document.getElementById('event_country').value,
    location: document.getElementById('event_location').value
  };
  return event;
}

// Get the Session input fields and convert them to a JSON Array
function getSessionsJson(id_prefix) {
  var sessions = [];
  for (var i = 1;; i++) {

    // Check if session box exists by checking if the name input field exists
    if (document.getElementById(id_prefix + '_name-' + i) === null) {
      break;
    }

    // Gather the fields
    var session = {
      name: document.getElementById(id_prefix + '_name-' + i).value,
      time: document.getElementById(id_prefix + '_time-' + i).value
    };
    sessions.push(session);
  }
  return sessions;
}
//
// Get the Session input fields and convert them to a JSON Array
function getUpdatedSessionsJson() {
  var sessions = [];
  for (var i = 1;; i++) {

    // Check if session box exists by checking if the name input field exists
    if (document.getElementById('session_name-' + i) === null) {
      break;
    }

    // Gather the fields
    var session_id_header = document.getElementById('session_id-' + i);
    var id_string = session_id_header.innerHTML.replace('Session ID: ','');
    var session = {
      id: parseInt(id_string),
      name: document.getElementById('session_name-' + i).value,
      time: document.getElementById('session_time-' + i).value
    };
    sessions.push(session);
  }
  return sessions;
}

// Convert the Event, Sessions and New Sessions to a json object
function convertFormToJson() {
  return {
    updated_event: getEventJson(),
    updated_sessions: getUpdatedSessionsJson('session'),
    new_sessions: getSessionsJson('new_session')
  };
}

function updateEventAndSessions() {
  var body_json = JSON.stringify(convertFormToJson());
  var xhr = new XMLHttpRequest();
  var event_id = window.location.pathname.split('/')[2];
  xhr.open("POST", '/update_events_and_sessions/' + event_id, true);
  xhr.setRequestHeader('Content-type','application/json');
  xhr.onload = function () {
    console.log(xhr);
    console.log(xhr.responseURL)
    console.log(xhr.getAllResponseHeaders())
    console.log(xhr.status)
    console.log(xhr.statusText)
    // window.location.replace(xhr.responseURL);
  };
  xhr.send(body_json);
}

function validateSessionNameInput(e) {
  var inputElement = e.target;
  var nameInputErrorElement = inputElement.parentElement.parentElement.getElementsByClassName('session-name-error')[0]
  if (inputElement.innerHTML.length >= 300) {
    e.target.style.backgroundColor = "#f9c7c7"; 
    nameInputErrorElement.innerHTML = "Name too long! Must be < 300 chars.";
  } else {
    e.target.style.backgroundColor = "#fff"; 
    nameInputErrorElement.innerHTML = "";
  }
}

function validateSessionTimeInput(e) {
  var inputElement = e.target;
  var timeInputErrorElement = inputElement.parentElement.parentElement.getElementsByClassName('session-time-error')[0]
  if (isNaN(Date.parse(e.target.value))) {
    e.target.style.backgroundColor = "#f9c7c7"; 
    timeInputErrorElement.innerHTML = "Error parsing time string";
  } else {
    e.target.style.backgroundColor = "#fff"; 
    timeInputErrorElement.innerHTML = "";
  }
}

function validateForm() {}

// function create_and_submit_hidden_form(json, event_id) {
//   var parent = document.getElementsByTagName("body")[0];
//   var form = document.createElement("form");
//   form.method = "post";
//   form.action = "/update_events_and_sessions/" + event_id;

//   var input = document.createElement("input");
//   input.type = "submit";
//   input.value = json;

//   form.appendChild(input);
//   console.log(parent);
//   console.log(form);

//   parent.appendChild(form);
//   form.submit();
// }

function addNewSession() {
  var parent = document.getElementsByClassName("session-box-section")[0];
  var div = document.createElement("div");
  div.classList.add('session-box');
  div.id = "new_session_div-" + new_session_count;

  header = document.createElement("h3");
  header.textContent = "New Session " + new_session_count;

  session_name_header = document.createElement("h5");
  session_name_header.textContent = "Session Name: ";
  session_name_input = document.createElement("input");
  session_name_input.id = "new_session_name-" + new_session_count;
  session_name_input.classList.add('session-name-input');
  session_name_input.type = "text";
  session_name_input.name = "name-" + new_session_count;
  session_name_input.value = "";
  session_name_header.appendChild(session_name_input);
  // session_name_input.addEventListener('keyup', validateSessionTimeInput);
  // session_name_header.appendChild(session_time_input);

  session_name_error = document.createElement("p");
  session_name_error.textContent = "";
  session_name_error.classList.add("error");
  session_name_error.classList.add("session-name-error");

  session_time_header = document.createElement("h5");
  session_time_header.textContent = "Session Time: ";
  session_time_input = document.createElement("input");
  session_time_input.id = "new_session_time-" + new_session_count;
  session_time_input.classList.add('session-time-input');
  session_time_input.type = "text";
  session_time_input.name = "time-" + new_session_count;
  session_time_input.value = "";
  session_time_input.addEventListener('keyup', validateSessionTimeInput);
  session_time_header.appendChild(session_time_input);

  session_time_error = document.createElement("p");
  session_time_error.textContent = "";
  session_time_error.classList.add("error");
  session_time_error.classList.add("session-time-error");

  div.appendChild(header);
  div.appendChild(document.createElement("br"));
  div.appendChild(session_name_header);
  div.appendChild(session_name_error);
  div.appendChild(session_time_header);
  div.appendChild(session_time_error);

  parent.appendChild(div);
  new_session_count++;
}

document.addEventListener('DOMContentLoaded', function () {
  // Add click listener to add session buttons
  var update_event_buttons = document.getElementsByClassName('update-event-button');
  for( var i = 0; i < update_event_buttons.length; i++ ) {
    var element = update_event_buttons[i];
    // element.addEventListener('click', convertFormToJson);
    element.addEventListener('click', updateEventAndSessions);
  }

  var add_sessions_buttons = document.getElementsByClassName('add-session-button');
  for( var i = 0; i < add_sessions_buttons.length; i++ ) {
    var element = add_sessions_buttons[i];
    element.addEventListener('click', addNewSession);
  }

  // var sessionNameInputs = document.getElementsByClassName('session-name-input');
  // for( var i = 0; i < sessionNameInputs.length; i++ ) {
  //   var element = sessionNameInputs[i];
  //   element.addEventListener('keyup', validateSessionNameInput);
  // }

  var sessionTimeInputs = document.getElementsByClassName('session-time-input');
  for( var i = 0; i < sessionTimeInputs.length; i++ ) {
    var element = sessionTimeInputs[i];
    element.addEventListener('keyup', validateSessionTimeInput);
  }
});
