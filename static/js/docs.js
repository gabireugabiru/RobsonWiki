const ids = ["Basic", "Opcodes", "Params","Aliases", "Overview"];
ids.forEach((id, index, array) => {
    const element = document.getElementById(id);
    if (element) {
        element.onclick =() => {
            hide_sections();
            const section = element_id(id); 
            if (section) {
                section.setAttribute("class", `_${id} appear`);
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
                    section.setAttribute("class", `_${array[index+1]} appear`);
                }
            }
        }
    }
})

const element_id = (id) => document.querySelector(`section._${id}`);

function hide_sections() {
    window.scrollTo({left: 0, top:0});
    for (const id of ids) {
        let element = element_id(id);
        if (element) {
            element.setAttribute("class", `_${id} hidden animate`);
        }
    }
}

const logo = document.getElementById("logo");
logo.onclick = () => window.location.href = "/";
const navbar = document.querySelector("navbar");
let had_open = false;
if (window.innerWidth < 675) {
    logo.setAttribute("class", "logo_svg");
}
document.getElementById("outside").onclick = () => {
    navbar.setAttribute("class", "open");
    if (!had_open) {
        logo.setAttribute("class", "logo_svg");
        logo.setAttribute("class", "logo_svg animate");
        had_open = true;
    }
}
document.getElementById("inside").onclick = () => {
    navbar.setAttribute("class", "");
    logo.setAttribute("class", "logo_svg");
}
