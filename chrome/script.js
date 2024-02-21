console.log("script is running");
// The goal is to use this to grab images: https://developer.chrome.com/docs/extensions/reference/api/contextMenus

function docListener()
{
    var button = document.getElementById('imgButton')
    button.addEventListener('click', buttonListener)
}

function buttonListener()
{
    document.getElementById("description").innerHTML = "This is where the score and resulting information will show"
}

document.addEventListener('DOMContentLoaded', docListener)