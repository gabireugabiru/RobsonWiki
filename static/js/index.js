document.querySelectorAll("div.curves").forEach(element => {
    load_svg("/static/Curvazinha.svg", element);
});
document.querySelectorAll("div.thumbs").forEach(element => {
    load_svg("/static/Mão.svg", element);
});
const theme_button = document.getElementById("theme"); 
function update_theme_button() {
    if (document.cookie.includes("dark")) {
        theme_button.setAttribute("class", "dark");
    } else {
        theme_button.setAttribute("class", "white");
    }
}
update_theme_button();

let theme = document.cookie.includes("dark") ? "dark" : "white";


document.getElementById("theme").onclick = () => {
    console.log(theme);
    if (theme == "dark") {
        theme = "white";
    } else {
        theme = "dark";
    }
    document.cookie=`theme=${theme}`;
    set_theme()
    update_theme_button();
}

function fetch_vars(callback) {
    fetch("/dynamic/css/vars.css").then(resposne => {
        resposne.text().then(text => {
            callback(text);
        })
    })
}

function set_theme() {
    let style = document.getElementById("new_theme");
    if (!style) {
        fetch_vars(text => {
            const new_style = document.createElement("style");
            new_style.innerText = text;
            new_style.setAttribute("id", "new_theme");
            document.querySelector("head").appendChild(new_style);
        })
    } else {
        fetch_vars(text => {
            style.innerText = text; 
        })
    }
}