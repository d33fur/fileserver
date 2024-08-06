<template>
  <div>
    <h1>Home</h1>
    <FileList />
  </div>
</template>

<script>
import FileList from '../components/FileList.vue'

export default {
  name: 'Home',
  components: {
    FileList
  }
}
</script>
