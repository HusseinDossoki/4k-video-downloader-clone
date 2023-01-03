<template>
  <section>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" value="" id="1">
      <label class="form-check-label" for="1">
        Prevent system sleep until downloads are finished
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" value="" id="2">
      <label class="form-check-label" for="2">
        Create subdirectory for downloaded playlists and channels
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" value="" id="3">
      <label class="form-check-label" for="3">
        Numerate files in playlists
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" value="" id="4">
      <label class="form-check-label" for="4">
        Skip duplicates in playlists
      </label>
    </div>
    <div class="form-check m-auto">
      <input class="form-check-input" type="checkbox" value="" id="5">
      <label class="form-check-label" for="5">
        Remove downloaded items from the list
      </label>
    </div>
  </section>
  <hr>
  <section>
    <div class="row mt-2">
        <div class="col-2">
          <label>Language</label>
        </div>
        <div class="col-10">
          <select class="w-100">
            <option value="MP4">System</option>
            <option value="MP4">English</option>
          </select>
        </div>
      </div>
  </section>
</template>

<script setup>

</script>

<style scoped>
section {
  padding: 20px;
  font-size: 13px;
}
select {
  -webkit-appearance: menulist-button;
  height: 25px;
}
</style>
