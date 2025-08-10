mergeInto(LibraryManager.library, {
  update: function (fittest_str, max_fitness, target_len) {
    // console.log(`JavaScript: 'update' called with index: ${fittest_str}`);
    const selected = document.getElementById("result");
    selected.innerText = UTF8ToString(fittest_str);

    const elMaxFitness = document.getElementById("maxFitness");
    elMaxFitness.innerText = max_fitness;

    const elRound = document.getElementById("round");
    elRound.innerText = target_len;

    const log = document.getElementById("result");
    const newEntry = document.createElement("p");
    newEntry.textContent = UTF8ToString(fittest_str);
    log.append(newEntry);
  },
});
