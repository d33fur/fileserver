<template>
  <div>
    <h1>Your Files</h1>
    <input type="file" @change="onFileSelected" />
    <ul>
      <li v-for="file in files" :key="file">{{ file }}</li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'FileList',
  data() {
    return {
      files: []
    }
  },
  created() {
    this.fetchFiles();
  },
  methods: {
    async fetchFiles() {
      const response = await fetch('http://127.0.0.1:8080/files');
      this.files = await response.json();
    },
    async onFileSelected(event) {
      const file = event.target.files[0];
      const formData = new FormData();
      formData.append('file', file);

      const response = await fetch('http://127.0.0.1:8080/files/upload', {
        method: 'POST',
        body: formData
      });

      if (response.ok) {
        this.fetchFiles();
      } else {
        alert('File upload failed');
      }
    }
  }
}
</script>
