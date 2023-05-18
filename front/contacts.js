document.getElementById("search-button").onclick = function () { 
    const searchTerm = document.getElementById("search-input").value
    const urlPrefix = "http://localhost:3030/questions/";
    const url = urlPrefix.concat(searchTerm);
    fetch(url)
      .then((response) => response.json())
      .then(
          (json) => { 
              console.log(json);
              document.getElementById("response-error-div").classList.add('hidden');
              document.getElementById("response-div").classList.remove('hidden');
              document.getElementById("search-result").textContent = JSON.stringify(json);
              document.getElementById("search-result-title").textContent = json.title;
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
