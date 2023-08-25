document.getElementById("search-button").onclick = function () { 
    const searchTerm = document.getElementById("search-input").value
    const urlPrefix = "http://localhost:3030/contacts?query=";
    const url = urlPrefix.concat(searchTerm);
    fetch(url)
      .then((response) => response.json())
      .then(
          (json) => { 
              console.log(json);
              document.getElementById("response-error-div").classList.add('hidden');
              document.getElementById("response-div").classList.remove('hidden');
              setContactsHtml(json);
          }
      )
      .catch((error) => {
              console.log(error)
              document.getElementById("response-div").classList.add('hidden');
              document.getElementById("response-error-div").classList.remove('hidden');
              document.getElementById("error-output").textContent = error;
          }
      );
    ;
};

function setContactsHtml(json) {
    html = getContactsHtml(json)
    document.getElementById("search-result").innerHTML = html;
}

function getContactsHtml(json) {
    let result = '';
    for(let i = 0; i < json.length; i++) {
        const html_contact = getContactHtml(json[i]);
        result = result.concat(html_contact);
    }
    return result;
}

function getContactHtml(json) {
    let title = json.user_name;
    if ( json.user_surname != null ) {
        title = `${title} ${json.user_surname}`;
    }
    let result = `<h1>${title}</h1>`;
    result = getHtmlAddIfNotNull(result, json.user_name, 'Name');
    result = getHtmlAddIfNotNull(result, json.user_surname, 'Surname');
    result = getHtmlAddIfNotNull(result, json.nickname, 'Nickname');
    result = getHtmlAddIfNotNull(result, json.phone, 'Phone');
    result = getHtmlAddIfNotNull(result, json.phone_description, 'Phone description');
    result = getHtmlAddIfNotNull(result, json.category, 'Category');
    result = getHtmlAddIfNotNull(result, json.address, 'Address');
    result = getHtmlAddIfNotNull(result, json.email, 'Email');
    result = getHtmlAddIfNotNull(result, json.url, 'URL');
    result = getHtmlAddIfNotNull(result, json.facebook_url, 'Facebook');
    result = getHtmlAddIfNotNull(result, json.twitter_handle, 'Twitter');
    result = getHtmlAddIfNotNull(result, json.instagram_handle, 'Instagram');
    result = getHtmlAddIfNotNull(result, json.note, 'Note');
    result = `
        ${result}
        <p class="title">ID</p>
        <p>${json.user_id}</p>
    `
    return result
}

function getHtmlAddIfNotNull(html, value, title) {
    let result = html;
    if ( value != null ) {
        result = `
            ${result}
            <p class="title">${title}</p>
            <p>${value}</p>
         `;
    }
    return result
}

// TODO
//
// "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\">",
//     image_path_name, "Profile photo", 200, 200
