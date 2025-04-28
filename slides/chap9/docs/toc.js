// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="slide-1.html"><strong aria-hidden="true">1.</strong> Slide 1</a></li><li class="chapter-item expanded "><a href="slide-2.html"><strong aria-hidden="true">2.</strong> Slide 2</a></li><li class="chapter-item expanded "><a href="slide-3.html"><strong aria-hidden="true">3.</strong> Slide 3</a></li><li class="chapter-item expanded "><a href="slide-4.html"><strong aria-hidden="true">4.</strong> Slide 4</a></li><li class="chapter-item expanded "><a href="slide-5.html"><strong aria-hidden="true">5.</strong> Slide 5</a></li><li class="chapter-item expanded "><a href="slide-6.html"><strong aria-hidden="true">6.</strong> Slide 6</a></li><li class="chapter-item expanded "><a href="slide-7.html"><strong aria-hidden="true">7.</strong> Slide 7</a></li><li class="chapter-item expanded "><a href="slide-8.html"><strong aria-hidden="true">8.</strong> Slide 8</a></li><li class="chapter-item expanded "><a href="slide-9.html"><strong aria-hidden="true">9.</strong> Slide 9</a></li><li class="chapter-item expanded "><a href="slide-10.html"><strong aria-hidden="true">10.</strong> Slide 10</a></li><li class="chapter-item expanded "><a href="slide-11.html"><strong aria-hidden="true">11.</strong> Slide 11</a></li><li class="chapter-item expanded "><a href="slide-12.html"><strong aria-hidden="true">12.</strong> Slide 12</a></li><li class="chapter-item expanded "><a href="slide-13.html"><strong aria-hidden="true">13.</strong> Slide 13</a></li><li class="chapter-item expanded "><a href="slide-14.html"><strong aria-hidden="true">14.</strong> Slide 14</a></li><li class="chapter-item expanded "><a href="slide-15.html"><strong aria-hidden="true">15.</strong> Slide 15</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
