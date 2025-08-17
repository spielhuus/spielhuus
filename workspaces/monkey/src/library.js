mergeInto(LibraryManager.library, {
  update: function (fittest_str, max_fitness, target_len) {
    const target = get_target();
    const fittest = UTF8ToString(fittest_str);
    const container = document.getElementById("result");
    container.innerHTML = "";
    const fragment = document.createDocumentFragment();
    for (let i = 0; i < target.length; i++) {
      const newSpan = document.createElement("span");
      newSpan.textContent = fittest[i];
      if (fittest[i] == target[i]) {
        newSpan.classList.add("good");
      } else {
        newSpan.classList.add("bad");
      }
      fragment.appendChild(newSpan);
    }
    container.appendChild(fragment);

    const elMaxFitness = document.getElementById("maxFitness");
    elMaxFitness.innerText = max_fitness;

    const elRound = document.getElementById("round");
    elRound.innerText = target_len;
  },
});
