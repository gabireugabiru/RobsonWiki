const versions = [
    {
        version: "0.1.4",
        features: [
            "Added --chars for getting ascci values",
            "Added opcode 16 for random numbers",
            "Added type conversion"
        ],
        changes: [
            "Now error of invalid flags to show all valid flags"
        ]
    },
    {
        version: "0.1.3",
        changes: [
            "Now --version shows robson's logo"
        ],
        features: [
            "Added --generate for getting string raw values",
            "Added robsons keyword for including other robson files in the robson binary",
        ]
    },
    {
        version: "0.1.2",
        changes: ["Fixed path lowercasing", "Increased cohesion in terminal commands", "Now compiled robsons are placed in ./out", "Terminal operations now are queued, so they will only run when flushed", "Removed some abstractions in terminal commands"],
        features: []
    },
    {
        version: "0.1.1",
        changes: [],
        features: ["Opcode 13 for time operations", "Opcode 14 for flushing terminal", "Opcode 15 for raw terminal manipulation"] 
    },
    {
        version: "0.1.0",
        changes: ["Changed from full interpreted, to an compiled/interpreted", "Up to 300% of performace improvement", "Now for run an .robson you need to use the run flag, or just compile it and run the rbsn"],
        features: ["An unique binary type for robson, rbsn", "Time flag to know runtime of a rbsn"]
    },
    {
        version: "0.0.9",
        features: ["Added penetrou keyword for double pointer"],
        changes: ["Fixed pop function", "Fixed lambeu keyword not working on abreviated pushs"]
    },
    {
        version: "0.0.8",
        changes: ["Changed opcode 2 from substracion to an if_lower statement", "Opcode 1 now has all operations supported"],
        features: []
    },
    {
        version: "0.0.7",
        changes: ["Fixed bug where you couldn't add two floating points or signed ints"],
        features: []
    },
    {
        version: "0.0.6",
        changes: ["Now robson store 32bit data with type safety", "Changed the way robson print color to be compatible with all terminals"],
        features: ["Added syntax for storing floating points and signed integers"]
    }
    ,{
        version: "0.0.5",
        changes: ["Now robson's cli prints colorful outputs"],
        features: ["Added jump aliases"]
    }
]
document.querySelectorAll("div.logo_svg").forEach(a => a.onclick = () => {
    window.location.href = "/";
})

const list = document.querySelector("div.list");

for (const version of versions) {
    const div = document.createElement("div");
    div.classList = "version";

    console.log(version);
    {
        const version_header = document.createElement("span");
        version_header.classList = "version_header";


        const v_number = document.createElement("h3");
        v_number.innerText = version.version;

        const line = document.createElement("span");
        line.classList = "line";

        const extend = document.createElement("span");
        extend.classList = "extend";
        extend.innerText = "+";
        version_header.append(v_number, line, extend);

        const log = document.createElement("div");
        log.classList = "log hide";

        const features_title = document.createElement("h4");
        features_title.innerText = "Added"
        const features = document.createElement("div");
        features.classList = "features";
        let features_html = "";
        for (const feature of version.features) {
            features_html += `<div class="feature">${feature}</div>`;
        }
        features.innerHTML = features_html;

        const changes_title = document.createElement("h4");
        changes_title.innerText = "Changes"

        const changes = document.createElement("div");
        changes.classList = "changes";
        let changes_html = "";
        for (const change of version.changes) {
            changes_html += `<div class="change">${change}</div>`;
        }
        changes.innerHTML = changes_html;


        log.append(features_title, features,changes_title,  changes);

        div.append(version_header, log);
    
        extend.onclick = () => {
            console.log(log.classList);
            if (log.classList.contains("hide")) {
                log.classList = "log animate";
                extend.innerText = "-"
                extend.classList = "extend hide";
            } else {
                log.classList = "log hide animate";
                extend.innerText = "+"
                extend.classList = "extend";

            }
        }

    }
    list.appendChild(div);
}