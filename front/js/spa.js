// alert("Hi!");

function renderMain() {
  alert("main");
}

function renderForum() {
  alert("forum");
}

function renderWiki() {
  alert("wiki");
}

const pathes = [
  { path: /^\/$/, handler: renderMain },
  { path: /^\/wiki$/, handler: renderWiki },
  { path: /^\/forum$/, handler: renderForum },
  { path: /^\/posts\/(\d+)$/, handler: (id) => renderPost(id) },
];

function routeHandle(path) {
  console.log("routeHandle: ", path);
  for (const p of pathes) {
    const args = path.match(p.path);
    if (args) {
      p.handler(...args.slice(1));
      break;
    }
  }
}

document.addEventListener("click", (e) => {
  const a = e.target.closest("a");

  if (a) {
    e.preventDefault();
    const path = a.getAttribute("href");
    // history.pushState({}, "", path);
    // console.log(location.origin);
    routeHandle(path);
  }
});

window.addEventListener("popstate", () => {
  routeHandle(location.pathname);
});
