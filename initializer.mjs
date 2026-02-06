export default function initializer() {
  let loadingBar = null;
  return {
    onStart: () => {
      loadingBar = document.getElementById("loading-bar");
      console.log("Loading...");
      console.time("trunk-initializer");
    },
    onProgress: ({ current, total }) => {
      if (!total) {
        console.log("Loading...", current, "bytes");
      } else {
        const percentage = Math.round((current / total) * 100);
        loadingBar.style.width = percentage + "%";
        console.log("Loading...", percentage, "%");
      }
    },
    onComplete: () => {
      console.log("Loading done");
      console.timeEnd("trunk-initializer");
    },
    onSuccess: (wasm) => {
      console.log("Loading successful!");
      console.log("WebAssembly: ", wasm);
    },
    onFailure: (error) => {
      console.warn("Loading failed!", error);
    },
  };
}
