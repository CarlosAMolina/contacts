document.getElementById("search-button").onclick = function () { 
    const searchTerm = document.getElementById("search-input").value
    const urlPrefix = "http://localhost:3030/contacts?query=";
    const url = urlPrefix.concat(searchTerm);
    fetch(url)
      .then((response) => response.json())
      .then(
          (json) => { 
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
    let result = '<br><p>Number of results: '.concat(json.length, '</p>');
    for(const contact of json) {
        const html_contact = getContactHtml(contact);
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
    result = getHtmlAddArrayIfNotNull(result, json.nicknames, 'Nicknames');
    result = getHtmlAddPhonesArrayIfNotNull(result, json.phones, 'Phones');
    result = getHtmlAddArrayIfNotNull(result, json.categories, 'Categories');
    result = getHtmlAddArrayIfNotNull(result, json.addresses, 'Addresses');
    result = getHtmlAddArrayIfNotNull(result, json.emails, 'Emails');
    result = getHtmlAddHyperlinkArrayIfNotNull(result, json.urls, 'URLs');
    result = getHtmlAddFacebookArrayIfNotNull(result, json.facebook_urls, 'Facebook');
    result = getHtmlAddTwitterArrayIfNotNull(result, json.twitter_handles, 'Twitter');
    result = getHtmlAddInstagramArrayIfNotNull(result, json.instagram_handles, 'Instagram');
    result = getHtmlAddValueIfNotNull(result, json.note, 'Note');
    return result;
}

function getHtmlAddValueIfNotNull(html, value, title) {
    let result = html;
    if ( value != null ) {
        result = result.concat(`
            <p class="title">${title}</p>
            <p>${value}</p>
         `);
    }
    return result;
}

function getHtmlAddArrayIfNotNull(html, array, title) {
    let result = html;
    if ( array.length != 0 ) {
        result = result.concat(`
            <p class="title">${title}</p>
         `);
        if ( array.length == 1 ) {
            const value = array[0];
            result = result.concat(`
                <p>${value}</p>
             `);
        } else {
            for(const value of array) {
                result = result.concat(`
                    <ul>${value}</ul>
                 `);
            }
        }
    }
    return result;
}

function getHtmlAddFacebookArrayIfNotNull(html, array, title) {
    result = getHtmlAddHyperlinkArrayIfNotNull(html, array, title);
    return result;
}

function getHtmlAddTwitterArrayIfNotNull(html, array_handles, title) {
    let array = Array.from(array_handles, (handle) => `https://twitter.com/${handle}`);
    result = getHtmlAddHyperlinkArrayIfNotNull(html, array, title);
    return result;
}

function getHtmlAddInstagramArrayIfNotNull(html, array_handles, title) {
    let array = Array.from(array_handles, (handle) => `https://www.instagram.com/${handle}/`);
    result = getHtmlAddHyperlinkArrayIfNotNull(html, array, title);
    return result;
}

function getHtmlAddHyperlinkArrayIfNotNull(html, array, title) {
    let result = html;
    if ( array.length != 0 ) {
        result = result.concat(`
            <p class="title">${title}</p>
         `);
        if ( array.length == 1 ) {
            const value = array[0];
            result = result.concat(`
                <p><a href="${value}">${value}</a></p>
             `);
        } else {
            for(const value of array) {
                result = result.concat(`
                    <ul><a href="${value}">${value}</a></ul>
                 `);
            }
        }
    }
    return result;
}

function getHtmlAddPhonesArrayIfNotNull(html, array, title) {
    let result = html;
    if ( array.length != 0 ) {
        result = result.concat(`
            <p class="title">${title}</p>
         `);
        if ( array.length == 1 ) {
            const phone = array[0];
            const phone_str = getStrFromPhone(phone);
            result = result.concat(`
                <p>${phone_str}</p>
             `);
        } else {
            for(const phone of array) {
                const phone_str = getStrFromPhone(phone);
                result = result.concat(`
                    <ul>${phone_str}</ul>
                 `);
            }
        }
    }
    return result;
}

function getStrFromPhone(phone) {
    let result = `${phone.value}`;
    if ( phone.description != null ) {
        result = result.concat( ` - ${phone.description}`);
    }
    return result;
}

// TODO
//
// "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\">",
//     image_path_name, "Profile photo", 200, 200
