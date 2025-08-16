mergeInto(LibraryManager.library, {
  update_counter: function (counter) {
    document.getElementById("counter").innerText = counter;
  },
});
