document.addEventListener('DOMContentLoaded', () => {
    // Find all secondary headings in the TOC that contain a hidden sub-list
    const expandableItems = document.querySelectorAll('.toc-sidebar > ul > li > ul > li:has(> ul)');

    expandableItems.forEach(item => {
        item.addEventListener('click', function(event) {
            // Ensure the click happened directly on the list item or arrow, 
            // and NOT on a link inside the open sub-sub-heading list
            if (event.target.tagName !== 'A' || event.target.parentElement === this) {
                // Toggle the 'is-open' class on and off
                this.classList.toggle('is-open');
            }
        });
    });
});