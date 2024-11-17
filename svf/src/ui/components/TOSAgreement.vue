<template>
  <div>
    <input type="checkbox" id="agreeToPolicies" name="agreeToPolicies" value="yes" required>
    <label for="agreeToPolicies">
        I have read and agree to the <a href="/term-of-service" target="_blank" class="link">Terms of Service</a> and <a href="/privacy" target="_blank" class="link">Privacy Policy</a>.
    </label>
  </div>
</template>

<style scoped>
div {
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  font-size: 14px;
}

input[type="checkbox"] {
    display: none;
}

input[type="checkbox"] + label {
    position: relative;
    padding-left: 30px;
    cursor: pointer;
    line-height: 20px;
}

input[type="checkbox"] + label::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    width: 1rem;
    height: 1rem;
    border: 2px solid var(--bg-color);
    border-radius: 4px;
    background-color: var(--bg-color-4);
    transition: background-color 0.3s ease, border-color 0.3s ease;
}

input[type="checkbox"]:hover + label::before {
    border-color: var(--fg-color);
}

input[type="checkbox"]:checked + label::before {
    background-color: var(--bg-color-4);
    border-color: var(--fg-color);
}

input[type="checkbox"]:checked + label::after {
    content: '\2713'; /* Unicode check mark */
    position: absolute;
    font-weight: 1000;
    left: 5px;
    font-size: 1rem;
    color: var(--fg-color);
}

label {
    font-weight: 500;
    margin-left: 0.3rem;
    color: var(--fg-color);
}

.link {
  color: var(--fg-color-2);
  font-weight: 500;
  text-decoration: none;
}

.link:hover {
  text-decoration: underline;
}

</style>
