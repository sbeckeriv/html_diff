import("../pkg/index.js")
  .catch(console.error)
  .then(module => {
    console.log(module.diff("hello world!", "goodbye world!"));
  });
