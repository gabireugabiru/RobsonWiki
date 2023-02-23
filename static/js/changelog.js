const list = document.querySelectorAll("div.version>span.version_header>span.extend");
for (const el of list) {
    el.onclick = ev => {
        let version_div = ev.target.parentNode.parentNode;

        const extend = version_div.firstChild.lastChild;
        const to_togg = version_div.lastChild;
        console.log(to_togg.classList);
        if (to_togg.classList.contains("hide")) {
            to_togg.classList.remove("hide");
            to_togg.classList.add("animate");
            extend.classList.add("hide");
            extend.innerText = "-";
        } else {
            to_togg.classList.remove("animate");
            to_togg.classList.add("animate");
            to_togg.classList.add("hide");
            extend.classList.remove("hide");
            extend.innerText = "+";

        }
    }
}