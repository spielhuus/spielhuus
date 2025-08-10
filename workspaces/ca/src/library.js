mergeInto(LibraryManager.library, {
  select_solution: function (index) {
    console.log(`JavaScript: 'select_solution' called with index: ${index}`);
    const selected = document.getElementById("solution" + index);
    selected.checked = true;
  },
});
