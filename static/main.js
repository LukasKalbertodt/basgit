function prepareFacade(owner, basket) {
    document.addEventListener('DOMContentLoaded', () => {
        let facadeIframe = document.getElementById('facade-iframe');
        if (facadeIframe) {
            facadeIframe.addEventListener('load', () => {
                var observer = new MutationObserver(mutations => {
                    var newHeight = facadeIframe.contentDocument.body.scrollHeight;
                    facadeIframe.style.height = newHeight + 'px';
                });

                var config = {
                    attributes: true,
                    childList: true,
                    characterData: true,
                    subtree: true
                };
                observer.observe(facadeIframe.contentDocument.body, config);

                var onHashChange;
                var parent = {
                    setHash: hash => {
                        location.hash = hash;
                        if (onHashChange) {
                            onHashChange('#' + hash);
                        }
                    },
                };
                var facadeModule = facadeIframe.contentWindow.FacadeModule(owner, basket, parent);

                document.addEventListener("hashchange", () => {
                    console.log("muhhh");
                    facadeModule.onHashChange(location.hash);
                });
                onHashChange = facadeModule.onHashChange;
                facadeModule.onHashChange(location.hash);
                console.log(location.hash);
                facadeModule.onLoad();
            });
        }
    });
}
