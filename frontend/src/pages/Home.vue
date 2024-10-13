<template>
  <v-container
    fluid
    class="bg-grey-darken-4 fill-height d-flex flex-column align-center ga-4"
  >
    <h1
      class="d-flex align-center text-medium-emphasis font-weight-bold text-h3 ga-1"
    >
      TASKS <v-icon color="red">mdi-language-rust</v-icon>
    </h1>
    <v-card variant="tonal" class="w-75 pa-4">
      <v-card-title class="pa-0 text-h5">Daily Tasks</v-card-title>

      <v-card-subtitle class="pa-0 text-subtitle-1"
        >You can create and see all your tasks here</v-card-subtitle
      >
      <v-list lines="one" class="pa-0">
        <v-sheet
          border
          color="grey-darken-3"
          v-if="tasks.length > 0"
          class="pa-0 mt-2 d-flex flex-column align-center justify-center ga-1"
        >
          <v-list-item
            v-if="tasks.length > 0"
            v-for="task in tasks"
            :key="task.id"
            class="text-subtitle-1 bg-grey-darken-4 text-grey-lighten-4 font-weight-bold w-100 pa-0 px-2"
          >
            <div class="d-flex align-center justify-space-between">
              <v-checkbox
                v-model="task.completed"
                @change="updateTask(task)"
                class="d-inline-flex"
              >
                <template v-slot:label>
                  <div class="text-subtitle-1">
                    {{ task.name }}
                  </div>
                </template>
              </v-checkbox>
              <v-btn
                icon="mdi-close-thick"
                color="red"
                variant="text"
                @click="deleteTask(task.id)"
              ></v-btn>
            </div>
          </v-list-item>
        </v-sheet>
        <v-list-item v-else class="pa-0"
          ><v-alert
            border="start"
            border-color="warning"
            variant="tonal"
            color="warning"
            class="font-weight-bold d-flex align-center mt-4"
            >You don't have any added task. You can create a new by clicking on
            <v-icon color="info">mdi-plus</v-icon>
            button.</v-alert
          ></v-list-item
        >
      </v-list>
      <v-divider class="my-3"></v-divider>
      <v-card-actions class="pa-0">
        <v-dialog v-model="addTaskDialog" max-width="500">
          <template v-slot:activator="{ props: activatorProps }">
            <v-btn
              v-bind="activatorProps"
              color="info"
              variant="flat"
              icon="mdi-plus"
            ></v-btn>
          </template>

          <template v-slot:default="{ isActive }" key="2">
            <v-card class="bg-grey-darken-4 pa-4">
              <div class="d-flex align-center justify-space-between">
                <v-card-title class="text-h5 pa-0">Task</v-card-title>
                <v-btn
                  key="2"
                  icon="mdi-close"
                  variant="text"
                  color="red"
                  @click="addTaskDialog = false"
                ></v-btn>
              </div>
              <v-card-subtitle class="text-body-1 pa-0">
                Create a new task
              </v-card-subtitle>
              <div class="d-flex flex-column my-5">
                <v-text-field
                  v-model="taskNameField"
                  prepend-inner-icon="mdi-calendar-plus"
                  variant="outlined"
                  class="w-100"
                  label="Insert your task name"
                  placeholder="ex: clean the room"
                  :rules="[(v) => !!v || 'Name is required']"
                  clearable
                ></v-text-field>
              </div>
              <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn
                  :disabled="!taskNameField"
                  :variant="!taskNameField ? 'tonal' : 'flat'"
                  :color="!taskNameField ? 'warning' : 'info'"
                  @click="saveTask"
                  text="Save task"
                ></v-btn>
              </v-card-actions>
            </v-card>
          </template>
        </v-dialog>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";

const tasks = ref([]);
const isCompleted = ref(false);
const addTaskDialog = ref(false);
const taskNameField = ref("");

const verifyCompleted = (task) => {
  if (task) {
    isCompleted.value = true;
  } else {
    isCompleted.value = false;
  }
};

const saveTask = async () => {
  axios
    .post(`http://localhost:8080/api/tasks/${taskNameField.value}`)
    .then((res) => {
      tasks.value.push(res.data);
      addTaskDialog.value = false;
      taskNameField.value = "";
    });
};

const deleteTask = async (id) => {
  tasks.value = tasks.value.filter((task) => task.id !== id);
  axios.delete(`http://localhost:8080/api/tasks/${id}`);
};

const updateTask = async (task) => {
  await axios.put(`http://localhost:8080/api/tasks/${task.id}`, {
    completed: task.completed,
  });

  if (task.completed) {
    tasks.value = tasks.value.filter((t) => t.id !== task.id);
  } else {
    const taskIndex = tasks.value.findIndex((t) => t.id === task.id);
    if (taskIndex !== -1) {
      tasks.value[taskIndex].completed = !task.completed;
    }
  }
};

onMounted(() => {
  axios.get("http://localhost:8080/api/tasks").then((res) => {
    tasks.value = res.data;
  });
});
</script>
