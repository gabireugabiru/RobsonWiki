const ids = ["Basic", "Opcodes", "Params","Aliases", "Overview"];
ids.forEach((id, index, array) => {
    const element = document.getElementById(id);
    if (element) {
        element.onclick =() => {
            hide_sections();
            const section = element_id(id); 
            if (section) {
                section.setAttribute("class", `_${id}`);
            }
        }
    }
    const section_button = document.querySelector(`section._${id} > button`);
    if (section_button) {
        section_button.onclick = () => {
            if ((ids.length-1) > index) {
                hide_sections();
                const section = document.querySelector(`section._${array[index + 1]}`);
                if (section) {
                    section.setAttribute("class", `_${array[index+1]}`);
                }
            }
        }
    }
})

const element_id = (id) => document.querySelector(`section._${id}`);

function hide_sections() {
    window.scrollTo({
        left: 0,
        top: 0,
        behavior: "smooth"
    })
    for (const id of ids) {
        let element = element_id(id);
        if (element) {
            element.setAttribute("class", `_${id} hidden`);
        }
    }
}

const logo = document.getElementById("logo");
logo.onclick = () => window.location.href = "/";
const navbar = document.querySelector("navbar");

document.getElementById("outside").onclick = () => {
    navbar.setAttribute("class", "open");
}
document.getElementById("inside").onclick = () => {
    navbar.setAttribute("class", "");
}
