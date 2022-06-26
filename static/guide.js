const ids = ["Download", "Setting", "Getting"];
ids.forEach(id => {
    const element = document.getElementById(id);
    console.log(id);
    if (element) {
        element.onclick = () => {
            const to_view = document.querySelector(`section._${id}`);
            if (to_view) {
                to_view.scrollIntoView({
                    behavior: "smooth"
                });
            }
        }
    }
})
const logo = document.getElementById("logo");
logo.onclick = () => window.location.href = "/";

