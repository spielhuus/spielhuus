mergeInto(LibraryManager.library, {
  select_solution: function (index) {
    document.getElementById("solution" + index).checked = true;
  },
});
