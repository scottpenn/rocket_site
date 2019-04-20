window.onscroll = function() {scroll()};

function scroll() {
    var scroll_top_body = parseInt(document.body.scrollTop / 1152);
    var scroll_top_document_element = parseInt(document.documentElement.scrollTop / 1152);
    if (scroll_top_body >= 2) {
        document.getElementById("scroll-text").hidden = false;
        document.getElementById("scroll-distance").textContent = scroll_top_body
    } else if (scroll_top_document_element >= 2) {
        document.getElementById("scroll-text").hidden = false;
        document.getElementById("scroll-distance").textContent = scroll_top_document_element
    } else {
        document.getElementById("scroll-text").hidden = true;
    }
}