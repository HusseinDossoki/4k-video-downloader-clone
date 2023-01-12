<template>
  <div class="p-4">
    <section class="d-flex">
      <p>Speed</p>
      <div class="toggles w-100">
        <select class="w-100 mb-1">
          <option value="1">Unlimited</option>
          <option value="1">50 Mbps</option>
        </select>
        <small>
          You can limit the download speed if you have a slow internet connection. Please note this setting will be
          reset with the app relaunch.
        </small>
      </div>
    </section>
    <hr>
  </div>
</template>

<script setup>
</script>

<style scoped lang="scss">
section {
  font-size: 13px;
  color: rgb(234, 231, 231);

  p {
    width: 70px;
    margin-right: 15px;
    text-align: right;
    font-weight: bold;
  }

  select {
    -webkit-appearance: menulist-button;
    height: 25px;
  }
}
</style>