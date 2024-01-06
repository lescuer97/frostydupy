/** @module utils_dom 
    * @param {HTMLFormElement} form - Form from the DOM Event
    *
    * @returns {Object} - Object with key value pairs from the form
    * */
export function formValuesToObject (form) {
  const formData = new FormData(form)
  const data = {}
  for (const key of formData.keys()) {
    data[key] = formData.get(key)
  }
  return data
}
