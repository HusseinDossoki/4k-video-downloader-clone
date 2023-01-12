<template>
  <div class="p-4">
    <section class="d-flex">
      <p>Notifications</p>
      <div class="toggles">
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="1">
          <label class="form-check-label" for="1">
            Show notification when downloads complete
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="2">
          <label class="form-check-label" for="2">
            Show notification about new videos from channel subscriptions
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="3">
          <label class="form-check-label" for="3">
            Play notification sound
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="4">
          <label class="form-check-label" for="4">
            Show progress on dock icon
          </label>
        </div>
      </div>
    </section>
    <hr>
    <section class="d-flex">
      <p>Confirmations</p>
      <div class="toggles">
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="5">
          <label class="form-check-label" for="5">
            Ask to confirm the app exit if there are incomplete downloads
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="6">
          <label class="form-check-label" for="6">
            Ask confirmation before item deleting
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="7">
          <label class="form-check-label" for="7">
            Ask confirmation before subscription deleting
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="8">
          <label class="form-check-label" for="8">
            Ask to select between a single video and playlist in case of ambiguity
          </label>
        </div>
        <div class="form-check m-auto">
          <input class="form-check-input" type="checkbox" id="9">
          <label class="form-check-label" for="9">
            Ask to download channel if multiple videos were downloaded from it
          </label>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>

</script>

<style scoped lang="scss">
section {
  font-size: 13px;
  color: rgb(234, 231, 231);

  p {
    width: 95px;
    margin-right: 15px;
    text-align: right;
    font-weight: bold;
  }
}
</style>
