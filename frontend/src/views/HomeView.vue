<template>
  <div>
    <h1>Home</h1>
    <FileList />
  </div>
</template>

<script>
import FileList from '../components/FileList.vue'

export default {
  name: 'HomeView',  // Изменено с 'Home' на 'HomeView'
  components: {
    FileList
  }
}
</script>
