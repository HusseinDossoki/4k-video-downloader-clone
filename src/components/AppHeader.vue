<template>
  <header>
    <div class="link-container">
      <div class="link past-link">
          <p class="link-icon">+</p>
          <p class="link-text">Past Link</p>
        </div>
        <div class="link smart-mode">
          <p class="link-icon">💡</p>
          <p class="link-text">Smart Mode</p>
        </div>
        <div class="link preferences">
          <p class="link-icon">⚙️</p>
          <p class="link-text">Preferences</p>
        </div>
    </div>
  </header>
</template>
  
<script setup>

</script>
  
<style scoped>
header {
  height: 57px;
  top: 0;
  width: 100%;
  background-color: #323130;
  color: rgba(238, 238, 238, 0.646);
}

.link-container {
  display: flex;
}
.link {
  margin-top: 3px;
  padding-left: 10px;
  padding-right: 10px;
  cursor: pointer;
}

.link .link-icon {
  color: white;
  border-radius: 50%;
  display: block;
  width: 30px;
  height: 30px;
  text-align: center;
  margin: auto;
  font-size: 20px;
  font-weight: bold;
}

.link .link-text {
  margin-top: 2px;
  text-align: center;
  font-size: 12px;
}

.past-link .link-icon {
  background-color: rgb(7, 155, 7);
}
.smart-mode .link-icon {
  background-color: transparent;
}
.preferences {
  margin-left: auto;
}
</style>
  