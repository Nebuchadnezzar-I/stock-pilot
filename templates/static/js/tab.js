document.getElementById("page-title").addEventListener("click", () => {
    if (window.innerWidth > 650) return;

    const primary = document.getElementById("primary");
    const secondary = document.getElementById("secondary");

    const primaryHidden = primary.style.display === "none";

    primary.style.display = primaryHidden ? "grid" : "none";
    secondary.style.display = primaryHidden ? "none" : "block";
});
