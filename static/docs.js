const ids = ["Basic", "Opcodes", "Params"];
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
    for (const id of ids) {
        let element = element_id(id);
        if (element) {
            element.setAttribute("class", `_${id} hidden`);
        }
    }
}


