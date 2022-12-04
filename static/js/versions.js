const versions = [
    {
        version: "0.0.5",
        windows: "/static/versions/0.0.5vWindows.zip",
        linux: "/static/versions/0.0.5vLinux.zip"
    },
    {
        version: "0.0.6",
        windows: "/static/versions/0.0.6vWindows.zip",
        linux: "/static/versions/0.0.6vLinux.zip"
    },
    {
        version: "0.0.7",
        windows: "/static/versions/0.0.7vWindows.zip",
        linux: "/static/versions/0.0.7vLinux.zip"
    },
    {
        version: "0.0.8",
        windows: "/static/versions/0.0.8vWindows.zip",
        linux: "/static/versions/0.0.8vLinux.zip"
    },
    {
        version: "0.0.9",
        windows: "/static/versions/0.0.9vWindows.zip",
        linux: "/static/versions/0.0.9vLinux.zip"
    },
    {
        version: "0.1.0",
        windows: "/static/versions/0.1.0vWindows.zip",
        linux: "/static/versions/0.1.0vLinux.zip"
    },
    {
        version: "0.1.1",
        windows: "/static/versions/0.1.1vWindows.zip",
        linux: "/static/versions/0.1.1vLinux.zip"
    },
    {
        version: "0.1.2",
        windows: "/static/versions/0.1.2vWindows.zip",
        linux: "/static/versions/0.1.2vLinux.zip"

    },
    {
        version: "0.1.3",
        windows: "/static/versions/0.1.3vWindows.zip",
        linux: "/static/versions/0.1.3vLinux.zip"
    },
    {
        version: "0.1.4",
        windows: "/static/versions/0.1.4vWindows.zip",
        linux: "/static/versions/0.1.4vLinux.zip"
    }
];
const list = document.getElementById("list");
if (list) {
    for (const version of versions) {
        const child = document.createElement("li");
        child.innerHTML = 
        `<h3>${version.version}</h3>
        <a href="${version.windows}">Windows</a>
        <a href="${version.linux}">Linux</a>`;
        list.appendChild(child);
    }
}
