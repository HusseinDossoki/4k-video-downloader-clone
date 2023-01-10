<template>
  <div class="root">
    <header>
      <div class="d-flex">
        <img src="https://media.wired.com/photos/5b899992404e112d2df1e94e/master/pass/trash2-01.jpg" class="card-img"
          alt="image">
        <div class="px-3 w-100">
          <p class="mb-2">Card title</p>
          <div class="d-flex">
            <div class="bar-item"><i class="fa-regular fa-clock"></i> 04:33</div>
            <div class="bar-item"><i class="fa-solid fa-link"></i> <a target="_blank"
                href="https://www.youtube.com/watch?v=qgLaFF6HK88">https://www.youtube.com/watch?v=qgLaFF6HK88</a></div>
          </div>
        </div>
      </div>
    </header>
    <section>
      <div class="d-flex first-section my-2">
        <select>
          <option>Download Video</option>
        </select>
        <label>Format:</label>
        <select>
          <option>MP4</option>
        </select>
      </div>
      <hr>
      <div class="second-section my-2">
        <div class="d-flex mb-3" v-for="x in [1, 2, 3,1, 2, 31, 2, 31, 2, 31, 2, 3]">
          <input type="radio" id="html" name="fav_language" value="HTML">
          <label for="html" class="w-100 d-flex">
            <span class="label-item">High Definition</span>
            <span class="label-item">1080p</span>
            <span class="label-item">MP4 . H264 . ACC</span>
            <span class="label-item">13.3MB</span>
          </label>
        </div>
      </div>
      <hr>
      <div class="section-three">
        <span class="path" v-if="directory">{{directory}}</span>
        <button class="button browse" @click="browseDirectory">Browse</button>
      </div>
    </section>
    <footer>
      <button class="button mx-2" @click="closeWindow">Cancel</button>
      <button class="button">Download</button>
    </footer>
  </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/api/dialog';
import { ref } from "vue";

const directory = ref(null);

async function closeWindow() {
  await appWindow.close();
}

async function browseDirectory() {
  directory.value = await open({
    title: '4K Downloder Output Directory',
    directory: true,
    defaultPath: '.'
  });
}

</script>

<style scoped lang="scss">
.root {
  background-color: #2F2F2F;
  height: 100%;
  position: fixed;
  bottom: 0;
  top: 0;
  right: 0;
  left: 0;
}

header,
section,
footer {
  font-size: 13px;
  padding: 12px;
  color: white;
}

header {
  height: 70px;
  background-color: #1e1e1e;
  border-bottom: 1px solid white;

  img {
    width: 100px;
    height: 50px;
    object-fit: cover;
  }

  a {
    color: #3F94F1;
  }
}

section {
  .first-section {
    label {
      margin-left: auto;
      margin-right: 5px;
    }
  }
}

.bar-item {
  margin-right: 10px;
  font-size: 12px;
  color: rgb(148, 147, 147);

  i {
    font-size: 15px;
  }
}

.button {
  width: 90px;
  background-color: #656565;
  border: none;
  border-radius: 5px;
  color: white;
}

.path::-webkit-scrollbar {
  display: none;
}

.path {
  background-color: #292929;
  padding: 6px;
  border-radius: 2px;
}

.browse {
  float: right;
}

footer {
  position: fixed;
  bottom: 0;
  right: 0;
  left: 0;
  text-align: right;
}

.button.disabled {
  color: gray;
}

select {
  -webkit-appearance: menulist-button;
  height: 22px;
  width: 165px;
}

.second-section {
  max-height: 210px;
  overflow-y: auto;

  label {
    line-height: 10px;
    margin-left: 10px;

    .label-item:nth-child(1) {
      margin-right: 30px;
    }

    .label-item:nth-child(2) {
      font-weight: bold;
    }

    .label-item:nth-child(3) {
      margin-left: auto;
    }

    .label-item:nth-child(4) {
      margin-left: 45px;
    }
  }
}
</style>
