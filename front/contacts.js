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
    return `
        <h1>${json.user_name} ${json.user_surname}</h1>
        <p class="title">Name</p>
        <p>${json.user_name}</p>
        <p class="title">Surname</p>
        <p>${json.user_surname}</p>
        <p class="title">Nickname</p>
        <p>${json.nickname}</p>
        <p class="title">Phone</p>
        <p>${json.phone}</p>
        <p class="title">Phone description</p>
        <p>${json.phone_description}</p>
        <p class="title">Category</p>
        <p>${json.category}</p>
        <p class="title">Address</p>
        <p>${json.address}</p>
        <p class="title">Email</p>
        <p>${json.email}</p>
        <p class="title">URL</p>
        <p>${json.url}</p>
        <p class="title">Facebook</p>
        <p>${json.facebook_url}</p>
        <p class="title">Twitter</p>
        <p>${json.twitter_handle}</p>
        <p class="title">Instagram</p>
        <p>${json.instagram_handle}</p>
        <p class="title">Note</p>
        <p>${json.note}</p>
        <p class="title">ID</p>
        <p>${json.id}</p>
    `
}

// TODO
//
// "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\">",
//     image_path_name, "Profile photo", 200, 200
