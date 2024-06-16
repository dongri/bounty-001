document.addEventListener('DOMContentLoaded', function() {
  const saveButton = document.getElementById('saveButton');
  const loadButton = document.getElementById('loadButton');
  const nameInput = document.getElementById('nameInput');
  const valueInput = document.getElementById('valueInput');
  const status = document.getElementById('status');

  saveButton.addEventListener('click', function() {
    const name = nameInput.value;
    const value = valueInput.value;
    chrome.storage.sync.set({[name]: value}, function() {
      status.textContent = 'Value saved!';
    });
  });

  loadButton.addEventListener('click', function() {
    const name = nameInput.value;
    chrome.storage.sync.get(name, function(data) {
      valueInput.value = data[name] || '';
      status.textContent = 'Value loaded!';
    });
  });
});
