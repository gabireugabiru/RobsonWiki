import init, { Communication } from "/static/robson_web.js";

const code = document.getElementById("code");
const run = document.getElementById("run");
const formated = document.getElementById("formated");
const syntax = document.getElementById("syntax");
const status = document.getElementById("status");
const output = document.getElementById("output");
const reset_button = document.getElementById("reset");
const local_code = localStorage.getItem("code");
const input = document.getElementById("stdin");
const enter = document.getElementById("enter");

if (navigator.userAgent.includes("Mobile") || navigator.userAgent.includes("Android")) {
    code.setAttribute("style", "color: var(--code-text);");
    formated.setAttribute("style", "display: none;")
}

document.getElementById("logo").onclick = () => window.location.href = "/";

init().then(() => {


    function highlight() {
        formated.innerHTML = code.innerHTML
        .replace("\n", "<br>")
        .replace(/(\w*:\b\w+)/g, "<span class='alias'>$1</span>")
        .replace(/(\w+\b:)/g, "<span class='alias'>$1</span>")
        .replace(/\b(robson)\b/g, "<span class='keyword'>$1</span>")
        .replace(/\b(comeu|fudeu|lambeu|chupou|penetrou)\b/g, "<span class='type'>$1</span>")
        .replace(/([0-9]|\b[f]\w*[0-9]\b|\b[i]\w*[-]\b|\b[.]\w*[0-9]\b)/g, "<span class='literal'>$1</span>")
    }
    function reset() {
        code.innerHTML = ";setting data<br>robson robson robson<br>comeu 100<br>comeu 108<br>comeu 114<br>comeu 111<br>comeu 119<br>comeu 32<br>comeu 111<br>comeu 108<br>comeu 108<br>comeu 101<br>comeu 72<br>;printing<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>robson robson robson robson robson robson robson<br>"
        highlight();
    }

    function scroll_output() {
        output.scrollTo({
            top: output.scrollHeight
        })
    }

    function waiting() {
        status.setAttribute("class", "waiting");
        status.innerHTML = "Waiting";
    }
    function running() {
        status.setAttribute("class", "running");
        status.innerHTML = "Running";
    }
    function finished() {
        status.setAttribute("class", "finished");
        status.innerHTML = "Finished";
    }
    
    if (local_code) {
        code.innerHTML = local_code;
        highlight();
    } else {
        reset();
    }


    reset_button.onclick = () => reset();

    syntax.onclick = () => code.focus();


    code.onscroll = (ev) => {
        formated.scrollTo({
            top: ev.target.scrollTop
        });
    }

    code.oninput = (ev) => {
        highlight();
        localStorage.setItem("code", ev.target.innerHTML);    
    }

    run.onclick = ev => {
        ev.preventDefault();
        output.innerHTML = "";
        let interpreter = new Communication(code.innerText);
        let is_running = false;
        let has_input_been_handled = false;
        let last_pos = 0;

        input.onkeyup = (ev) => {
            if (ev.code == "Enter") {
                enter.click();
            }
        }

        enter.onclick = ev => {
            ev.preventDefault();
            if (!is_running) {
                let value = input.value;
                input.value = "";
                interpreter.set_stdin(value);
                has_input_been_handled = true;
                start_running();
            }
        }

        async function start_running() {
            running();
            is_running = true;
            while (true) {
                const result = interpreter.run_line();
                const pos = interpreter.pos();
                if (result) {
                    output.innerHTML = output.innerText + result;
                    scroll_output();
                    finished();
                    break;
                }
                const stdout = interpreter.stdout();
                if (interpreter.opcode() == 6 && !has_input_been_handled && last_pos === pos) {
                    is_running = false;
                    waiting();
                    input.focus();
                    output.innerHTML = stdout;
                    scroll_output();
                    last_pos = pos;
                    break;
                }
                if (has_input_been_handled) {
                    has_input_been_handled = false;
                }
                last_pos = pos;
                output.innerHTML = stdout;
                scroll_output();
            }
        }
        start_running();
    }
});