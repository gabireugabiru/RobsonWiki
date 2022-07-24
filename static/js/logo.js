function load_svg(path, element) {
    if (element && path) {
        fetch(path).then(response => {
            response.text().then(text => {
                element.innerHTML = text;
            });
        });
    }
    
}
const logo_svg = document.querySelector("div.logo_svg");
load_svg("/static/Robson.svg", logo_svg);

