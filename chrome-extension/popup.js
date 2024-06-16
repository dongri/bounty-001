document.addEventListener('DOMContentLoaded', function() {
  const saveButton = document.getElementById('saveButton');
  const loadButton = document.getElementById('loadButton');
  const valueInput = document.getElementById('valueInput');
  const status = document.getElementById('status');

  saveButton.addEventListener('click', function() {
    const value = valueInput.value;
    chrome.storage.sync.set({storedValue: value}, function() {
      status.textContent = 'Value saved!';
    });
  });

  loadButton.addEventListener('click', function() {
    chrome.storage.sync.get('storedValue', function(data) {
      valueInput.value = data.storedValue || '';
      status.textContent = 'Value loaded!';
    });
  });
});
