import init, { Communication } from "/static/robson_web.js";
document.getElementById("logo").onclick = () => window.location.href = "/";
init().then(() => {
    const code = document.getElementById("code");
    const run = document.getElementById("run");
    const formated = document.getElementById("formated");
    const syntax = document.getElementById("syntax");
    const status = document.getElementById("status");
    const output = document.getElementById("output");

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
        status.innerHTML = "Finished"
    }

    code.innerHTML = "";
    syntax.onclick = () => code.focus();

    code.onscroll = (ev) => {
        formated.scrollTo({
            top: ev.target.scrollTop
        });
    }

    code.oninput = (ev) => {
        formated.innerHTML = ev.target.innerHTML
            .replace("\n", "<br>")
            .replace(/(\w+\b:)/g, "<span class='alias'>$1</span>")
            .replace(/(\w*:\b\w+)/g, "<span class='alias'>$1</span>")
            .replace(/(robson)/g, "<span class='keyword'>$1</span>")
            .replace(/(f[0-9]|.[0-9]|[0-9]|i[0-9])/g, "<span class='literal'>$1</span>")
    }

    run.onclick = () => {
        output.innerHTML = "";
        let interpreter = new Communication(code.innerText);
        let is_running = false;
        let has_input_been_handled = false;
        let input = document.getElementById("stdin");
        let enter = document.getElementById("enter");

        enter.onclick = () => {
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
                if (result) {
                    output.innerHTML = output.innerText + result;
                    finished();
                    break;
                }
                const stdout = interpreter.stdout();
                console.log(interpreter.opcode());
                if (interpreter.opcode() == 6 && !has_input_been_handled) {
                    is_running = false;
                    waiting();
                    break;
                }
                if (has_input_been_handled) {
                    has_input_been_handled = false;
                }
                output.innerHTML = stdout;
            }
        }
        start_running();
    }
});