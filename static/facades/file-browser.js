function FacadeModule(owner, basket, parent) {
    document.addEventListener("click", e => {
        if (e.target.tagName === 'A') {
            parent.setHash(e.target.getAttribute('href'));
            e.preventDefault();
        }
    });

    function repoUrl(method, param) {
        return "/api/v1/repo/" + method
            + "?username=" + owner + "&basket=" + basket
            + param;
    }

    function updateView(path) {
        console.log(path);

        var config = {
            credentials: "same-origin"
        };
        fetch(repoUrl("tree_entry", "&commit_ref=HEAD&path=" + path), config)
            // .then(response => response.json())
            // .then(commit => fetch(repoUrl("tree", "&id=" + commit.tree_id + "&prefix=" + path), config))
            .then(response => response.json())
            .then(entry => {
                document.body.innerHTML = "";
                if (path !== "") {
                    var parentPath = path.substring(0, path.lastIndexOf("/"));
                    document.body.innerHTML = "<a href='" + parentPath + "'>Back</a>";
                }

                if (entry.tree) {
                    document.body.innerHTML += "<ul>";
                    entry.tree.entries.forEach(entry => {
                        document.body.innerHTML += "<li><a href='" + path + "/" + entry.filename + "'>"
                            + entry.filename + "</a></li>";
                    });
                    document.body.innerHTML += "</ul>";
                } else {
                    document.body.innerHTML += "<div style='border: 1px solid black; padding: 10px;'>"
                        + atob(entry.blob.content) + "</div>";
                }
            });
    }

    document.body.innerHTML = "Loading...";

    return {
        onHashChange: hash => updateView(hash.slice(1)),
        onLoad: () => {},
    };
}
