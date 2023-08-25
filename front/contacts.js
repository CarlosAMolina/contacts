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
    let result = `<p>Number of results: ${json.length}</p>`;
    for(let i = 0; i < json.length; i++) {
        const html_contact = getContactHtml(json[i]);
        result = result.concat(html_contact);
    }
    return result;
}

function getContactHtml(json) {
    let title = json.user_name;
    if ( json.user_surname != null ) {
        title = title.concat(` ${json.user_surname}`);
    }
    let result = `<h1>${title}</h1>`;
    result = getHtmlAddValueIfNotNull(result, json.user_name, 'Name');
    result = getHtmlAddValueIfNotNull(result, json.user_surname, 'Surname');
    result = getHtmlAddValueIfNotNull(result, json.nickname, 'Nickname');
    result = getHtmlAddValueIfNotNull(result, json.phone, 'Phone');
    result = getHtmlAddValueIfNotNull(result, json.phone_description, 'Phone description');
    result = getHtmlAddValueIfNotNull(result, json.category, 'Category');
    result = getHtmlAddValueIfNotNull(result, json.address, 'Address');
    result = getHtmlAddValueIfNotNull(result, json.email, 'Email');
    result = getHtmlAddValueIfNotNull(result, json.url, 'URL');
    result = getHtmlAddValueIfNotNull(result, json.facebook_url, 'Facebook');
    result = getHtmlAddValueIfNotNull(result, json.twitter_handle, 'Twitter');
    result = getHtmlAddValueIfNotNull(result, json.instagram_handle, 'Instagram');
    result = getHtmlAddValueIfNotNull(result, json.note, 'Note');
    result = result.concat(`
        <p class="title">ID</p>
        <p>${json.user_id}</p>
    `);
    return result
}

function getHtmlAddValueIfNotNull(html, value, title) {
    let result = html;
    if ( value != null ) {
        result = result.concat(`
            <p class="title">${title}</p>
            <p>${value}</p>
         `);
    }
    return result
}

// TODO
//
// "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\">",
//     image_path_name, "Profile photo", 200, 200
